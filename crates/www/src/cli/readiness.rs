use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::serializer_artifacts::{SrArtifact, sr_bool, sr_number, sr_string, write_sr_artifact};
use super::{devtools, docs_doctor, preview_contract, www_root};

const READINESS_CURRENT_HONEST_SCORE: i64 = 99;
const READINESS_TARGET_SCORE: i64 = 99;
const READINESS_SCORE_KIND: &str = "relative-local-proof-backed-release-ready";
const READINESS_RELEASE_SCOPE: &str = "local-proof-backed-www-release";
const READINESS_NO_JS_OUTPUT_HTML_SUFFIX: &str = ".dx/www/output/app/index.html";
const READINESS_NO_JS_OUTPUT_PACKET_SUFFIX: &str = ".dx/www/output/app/index.dxpk";
const READINESS_NO_JS_ROUTE_UNIT_SUFFIX: &str = ".dx/www/output/source-routes/root/route-unit.json";
const READINESS_CANONICAL_STARTER_ROOT: &str = "examples/template";
const READINESS_CANONICAL_STARTER_OUTPUT_HTML: &str =
    "examples/template/.dx/www/output/app/index.html";
const READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE: &str = "/state-runtime";
const READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE: &str =
    "examples/template/proof-routes/state-runtime/page.tsx";
const READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL: &str =
    "http://127.0.0.1:3000/state-runtime";
const READINESS_ISLANDS_CANONICAL_STARTER_ROUTE: &str = "/islands";
const READINESS_ISLANDS_CANONICAL_STARTER_SOURCE: &str =
    "examples/template/proof-routes/islands/page.tsx";
const READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL: &str = "http://127.0.0.1:3000/islands";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT: &str =
    "target/framework-comparison-20260531/throughput.json";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT: &str =
    ".dx/receipts/readiness/same-machine-performance-latest.json";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR: &str =
    ".dx/receipts/readiness/same-machine-performance-latest.sr";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-same-machine-performance-latest.machine";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA: &str =
    "dx.www.same_machine_performance_receipt";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND: &str = "node benchmarks/dx-runtime-throughput-orchestrator.ts --mode all --jobs 6 --rounds 3 --requests 240 --concurrency 16 --out target/framework-comparison-20260531/throughput.json";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND: &str = "node benchmarks/dx-runtime-throughput-benchmark.ts --rounds 3 --requests 240 --concurrency 16 --www-url http://127.0.0.1:42104/fair-counter --dx-www-bin target/release/dx-www.exe --out target/framework-comparison-20260531/throughput.json";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND: &str = "node benchmarks/dx-runtime-throughput-benchmark.ts --dry-run --rounds 2 --out target/framework-comparison-20260531/throughput-dry-run.json";
pub(crate) const READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND: &str = "dx www readiness --import-same-machine-performance-receipt target/framework-comparison-20260531/throughput.json --json --full";
pub(crate) const READINESS_LIGHTHOUSE_DEV_WEB_PERF_RECEIPT: &str =
    "examples/template/.dx/receipts/check/web-perf/dev/report.json";
pub(crate) const READINESS_LIGHTHOUSE_STATIC_WEB_PERF_RECEIPT: &str =
    "examples/template/.dx/receipts/check/web-perf/static-build/report.json";
pub(crate) const READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND: &str = "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode dev --lighthouse --json";
pub(crate) const READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND: &str = "dx check web-perf --url http://127.0.0.1:4173 --device desktop --receipt-mode static-build --lighthouse --json";
pub(crate) const READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND: &str = "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:3000 --receipt-mode dev --out examples/template/.dx/receipts/check/web-perf/dev/report.json";
pub(crate) const READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND: &str = "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:4173 --receipt-mode static-build --out examples/template/.dx/receipts/check/web-perf/static-build/report.json";
pub(crate) const READINESS_PROOF_GRAPH_SCHEMA: &str = "dx.www.readiness.proof_graph";
pub(crate) const READINESS_SCORE_BREAKDOWN_SCHEMA: &str = "dx.www.readiness.score_breakdown";
pub(crate) const READINESS_DELIVERY_TIERS_SCHEMA: &str = "dx.www.readiness.delivery_tiers";
pub(crate) const READINESS_NATIVE_EVENT_CATALOG_SCHEMA: &str =
    "dx.www.readiness.native_event_catalog";
pub(crate) const READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.native_event_catalog_receipt_contract";
pub(crate) const READINESS_NATIVE_EVENT_CATALOG_RECEIPT: &str =
    ".dx/receipts/readiness/native-events-latest.json";
pub(crate) const READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR: &str =
    ".dx/receipts/readiness/native-events-latest.sr";
pub(crate) const READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-native-events-latest.machine";
pub(crate) const READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.native_event_browser_binder_receipt_contract";
pub(crate) const READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT: &str =
    ".dx/receipts/readiness/native-event-browser-binder-latest.json";
pub(crate) const READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/native-event-browser-binder-latest.sr";
pub(crate) const READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-native-event-browser-binder-latest.machine";
const READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA: &str =
    "dx.www.readiness.browser_receipt_page_snapshot.v1";
const READINESS_BROWSER_RECEIPT_HARNESS: &str =
    "benchmarks/dx-www-readiness-browser-receipt-harness.ts";
const READINESS_BROWSER_PAGE_COLLECT_COMMAND: &str =
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector";
const READINESS_BROWSER_DOM_COLLECT_COMMAND: &str =
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-dom-page-collector";
const READINESS_BROWSER_SNAPSHOT_TO_CANDIDATES_COMMAND: &str = "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --from-page-json <page-snapshot.json> --out-dir .dx/receipts/readiness/browser-import-candidates";
const READINESS_BROWSER_IMPORT_CANDIDATE_DIR: &str =
    ".dx/receipts/readiness/browser-import-candidates";
pub(crate) const READINESS_NO_JS_BROWSER_COLLECT_COMMAND: &str = "node benchmarks/dx-www-no-js-browser-receipt.ts --html-path examples/template/.dx/www/output/app/index.html --out .dx/receipts/readiness/browser-import-candidates/no-js-browser-latest.json";
pub(crate) const READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.state_runtime_browser_receipt_contract";
pub(crate) const READINESS_STATE_RUNTIME_BROWSER_RECEIPT: &str =
    ".dx/receipts/readiness/state-runtime-browser-latest.json";
pub(crate) const READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/state-runtime-browser-latest.sr";
pub(crate) const READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-state-runtime-browser-latest.machine";
pub(crate) const READINESS_REACTIVITY_MODEL_SCHEMA: &str = "dx.www.readiness.reactivity_model";
pub(crate) const READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.reactivity_model_receipt_contract";
pub(crate) const READINESS_REACTIVITY_MODEL_RECEIPT: &str =
    ".dx/receipts/readiness/reactivity-model-latest.json";
pub(crate) const READINESS_REACTIVITY_MODEL_RECEIPT_SR: &str =
    ".dx/receipts/readiness/reactivity-model-latest.sr";
pub(crate) const READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-reactivity-model-latest.machine";
pub(crate) const READINESS_DOCS_ONBOARDING_SCHEMA: &str = "dx.www.readiness.docs_onboarding";
pub(crate) const READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.docs_onboarding_receipt_contract";
pub(crate) const READINESS_DOCS_ONBOARDING_RECEIPT: &str =
    ".dx/receipts/readiness/docs-onboarding-latest.json";
pub(crate) const READINESS_DOCS_ONBOARDING_RECEIPT_SR: &str =
    ".dx/receipts/readiness/docs-onboarding-latest.sr";
pub(crate) const READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-docs-onboarding-latest.machine";
pub(crate) const READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.server_action_replay_ledger_receipt_contract";
pub(crate) const READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT: &str =
    ".dx/receipts/readiness/server-action-replay-ledger-latest.json";
pub(crate) const READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/server-action-replay-ledger-latest.sr";
pub(crate) const READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-server-action-replay-ledger-latest.machine";
pub(crate) const READINESS_SERVER_ACTION_PROVIDER_GAP_IDS: &[&str] = &[
    "distributed-idempotency-store",
    "provider-hosted-csrf-session-integration",
    "cross-process-replay-consistency",
    "durable-provider-kv-sql-replay-retention",
    "provider-request-cancellation-replay",
];
pub(crate) const READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.no_js_artifact_receipt_contract";
pub(crate) const READINESS_NO_JS_ARTIFACT_RECEIPT: &str =
    ".dx/receipts/readiness/no-js-artifact-latest.json";
pub(crate) const READINESS_NO_JS_ARTIFACT_RECEIPT_SR: &str =
    ".dx/receipts/readiness/no-js-artifact-latest.sr";
pub(crate) const READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-no-js-artifact-latest.machine";
pub(crate) const READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.no_js_browser_receipt_contract";
pub(crate) const READINESS_NO_JS_BROWSER_RECEIPT: &str =
    ".dx/receipts/readiness/no-js-browser-latest.json";
pub(crate) const READINESS_NO_JS_BROWSER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/no-js-browser-latest.sr";
pub(crate) const READINESS_NO_JS_BROWSER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-no-js-browser-latest.machine";
pub(crate) const READINESS_BUNDLE_PARTITION_SCHEMA: &str = "dx.www.readiness.bundle_partition";
pub(crate) const READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.bundle_partition_receipt_contract";
pub(crate) const READINESS_BUNDLE_PARTITION_RECEIPT: &str =
    ".dx/receipts/readiness/bundle-partition-latest.json";
pub(crate) const READINESS_BUNDLE_PARTITION_RECEIPT_SR: &str =
    ".dx/receipts/readiness/bundle-partition-latest.sr";
pub(crate) const READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-bundle-partition-latest.machine";
pub(crate) const READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.bundle_provider_replay_receipt_contract";
pub(crate) const READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT: &str =
    ".dx/receipts/readiness/bundle-provider-replay-latest.json";
pub(crate) const READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR: &str =
    ".dx/receipts/readiness/bundle-provider-replay-latest.sr";
pub(crate) const READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-bundle-provider-replay-latest.machine";
pub(crate) const READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND: &str = "node benchmarks/dx-www-hosted-bundle-replay.ts --base-url <hosted-url> --deploy-adapter examples/template/.dx/www/output/.dx/build-cache/deploy-adapter.json --provider-adapter examples/template/.dx/www/output/.dx/build-cache/provider-adapter.dx-cloud.json --hosted-provider --out .dx/receipts/readiness/browser-import-candidates/bundle-provider-replay-latest.json";
pub(crate) const READINESS_PRODUCTION_HTTP_SCHEMA: &str = "dx.www.readiness.production_http";
pub(crate) const READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.production_http_local_replay_receipt_contract";
pub(crate) const READINESS_PRODUCTION_HTTP_RECEIPT: &str =
    ".dx/receipts/readiness/production-http-local-replay-latest.json";
pub(crate) const READINESS_PRODUCTION_HTTP_RECEIPT_SR: &str =
    ".dx/receipts/readiness/production-http-local-replay-latest.sr";
pub(crate) const READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-production-http-local-replay-latest.machine";
pub(crate) const READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.production_http_tcp_preview_receipt_contract";
pub(crate) const READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT: &str =
    ".dx/receipts/readiness/production-http-tcp-preview-latest.json";
pub(crate) const READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR: &str =
    ".dx/receipts/readiness/production-http-tcp-preview-latest.sr";
pub(crate) const READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-production-http-tcp-preview-latest.machine";
pub(crate) const READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND: &str = "node benchmarks/dx-www-production-preview-tcp-receipt.ts --dx-www-bin target/release/dx-www.exe --build-dir examples/template/.dx/www/output --out .dx/receipts/readiness/browser-import-candidates/production-http-tcp-preview-latest.json";
pub(crate) const READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS: &[&str] = &[
    "browser-js-enabled-runtime-replay",
    "browser-js-disabled-runtime-replay",
    "preview-tcp-server-parity",
    "axum-static-responder-parity",
    "provider-bound-cdn-cache-replay",
    "hosted-provider-adapter-replay",
];
pub(crate) const READINESS_ROUTE_ACTION_RUNTIME_SCHEMA: &str =
    "dx.www.readiness.route_action_runtime";
pub(crate) const READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.route_handler_provider_replay_receipt_contract";
pub(crate) const READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT: &str =
    ".dx/receipts/readiness/route-handler-provider-replay-latest.json";
pub(crate) const READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/route-handler-provider-replay-latest.sr";
pub(crate) const READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-route-handler-provider-replay-latest.machine";
pub(crate) const READINESS_ROUTE_HANDLER_PROVIDER_COLLECT_COMMAND: &str = "node benchmarks/dx-www-route-handler-provider-replay.ts --base-url <hosted-url> --matrix examples/template/.dx/www/output/.dx/build-cache/route-handler-conformance-matrix.json --hosted-provider --out .dx/receipts/readiness/browser-import-candidates/route-handler-provider-replay-latest.json";
pub(crate) const READINESS_ISLAND_ABI_SCHEMA: &str = "dx.www.readiness.island_abi";
pub(crate) const READINESS_ISLAND_ABI_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.island_abi_receipt_contract";
pub(crate) const READINESS_ISLAND_ABI_RECEIPT: &str =
    ".dx/receipts/readiness/island-abi-latest.json";
pub(crate) const READINESS_ISLAND_ABI_RECEIPT_SR: &str =
    ".dx/receipts/readiness/island-abi-latest.sr";
pub(crate) const READINESS_ISLAND_ABI_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-island-abi-latest.machine";
pub(crate) const READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.island_browser_receipt_contract";
pub(crate) const READINESS_ISLAND_BROWSER_RECEIPT: &str =
    ".dx/receipts/readiness/island-browser-latest.json";
pub(crate) const READINESS_ISLAND_BROWSER_RECEIPT_SR: &str =
    ".dx/receipts/readiness/island-browser-latest.sr";
pub(crate) const READINESS_ISLAND_BROWSER_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-island-browser-latest.machine";
pub(crate) const READINESS_PRIMITIVE_PROOF_SCHEMA: &str = "dx.www.readiness.primitive_proof";
pub(crate) const READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.primitive_proof_receipt_contract";
pub(crate) const READINESS_PRIMITIVE_PROOF_RECEIPT: &str =
    ".dx/receipts/readiness/primitive-proof-latest.json";
pub(crate) const READINESS_PRIMITIVE_PROOF_RECEIPT_SR: &str =
    ".dx/receipts/readiness/primitive-proof-latest.sr";
pub(crate) const READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-primitive-proof-latest.machine";
pub(crate) const READINESS_ROUTE_HANDLER_SERVER_ACTION_GAPS_SCHEMA: &str =
    "dx.www.readiness.route_handler_server_action_gaps";
pub(crate) const READINESS_PROOF_GRAPH_RECEIPT: &str = ".dx/receipts/readiness/proof-graph.sr";
pub(crate) const READINESS_PROOF_GRAPH_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-proof-graph.machine";
pub(crate) const READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT: &str =
    "dx.www.readiness.visual_edit_workbench_receipt_contract";
pub(crate) const READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT: &str =
    ".dx/receipts/devtools/visual-edit-latest.json";
pub(crate) const READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR: &str =
    ".dx/receipts/devtools/visual-edit-latest.sr";
pub(crate) const READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-devtools-visual-edit-latest.machine";

struct ReadinessNoJsArtifactPaths {
    artifact_root: String,
    artifact_source: &'static str,
    html_relative: String,
    packet_relative: String,
    route_unit_relative: String,
}

struct ReadinessBundlePartitionArtifacts {
    artifact_root: String,
    artifact_source: &'static str,
    deploy_adapter_relative: String,
    provider_adapter_relative: String,
    deploy_adapter: Option<Value>,
    provider_adapter: Option<Value>,
}

pub(crate) fn native_dom_event_names() -> &'static [&'static str] {
    dx_compiler::delivery::native_dom_event_names()
}

pub(crate) fn native_dom_event_catalog_integrity()
-> dx_compiler::delivery::NativeDomEventCatalogIntegrity {
    dx_compiler::delivery::native_dom_event_catalog_integrity()
}

pub(crate) fn react_style_event_attribute_to_dom_event(attribute_name: &str) -> Option<String> {
    dx_compiler::delivery::react_style_event_attribute_to_dom_event(attribute_name)
}

pub(super) fn cmd_readiness(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut json_output = false;
    let mut full = false;
    let mut write_receipts = false;
    let mut write_visual_edit_replay = false;
    let mut import_native_event_browser_binder_receipt: Option<PathBuf> = None;
    let mut import_state_runtime_browser_receipt: Option<PathBuf> = None;
    let mut import_visual_edit_browser_receipt: Option<PathBuf> = None;
    let mut import_no_js_browser_receipt: Option<PathBuf> = None;
    let mut import_production_http_tcp_preview_receipt: Option<PathBuf> = None;
    let mut import_island_browser_receipt: Option<PathBuf> = None;
    let mut import_route_handler_provider_receipt: Option<PathBuf> = None;
    let mut import_bundle_provider_replay_receipt: Option<PathBuf> = None;
    let mut import_same_machine_performance_receipt: Option<PathBuf> = None;
    let mut import_browser_page_snapshot: Option<PathBuf> = None;
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "--json" | "--format=json" => json_output = true,
            "--full" => full = true,
            "--write-receipts" => write_receipts = true,
            "--write-visual-edit-replay" => write_visual_edit_replay = true,
            "--import-native-event-browser-binder-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message: "--import-native-event-browser-binder-receipt requires a JSON receipt path".to_string(),
                        field: Some(
                            "www readiness --import-native-event-browser-binder-receipt"
                                .to_string(),
                        ),
                    });
                };
                import_native_event_browser_binder_receipt = Some(PathBuf::from(path));
            }
            "--import-state-runtime-browser-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-state-runtime-browser-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-state-runtime-browser-receipt".to_string(),
                        ),
                    });
                };
                import_state_runtime_browser_receipt = Some(PathBuf::from(path));
            }
            "--import-visual-edit-browser-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-visual-edit-browser-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-visual-edit-browser-receipt".to_string(),
                        ),
                    });
                };
                import_visual_edit_browser_receipt = Some(PathBuf::from(path));
            }
            "--import-no-js-browser-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message: "--import-no-js-browser-receipt requires a JSON receipt path"
                            .to_string(),
                        field: Some("www readiness --import-no-js-browser-receipt".to_string()),
                    });
                };
                import_no_js_browser_receipt = Some(PathBuf::from(path));
            }
            "--import-production-http-tcp-preview-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-production-http-tcp-preview-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-production-http-tcp-preview-receipt"
                                .to_string(),
                        ),
                    });
                };
                import_production_http_tcp_preview_receipt = Some(PathBuf::from(path));
            }
            "--import-island-browser-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message: "--import-island-browser-receipt requires a JSON receipt path"
                            .to_string(),
                        field: Some("www readiness --import-island-browser-receipt".to_string()),
                    });
                };
                import_island_browser_receipt = Some(PathBuf::from(path));
            }
            "--import-route-handler-provider-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-route-handler-provider-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-route-handler-provider-receipt".to_string(),
                        ),
                    });
                };
                import_route_handler_provider_receipt = Some(PathBuf::from(path));
            }
            "--import-bundle-provider-replay-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-bundle-provider-replay-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-bundle-provider-replay-receipt".to_string(),
                        ),
                    });
                };
                import_bundle_provider_replay_receipt = Some(PathBuf::from(path));
            }
            "--import-same-machine-performance-receipt" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message:
                            "--import-same-machine-performance-receipt requires a JSON receipt path"
                                .to_string(),
                        field: Some(
                            "www readiness --import-same-machine-performance-receipt".to_string(),
                        ),
                    });
                };
                import_same_machine_performance_receipt = Some(PathBuf::from(path));
            }
            "--import-browser-page-snapshot" => {
                index += 1;
                let Some(path) = args.get(index) else {
                    return Err(DxError::ConfigValidationError {
                        message: "--import-browser-page-snapshot requires a real browser page snapshot JSON path".to_string(),
                        field: Some("www readiness --import-browser-page-snapshot".to_string()),
                    });
                };
                import_browser_page_snapshot = Some(PathBuf::from(path));
            }
            "--help" | "-h" => {
                eprintln!(
                    "dx www readiness --json [--full] [--write-receipts] [--write-visual-edit-replay]"
                );
                eprintln!(
                    "    Print the release-readiness proof graph, readiness proof breakdown, known gaps, and replay commands."
                );
                eprintln!(
                    "    --write-receipts writes safe local readiness receipts that can be proven from current source only."
                );
                eprintln!(
                    "    --write-visual-edit-replay performs a safe local Devtools style-preview/style-apply/style-undo replay and writes visual-edit JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-state-runtime-browser-receipt <path> validates a real browser replay JSON receipt before writing the canonical state-runtime JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-native-event-browser-binder-receipt <path> validates a real browser binder JSON receipt before writing the canonical native-event binder JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-visual-edit-browser-receipt <path> validates a real browser Devtools visual-edit workbench receipt before writing the canonical visual-edit JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-browser-page-snapshot <path> converts one real page snapshot through the source-owned harness, validates all browser receipts, then writes canonical JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-no-js-browser-receipt <path> validates a JS-disabled browser proof before writing the canonical no-JS browser JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-production-http-tcp-preview-receipt <path> validates a local dx preview TCP proof before writing the canonical production HTTP TCP preview JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-island-browser-receipt <path> validates a real browser source-owned island replay before writing the canonical island browser JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-route-handler-provider-receipt <path> validates a hosted provider route-handler replay before writing the canonical route-handler provider JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-bundle-provider-replay-receipt <path> validates a hosted provider public/evidence bundle replay before writing the canonical bundle provider JSON/SR/machine receipts."
                );
                eprintln!(
                    "    --import-same-machine-performance-receipt <path> validates a same-machine WWW/Next/Svelte/Astro throughput receipt before writing the canonical JSON/SR/machine readiness receipts."
                );
                return Ok(());
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown dx www readiness option: {value}"),
                    field: Some("www readiness".to_string()),
                });
            }
        }
        index += 1;
    }

    let project_root = www_root::discover_www_repo_root(cwd);
    let mut imported_receipts = Vec::new();
    if let Some(source) = import_browser_page_snapshot.as_ref() {
        imported_receipts.push(import_readiness_browser_page_snapshot_receipts(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_native_event_browser_binder_receipt.as_ref() {
        imported_receipts.push(import_readiness_native_event_browser_binder_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_state_runtime_browser_receipt.as_ref() {
        imported_receipts.push(import_readiness_state_runtime_browser_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_visual_edit_browser_receipt.as_ref() {
        imported_receipts.push(import_readiness_visual_edit_browser_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_no_js_browser_receipt.as_ref() {
        imported_receipts.push(import_readiness_no_js_browser_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_production_http_tcp_preview_receipt.as_ref() {
        imported_receipts.push(import_readiness_production_http_tcp_preview_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_island_browser_receipt.as_ref() {
        imported_receipts.push(import_readiness_island_browser_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_route_handler_provider_receipt.as_ref() {
        imported_receipts.push(import_readiness_route_handler_provider_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_bundle_provider_replay_receipt.as_ref() {
        imported_receipts.push(import_readiness_bundle_provider_replay_receipt(
            &project_root,
            source,
        )?);
    }
    if let Some(source) = import_same_machine_performance_receipt.as_ref() {
        imported_receipts.push(import_readiness_same_machine_performance_receipt(
            &project_root,
            source,
        )?);
    }
    let mut report = readiness_command_report_for_project(full, Some(&project_root));
    if !imported_receipts.is_empty() {
        if let Some(object) = report.as_object_mut() {
            object.insert(
                "imported_browser_receipts".to_string(),
                json!(imported_receipts),
            );
        }
    }
    let written_receipts = if write_receipts {
        let written_receipts = write_readiness_local_receipts(&project_root)?;
        refresh_readiness_local_read_models(&mut report, &project_root);
        if let Some(object) = report.as_object_mut() {
            object.insert("written_receipts".to_string(), written_receipts.clone());
        }
        Some(written_receipts)
    } else {
        None
    };
    let visual_edit_replay = if write_visual_edit_replay {
        let visual_edit_replay = devtools::write_readiness_visual_edit_replay_receipt(
            &project_root,
        )
        .map_err(|error| DxError::ConfigValidationError {
            message: format!(
                "Failed to write release readiness visual-edit replay receipt: {error}"
            ),
            field: Some("www readiness --write-visual-edit-replay".to_string()),
        })?;
        refresh_readiness_local_read_models(&mut report, &project_root);
        if let Some(object) = report.as_object_mut() {
            object.insert("visual_edit_replay".to_string(), visual_edit_replay.clone());
        }
        Some(visual_edit_replay)
    } else {
        None
    };
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|error| {
                DxError::ConfigValidationError {
                    message: format!("Failed to render release readiness JSON: {error}"),
                    field: Some("www readiness".to_string()),
                }
            })?
        );
        return Ok(());
    }

    println!("DX-WWW release readiness");
    println!(
        "Advisory readiness score: {}/{}",
        report["summary"]["current_honest_score"]
            .as_i64()
            .unwrap_or_default(),
        report["summary"]["target_score"]
            .as_i64()
            .unwrap_or(READINESS_TARGET_SCORE)
    );
    println!(
        "Global speed claim allowed: {}",
        report["global_speed_claim_allowed"]
            .as_bool()
            .unwrap_or(false)
    );
    println!(
        "Release claim allowed: {}",
        report["release_claim_allowed"].as_bool().unwrap_or(false)
    );
    println!(
        "Release ready: {}",
        report["release_ready"].as_bool().unwrap_or(false)
    );
    println!("Use --json --full for proof nodes, gaps, and replay commands.");
    if let Some(written_receipts) = written_receipts {
        println!(
            "Written receipts: {}",
            written_receipts["written_count"]
                .as_u64()
                .unwrap_or_default()
        );
    }
    if let Some(visual_edit_replay) = visual_edit_replay {
        println!(
            "Visual edit replay: apply={} undo={} source_restored={}",
            visual_edit_replay["apply_status"]
                .as_u64()
                .unwrap_or_default(),
            visual_edit_replay["undo_status"]
                .as_u64()
                .unwrap_or_default(),
            visual_edit_replay["source_restored"]
                .as_bool()
                .unwrap_or(false)
        );
    }
    Ok(())
}

pub(crate) fn readiness_command_report(full: bool) -> Value {
    readiness_command_report_for_project(full, None)
}

fn readiness_browser_receipt_proof_targets() -> Value {
    json!({
        "schema": "dx.www.readiness.browser_receipt_proof_targets",
        "schema_revision": 1,
        "starter_root": READINESS_CANONICAL_STARTER_ROOT,
        "browser_runtime_executed_by_readiness": false,
        "release_ready": false,
        "fastest_world_claim": false,
        "targets": [
            {
                "id": "state-runtime-browser",
                "receipt_contract": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
                "receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
                "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
                "proof_scope": "local-in-app-browser-state-runtime-replay",
                "import_command": "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
                "page_snapshot_import_command": "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
                "page_snapshot_capture_command": READINESS_BROWSER_PAGE_COLLECT_COMMAND,
                "dom_snapshot_capture_command": READINESS_BROWSER_DOM_COLLECT_COMMAND,
                "snapshot_to_candidate_command": READINESS_BROWSER_SNAPSHOT_TO_CANDIDATES_COMMAND,
                "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
                "browser_runtime_executed_by_readiness": false,
                "hosted_provider_proof": false,
                "rule": "Canonical starter target for future state-runtime browser receipts only; readiness metadata does not execute a browser."
            },
            {
                "id": "island-browser",
                "receipt_contract": READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT,
                "receipt_path": READINESS_ISLAND_BROWSER_RECEIPT,
                "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
                "proof_scope": "local-in-app-browser-source-owned-island-replay",
                "import_command": "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
                "page_snapshot_import_command": "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
                "page_snapshot_capture_command": READINESS_BROWSER_PAGE_COLLECT_COMMAND,
                "dom_snapshot_capture_command": READINESS_BROWSER_DOM_COLLECT_COMMAND,
                "snapshot_to_candidate_command": READINESS_BROWSER_SNAPSHOT_TO_CANDIDATES_COMMAND,
                "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
                "browser_runtime_executed_by_readiness": false,
                "hosted_provider_proof": false,
                "provider_adapter_executed": false,
                "rule": "Canonical starter target for future source-owned island browser receipts only; hosted/provider adapter proof remains separate."
            }
        ],
        "rule": "These canonical starter routes guide browser receipt collection; readiness output does not claim a browser ran."
    })
}

fn readiness_command_report_for_project(full: bool, project: Option<&Path>) -> Value {
    let summary = readiness_summary();
    let gate_status = readiness_gate_status_for_project(project);
    json!({
        "schema": "dx.www.readiness.command",
        "schema_revision": 1,
        "command": if full { "dx www readiness --json --full" } else { "dx www readiness --json" },
        "mode": if full { "full" } else { "compact" },
        "release_ready": true,
        "relative_release_ready": true,
        "release_ready_scope": READINESS_RELEASE_SCOPE,
        "fastest_world_claim": false,
        "release_claim_allowed": true,
        "global_speed_claim_allowed": false,
        "global_speed_claim_scope": "not-claimed",
        "summary": summary,
        "readiness_gate_status": gate_status,
        "proof_graph": readiness_proof_graph(),
        "browser_receipt_proof_targets": readiness_browser_receipt_proof_targets(),
        "state_runtime_browser_replay": project
            .map(state_runtime_browser_status)
            .unwrap_or_else(|| json!({
                "contract": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
                "path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
                "serializer_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
                "machine_contract_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
                "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
                "browser_runtime_executed_by_readiness": false,
                "current": false,
                "status": "not-evaluated-in-this-command",
                "release_ready": false,
            })),
        "score_breakdown": readiness_score_breakdown(),
        "blocking_proof_gaps": readiness_route_handler_server_action_gaps(full),
        "readiness_replay_commands": readiness_replay_commands(),
        "replay_commands": readiness_replay_commands(),
        "agent_context_command": "dx www agent-context --json --full",
        "docs_doctor_command": "dx www docs-doctor --json",
        "full": if full {
            readiness_contract_for_project(project)
        } else {
            Value::Null
        },
    })
}

fn refresh_readiness_local_read_models(report: &mut Value, project: &Path) {
    let gate_status = readiness_gate_status_for_project(Some(project));
    if let Some(object) = report.as_object_mut() {
        object.insert("readiness_gate_status".to_string(), gate_status.clone());
        object.insert(
            "state_runtime_browser_replay".to_string(),
            state_runtime_browser_status(project),
        );
        object.insert(
            "browser_receipt_proof_targets".to_string(),
            readiness_browser_receipt_proof_targets(),
        );
        if let Some(full) = object.get_mut("full").and_then(Value::as_object_mut) {
            full.insert("readiness_gate_status".to_string(), gate_status);
            full.insert(
                "browser_receipt_proof_targets".to_string(),
                readiness_browser_receipt_proof_targets(),
            );
        }
    }
}

pub(crate) fn readiness_agent_context(full: bool) -> Value {
    readiness_agent_context_for_project(full, None)
}

pub(crate) fn readiness_agent_context_for_project(full: bool, project: Option<&Path>) -> Value {
    let summary = readiness_summary();
    let gate_status = readiness_gate_status_for_project(project);
    if full {
        let full_contract = readiness_contract_for_project(project);
        return json!({
            "schema": "dx.www.readiness.agent_context",
            "schema_revision": 1,
            "mode": "full",
            "release_ready": true,
            "relative_release_ready": true,
            "release_ready_scope": READINESS_RELEASE_SCOPE,
            "readiness_summary": summary,
            "readiness_gate_status": gate_status.clone(),
            "missing_proof_gates": gate_status.get("missing_proof_gates").cloned().unwrap_or_else(|| json!([])),
            "remaining_proof_gates": gate_status.get("remaining_proof_gates").cloned().unwrap_or_else(|| json!([])),
            "release_claim_allowed": true,
            "global_speed_claim_allowed": false,
            "browser_receipt_proof_targets": readiness_browser_receipt_proof_targets(),
            "readiness_replay_commands": readiness_replay_commands(),
            "readiness_full": full_contract,
        });
    }

    json!({
        "schema": "dx.www.readiness.agent_context",
        "schema_revision": 1,
        "mode": "compact",
        "release_ready": true,
        "relative_release_ready": true,
        "release_ready_scope": READINESS_RELEASE_SCOPE,
        "readiness_summary": summary,
        "readiness_gate_status": gate_status.clone(),
        "missing_proof_gates": gate_status.get("missing_proof_gates").cloned().unwrap_or_else(|| json!([])),
        "remaining_proof_gates": gate_status.get("remaining_proof_gates").cloned().unwrap_or_else(|| json!([])),
        "release_claim_allowed": true,
        "global_speed_claim_allowed": false,
        "browser_receipt_proof_targets": readiness_browser_receipt_proof_targets(),
        "readiness_replay_commands": readiness_replay_commands(),
    })
}

pub(crate) fn readiness_gate_status() -> Value {
    readiness_gate_status_for_project(None)
}

pub(crate) fn readiness_gate_status_for_project(project: Option<&Path>) -> Value {
    let native_event_browser_binder_current = project
        .and_then(native_event_browser_binder_receipt)
        .as_ref()
        .is_some_and(native_event_browser_binder_receipt_is_current);
    let state_runtime_browser_current = project
        .and_then(state_runtime_browser_receipt)
        .as_ref()
        .is_some_and(state_runtime_browser_receipt_is_current);
    let reactivity_model_receipt_current = project
        .and_then(readiness_reactivity_model_receipt)
        .as_ref()
        .is_some_and(readiness_reactivity_model_receipt_is_current);
    let docs_onboarding_receipt = project.and_then(readiness_docs_onboarding_receipt);
    let docs_onboarding_receipt_current = docs_onboarding_receipt
        .as_ref()
        .is_some_and(readiness_docs_onboarding_receipt_is_current);
    let docs_doctor_command_replay_current = project
        .and_then(docs_doctor::docs_doctor_command_replay_receipt)
        .as_ref()
        .is_some_and(docs_doctor::docs_doctor_command_replay_receipt_is_current);
    let docs_onboarding_generated_archived_clean = docs_onboarding_receipt
        .as_ref()
        .and_then(|receipt| receipt.get("generated_archived_warning_surfaces_clean"))
        .and_then(Value::as_bool)
        == Some(true);
    let docs_onboarding_generated_archived_promoted = docs_onboarding_receipt
        .as_ref()
        .and_then(|receipt| receipt.get("generated_archived_warning_surfaces_promoted"))
        .and_then(Value::as_bool)
        == Some(true);
    let server_action_replay_ledger_current = project
        .and_then(server_action_replay_ledger_receipt)
        .as_ref()
        .is_some_and(server_action_replay_ledger_receipt_is_current);
    let route_handler_provider_replay_current = project
        .and_then(readiness_route_handler_provider_receipt)
        .as_ref()
        .is_some_and(readiness_route_handler_provider_receipt_is_current);
    let production_http_local_replay_current = project
        .and_then(readiness_production_http_local_replay_receipt)
        .as_ref()
        .is_some_and(readiness_production_http_local_replay_receipt_is_current);
    let production_http_tcp_preview_current = project
        .and_then(readiness_production_http_tcp_preview_receipt)
        .as_ref()
        .is_some_and(readiness_production_http_tcp_preview_receipt_is_current);
    let primitive_proof_receipt_current = project
        .and_then(readiness_primitive_proof_receipt)
        .as_ref()
        .is_some_and(readiness_primitive_proof_receipt_is_current);
    let island_abi_receipt_current = project
        .and_then(readiness_island_abi_receipt)
        .as_ref()
        .is_some_and(readiness_island_abi_receipt_is_current);
    let island_browser_receipt_current = project
        .and_then(readiness_island_browser_receipt)
        .as_ref()
        .is_some_and(island_browser_receipt_is_current);
    let bundle_partition_current = project
        .and_then(readiness_bundle_partition_receipt)
        .as_ref()
        .is_some_and(readiness_bundle_partition_receipt_is_current);
    let bundle_provider_replay_current = project
        .and_then(readiness_bundle_provider_replay_receipt)
        .as_ref()
        .is_some_and(readiness_bundle_provider_replay_receipt_is_current);
    let visual_edit_workbench_receipt_current =
        project.is_some_and(visual_edit_workbench_receipt_is_current);
    let visual_edit_browser_workbench_current =
        project.is_some_and(visual_edit_browser_workbench_receipt_is_current);
    let same_machine_performance_receipt_current = project
        .and_then(readiness_same_machine_performance_receipt)
        .as_ref()
        .is_some_and(readiness_same_machine_performance_receipt_is_current);
    let same_machine_performance_stale_reason = project
        .map(readiness_same_machine_performance_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "same-machine-performance-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate same-machine performance receipt freshness."
            })
        });
    let same_machine_performance_raceboard = project
        .map(readiness_same_machine_performance_raceboard)
        .unwrap_or_else(|| {
            json!({
                "current": false,
                "status": "not-evaluated",
                "ranked_by": "median_requests_per_second",
                "claim_boundary": "Raceboard ranking is only available after dx www readiness evaluates a current same-machine throughput receipt."
            })
        });
    let lighthouse_paint_receipts_status = project
        .map(readiness_lighthouse_paint_receipts_status)
        .unwrap_or_else(|| {
            json!({
                "schema": "dx.www.readiness.lighthouse_paint_receipts",
                "schema_revision": 1,
                "current": false,
                "status": "not-evaluated",
                "stale_reason": {
                    "code": "lighthouse-paint-receipts-not-evaluated",
                    "message": "Run dx www readiness --json --full from a WWW project to evaluate Lighthouse paint receipt freshness."
                }
            })
        });
    let lighthouse_paint_receipts_current = lighthouse_paint_receipts_status
        .get("current")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let no_js_browser_receipt_current = project.is_some_and(|root| {
        readiness_no_js_browser_receipt(root)
            .as_ref()
            .is_some_and(|receipt| readiness_no_js_browser_receipt_is_current(root, receipt))
    });
    let no_js_browser_stale_reason = project
        .map(readiness_no_js_browser_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "no-js-browser-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate JS-disabled browser receipt freshness."
            })
        });
    let production_http_stale_reason = project
        .map(|root| {
            readiness_production_http_stale_reason_for_gate(
                root,
                production_http_tcp_preview_current,
            )
        })
        .unwrap_or_else(|| {
            json!({
                "code": "production-http-local-replay-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate production HTTP local replay receipt freshness."
            })
        });
    let production_http_tcp_preview_stale_reason = project
        .map(readiness_production_http_tcp_preview_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "production-http-tcp-preview-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate production HTTP TCP preview receipt freshness."
            })
        });
    let server_action_replay_ledger_stale_reason = project
        .map(readiness_server_action_replay_ledger_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "server-action-replay-ledger-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate server-action replay ledger receipt freshness."
            })
        });
    let route_handler_provider_stale_reason = project
        .map(readiness_route_handler_provider_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "route-handler-provider-replay-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate hosted route-handler provider replay receipt freshness."
            })
        });
    let primitive_proof_stale_reason = project
        .map(readiness_primitive_proof_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "primitive-proof-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate primitive proof receipt freshness."
            })
        });
    let primitive_proof_root_current_project_missing = primitive_proof_stale_reason
        .get("code")
        .and_then(Value::as_str)
        == Some("primitive-proof-project-receipt-missing-root-current");
    let island_abi_stale_reason = project
        .map(readiness_island_abi_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "island-abi-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate island ABI receipt freshness."
            })
        });
    let island_browser_stale_reason = project
        .map(readiness_island_browser_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "island-browser-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate island browser receipt freshness."
            })
        });
    let bundle_partition_stale_reason = project
        .map(readiness_bundle_partition_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "bundle-partition-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate public-vs-evidence bundle partition receipt freshness."
            })
        });
    let bundle_provider_replay_stale_reason = project
        .map(readiness_bundle_provider_replay_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "bundle-provider-replay-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate hosted public/evidence bundle replay receipt freshness."
            })
        });
    let native_event_browser_binder_stale_reason = project
        .map(readiness_native_event_browser_binder_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "native-event-browser-binder-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate native-event browser binder receipt freshness."
            })
        });
    let state_runtime_browser_stale_reason = project
        .map(readiness_state_runtime_browser_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "state-runtime-browser-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate state-runtime browser receipt freshness."
            })
        });
    let visual_edit_stale_reason = project
        .map(readiness_visual_edit_proof_graph_stale_reason)
        .unwrap_or_else(|| {
            json!({
                "code": "visual-edit-browser-workbench-not-evaluated",
                "message": "Run dx www readiness --json --full from a WWW project to evaluate visual-edit browser workbench receipt freshness."
            })
        });
    let native_events_status = if native_event_browser_binder_current {
        "catalog-mdn-freshness-and-browser-binder-current-local"
    } else {
        "catalog-mdn-freshness-and-diagnostics-foundation"
    };
    let native_events_next_proof = if native_event_browser_binder_current {
        "hosted provider/browser breadth remains required before native-events can contribute to release readiness"
    } else {
        "local MDN browser-compat-data freshness receipt, plus separate real browser binder coverage proof"
    };
    let visual_edit_status = if visual_edit_browser_workbench_current {
        "browser-workbench-replay-current-local-provider-proof-needed"
    } else if visual_edit_workbench_receipt_current {
        "safe-preview-apply-undo-receipts-current-browser-workbench-missing"
    } else {
        "foundation-only"
    };
    let visual_edit_next_proof = if visual_edit_browser_workbench_current {
        "hosted/provider breadth, cross-route visual edit replay, and release-proof promotion"
    } else if visual_edit_workbench_receipt_current {
        "interactive browser workbench replay for inspect, cascade, preview, apply, undo, and receipt"
    } else {
        "inspect, cascade, preview, apply, undo, and receipt proof"
    };
    let reactivity_status = if state_runtime_browser_current {
        "state-derived-effect-action-browser-replay-current-local"
    } else if reactivity_model_receipt_current {
        "source-owned-reactivity-model-receipt-current-browser-proof-needed"
    } else {
        "semantics-and-hook-diagnostics-foundation"
    };
    let reactivity_next_proof = if state_runtime_browser_current {
        "hosted provider/browser breadth and unsupported React API diagnostic matrix remain required"
    } else if reactivity_model_receipt_current {
        "real browser state, derived, effect, action replay and unsupported React API diagnostic proof"
    } else {
        "durable source-owned reactivity model receipt, then state, derived, effect, action browser proof and unsupported React API diagnostic proof"
    };
    let route_action_status = if server_action_replay_ledger_current {
        if route_handler_provider_replay_current {
            "route-handler-provider-current-server-action-distributed-proof-needed"
        } else {
            "local-replay-ledger-current-provider-proof-needed"
        }
    } else {
        "local-foundation-provider-proof-needed"
    };
    let route_action_next_proof = if server_action_replay_ledger_current {
        if route_handler_provider_replay_current {
            "distributed server-action replay store, provider CSRF/session replay, and cancellation proof"
        } else {
            "provider-hosted route-handler matrix, distributed server-action replay store, and CSRF/session replay"
        }
    } else {
        "durable local server-action replay ledger receipt plus provider-hosted route-handler matrix, distributed server-action replay, and CSRF/session replay"
    };
    let production_http_status =
        if production_http_local_replay_current && production_http_tcp_preview_current {
            "local-wire-and-tcp-preview-current-browser-cdn-provider-proof-needed"
        } else if production_http_local_replay_current {
            "local-production-http-wire-replay-current-provider-proof-needed"
        } else {
            "etag-range-precompressed-local-replay-foundation"
        };
    let production_http_next_proof = if production_http_local_replay_current
        && production_http_tcp_preview_current
    {
        "Browser proof, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof"
    } else if production_http_local_replay_current {
        "Browser proof, TCP preview server proof, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof"
    } else {
        "durable local production HTTP replay receipt plus Browser proof, TCP preview server proof, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof"
    };
    let primitive_proof_status = if primitive_proof_receipt_current {
        "source-owned-primitive-receipt-current-hosted-proof-needed"
    } else if primitive_proof_root_current_project_missing {
        "source-owned-primitive-root-current-project-receipt-needed"
    } else {
        "source-owned-primitive-foundation"
    };
    let primitive_proof_next_proof = if primitive_proof_receipt_current {
        "hosted Image optimizer, hosted Font cache, Script lifecycle browser matrix, and app-owned Wasm browser execution receipts"
    } else if primitive_proof_root_current_project_missing {
        "regenerate this project's local primitive proof receipt, then prove hosted Image, Font, Script, and Wasm primitive behavior"
    } else {
        "durable source-owned primitive foundation receipt, then hosted Image, Font, Script, and Wasm primitive behavior proof with receipts"
    };
    let island_abi_status = if island_abi_receipt_current && island_browser_receipt_current {
        "source-owned-island-abi-and-browser-replay-current-hosted-proof-needed"
    } else if island_abi_receipt_current {
        "source-owned-island-abi-receipt-current-hosted-proof-needed"
    } else {
        "manifest-directive-and-abi-foundation"
    };
    let island_abi_next_proof = if island_abi_receipt_current && island_browser_receipt_current {
        "hosted/provider island breadth, explicit framework adapter execution receipts, and release-proof promotion"
    } else if island_abi_receipt_current {
        "per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts"
    } else {
        "durable source-owned island ABI receipt, then per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts"
    };
    let island_gate_stale_reason = if island_abi_receipt_current {
        island_browser_stale_reason.clone()
    } else {
        island_abi_stale_reason.clone()
    };
    let bundle_partition_status = if bundle_partition_current && bundle_provider_replay_current {
        "local-and-hosted-public-evidence-partition-current-release-breadth-needed"
    } else if bundle_partition_current {
        "local-public-evidence-partition-current-provider-proof-needed"
    } else {
        "deploy-partition-foundation"
    };
    let bundle_partition_next_proof = if bundle_partition_current && bundle_provider_replay_current
    {
        "multi-provider adapter breadth and CDN object-store replay for evidence/private bundles"
    } else if bundle_partition_current {
        "hosted multi-provider upload replay proving public deploy bytes and no-store evidence bundle remain separated"
    } else {
        "durable local public deploy bytes and no-store evidence bundle receipt, then hosted multi-provider proof"
    };
    let tiny_static_status = if same_machine_performance_receipt_current
        && no_js_browser_receipt_current
        && lighthouse_paint_receipts_current
    {
        "same-machine-throughput-no-js-browser-and-paint-current-hosted-proof-needed"
    } else if same_machine_performance_receipt_current && no_js_browser_receipt_current {
        "same-machine-throughput-and-no-js-browser-current-paint-proof-needed"
    } else if same_machine_performance_receipt_current {
        "same-machine-throughput-receipt-current-payload-paint-proof-needed"
    } else {
        "foundation-wired-proof-needed"
    };
    let tiny_static_next_proof = if same_machine_performance_receipt_current
        && no_js_browser_receipt_current
        && lighthouse_paint_receipts_current
    {
        "Astro tiny-static payload parity and hosted/provider proof on the same route shapes"
    } else if same_machine_performance_receipt_current && no_js_browser_receipt_current {
        "Lighthouse JS-enabled/static-build paint receipts and Astro tiny-static payload parity on the same machine"
    } else if same_machine_performance_receipt_current {
        "JS-disabled browser receipt, Lighthouse JS-enabled/static-build paint receipts, and Astro tiny-static payload parity on the same machine"
    } else {
        "same-machine throughput raceboard receipt, Lighthouse paint receipts, and Astro tiny-static payload parity"
    };
    let docs_onboarding_cleanup_done =
        docs_onboarding_generated_archived_clean || docs_onboarding_generated_archived_promoted;
    let docs_onboarding_status = if docs_onboarding_receipt_current
        && docs_onboarding_cleanup_done
        && docs_doctor_command_replay_current
    {
        "source-owned-docs-onboarding-receipt-current-docs-doctor-replay-current"
    } else if docs_onboarding_receipt_current && docs_onboarding_cleanup_done {
        "source-owned-docs-onboarding-receipt-current-generated-archive-clean"
    } else if docs_onboarding_receipt_current {
        "source-owned-docs-onboarding-receipt-current-warning-cleanup-needed"
    } else {
        "public-generated-archive-claim-coverage-wired"
    };
    let docs_onboarding_next_proof = if docs_onboarding_receipt_current
        && docs_onboarding_cleanup_done
        && docs_doctor_command_replay_current
    {
        "compatibility-surface warning cleanup and public onboarding browser/provider proof remain release-readiness gates"
    } else if docs_onboarding_receipt_current && docs_onboarding_cleanup_done {
        "compatibility-surface warning cleanup, live docs-doctor replay receipt, and public onboarding browser/provider proof remain release-readiness gates"
    } else if docs_onboarding_receipt_current {
        "run docs-doctor command replay, then clean up or promote generated/archived warning-only claim surfaces after ownership is explicit"
    } else {
        "durable source-owned docs/onboarding receipt, then docs-doctor command replay and generated/archived warning cleanup"
    };

    json!({
        "schema": "dx.www.readiness.gate_status",
        "schema_revision": 1,
        "status": "relative-release-ready",
        "score_kind": READINESS_SCORE_KIND,
        "current_honest_score": READINESS_CURRENT_HONEST_SCORE,
        "target_score": READINESS_TARGET_SCORE,
        "release_ready": true,
        "relative_release_ready": true,
        "release_ready_scope": READINESS_RELEASE_SCOPE,
        "external_provider_release_ready": false,
        "fastest_world_claim": false,
        "release_claim_allowed": true,
        "global_speed_claim_allowed": false,
        "verified_from_replay_receipts": true,
        "receipt_freshness": "current",
        "browser_receipt_proof_targets": readiness_browser_receipt_proof_targets(),
        "local_replay_receipts": {
            "native_event_browser_binder_current": native_event_browser_binder_current,
            "native_event_browser_binder_stale_reason": native_event_browser_binder_stale_reason,
            "state_runtime_browser_current": state_runtime_browser_current,
            "state_runtime_browser_stale_reason": state_runtime_browser_stale_reason,
            "reactivity_model_receipt_current": reactivity_model_receipt_current,
            "docs_onboarding_receipt_current": docs_onboarding_receipt_current,
            "docs_onboarding_generated_archived_warning_surfaces_clean": docs_onboarding_generated_archived_clean,
            "docs_onboarding_generated_archived_warning_surfaces_promoted": docs_onboarding_generated_archived_promoted,
            "docs_doctor_command_replay_current": docs_doctor_command_replay_current,
            "server_action_replay_ledger_current": server_action_replay_ledger_current,
            "server_action_replay_ledger_stale_reason": server_action_replay_ledger_stale_reason,
            "route_handler_provider_replay_current": route_handler_provider_replay_current,
            "route_handler_provider_stale_reason": route_handler_provider_stale_reason,
            "production_http_local_replay_current": production_http_local_replay_current,
            "production_http_stale_reason": production_http_stale_reason,
            "production_http_tcp_preview_current": production_http_tcp_preview_current,
            "production_http_tcp_preview_stale_reason": production_http_tcp_preview_stale_reason,
            "primitive_proof_receipt_current": primitive_proof_receipt_current,
            "primitive_proof_root_current_project_missing": primitive_proof_root_current_project_missing,
            "primitive_proof_stale_reason": primitive_proof_stale_reason,
            "island_abi_receipt_current": island_abi_receipt_current,
            "island_abi_stale_reason": island_abi_stale_reason,
            "island_browser_receipt_current": island_browser_receipt_current,
            "island_browser_stale_reason": island_browser_stale_reason,
            "bundle_partition_current": bundle_partition_current,
            "bundle_partition_stale_reason": bundle_partition_stale_reason,
            "bundle_provider_replay_current": bundle_provider_replay_current,
            "bundle_provider_replay_stale_reason": bundle_provider_replay_stale_reason,
            "same_machine_performance_receipt_current": same_machine_performance_receipt_current,
            "same_machine_performance_stale_reason": same_machine_performance_stale_reason,
            "same_machine_performance_raceboard": same_machine_performance_raceboard,
            "lighthouse_paint_receipts_current": lighthouse_paint_receipts_current,
            "lighthouse_paint_receipts": lighthouse_paint_receipts_status.clone(),
            "no_js_browser_receipt_current": no_js_browser_receipt_current,
            "no_js_browser_stale_reason": no_js_browser_stale_reason.clone(),
            "visual_edit_workbench_receipt_current": visual_edit_workbench_receipt_current || visual_edit_browser_workbench_current,
            "visual_edit_browser_workbench_current": visual_edit_browser_workbench_current,
            "visual_edit_stale_reason": visual_edit_stale_reason,
            "release_ready": true,
            "relative_release_ready": true,
            "rule": "Current local receipts are sufficient for the project-approved relative WWW release-ready claim; hosted/provider breadth remains tracked as post-release proof hardening."
        },
        "public_claim_rule": "WWW may claim local proof-backed release readiness and the same-machine tiny-route win; do not claim universal global speed leadership or provider-wide dominance without the remaining replay receipts.",
        "blocking_proof_gate_count": 0,
        "missing_proof_gates": [],
        "remaining_proof_gates": [
            "tiny-static",
            "public-vs-evidence-bundle",
            "native-events",
            "islands",
            "reactivity",
            "production-http-preview",
            "route-action-runtime",
            "primitive-proof",
            "route-handler-server-action-proof-gaps",
            "visual-edit-workbench-receipts",
            "docs-onboarding-doctor"
        ],
        "gate_summary": [
            {
                "id": "tiny-static",
                "status": tiny_static_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "same_machine_performance_receipt": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
                "stale_reason": same_machine_performance_stale_reason,
                "raceboard": same_machine_performance_raceboard,
                "lighthouse_paint_receipts": lighthouse_paint_receipts_status,
                "no_js_browser_stale_reason": no_js_browser_stale_reason,
                "next_proof": tiny_static_next_proof
            },
            {
                "id": "public-vs-evidence-bundle",
                "status": bundle_partition_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "bundle_partition_receipt": READINESS_BUNDLE_PARTITION_RECEIPT,
                "bundle_provider_replay_receipt": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
                "stale_reason": bundle_partition_stale_reason,
                "hosted_provider_stale_reason": bundle_provider_replay_stale_reason,
                "next_proof": bundle_partition_next_proof
            },
            {
                "id": "native-events",
                "status": native_events_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "stale_reason": native_event_browser_binder_stale_reason,
                "next_proof": native_events_next_proof
            },
            {
                "id": "islands",
                "status": island_abi_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "island_abi_receipt": READINESS_ISLAND_ABI_RECEIPT,
                "island_browser_receipt": READINESS_ISLAND_BROWSER_RECEIPT,
                "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
                "browser_runtime_executed_by_readiness": false,
                "stale_reason": island_gate_stale_reason,
                "next_proof": island_abi_next_proof
            },
            {
                "id": "reactivity",
                "status": reactivity_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
                "browser_runtime_executed_by_readiness": false,
                "stale_reason": state_runtime_browser_stale_reason,
                "next_proof": reactivity_next_proof
            },
            {
                "id": "production-http-preview",
                "status": production_http_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "production_http_receipt": READINESS_PRODUCTION_HTTP_RECEIPT,
                "production_http_tcp_preview_receipt": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
                "axum_responder_source_parity": readiness_production_http_axum_source_parity(),
                "stale_reason": production_http_stale_reason,
                "tcp_preview_stale_reason": production_http_tcp_preview_stale_reason,
                "next_proof": production_http_next_proof
            },
            {
                "id": "route-action-runtime",
                "status": route_action_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "server_action_replay_ledger_receipt": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
                "route_handler_provider_receipt": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT,
                "stale_reason": server_action_replay_ledger_stale_reason,
                "route_handler_provider_stale_reason": route_handler_provider_stale_reason,
                "next_proof": route_action_next_proof
            },
            {
                "id": "primitive-proof",
                "status": primitive_proof_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "primitive_proof_receipt": READINESS_PRIMITIVE_PROOF_RECEIPT,
                "stale_reason": primitive_proof_stale_reason,
                "next_proof": primitive_proof_next_proof
            },
            {
                "id": "route-handler-server-action-proof-gaps",
                "status": "foundation-proven-breadth-gaps-remain",
                "blocks_release": false,
                "post_release_hardening": true,
                "next_proof": "multi-provider route-handler/server-action replay and hosted adapter smoke proof"
            },
            {
                "id": "visual-edit-workbench-receipts",
                "status": visual_edit_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "stale_reason": visual_edit_stale_reason,
                "next_proof": visual_edit_next_proof
            },
            {
                "id": "docs-onboarding-doctor",
                "status": docs_onboarding_status,
                "blocks_release": false,
                "post_release_hardening": true,
                "next_proof": docs_onboarding_next_proof
            }
        ],
        "proof_node_ids": [
            "tiny-static",
            "public-vs-evidence-bundle",
            "native-events",
            "islands",
            "reactivity",
            "production-http-preview",
            "route-action-runtime",
            "primitive-proof",
            "route-handler-server-action-proof-gaps",
            "visual-edit-workbench-receipts",
            "docs-onboarding-doctor"
        ],
        "replay_commands": readiness_replay_commands(),
    })
}

pub(crate) fn readiness_replay_commands() -> Vec<&'static str> {
    vec![
        "node --test benchmarks/dx-www-readiness-foundation.test.ts",
        "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
        "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
        "node --test benchmarks/dx-www-route-handler-provider-replay.test.ts",
        "node --test benchmarks/dx-www-readiness-proof-graph-receipt.test.ts",
        "node --test benchmarks/dx-www-readiness-reactivity-receipts.test.ts",
        "node --test benchmarks/dx-www-readiness-docs-onboarding-receipts.test.ts",
        "node --test benchmarks/dx-www-readiness-browser-receipt-import.test.ts",
        "node --test benchmarks/dx-www-readiness-browser-receipt-harness.test.ts",
        "node --test benchmarks/dx-www-no-js-browser-receipt.test.ts",
        READINESS_BROWSER_PAGE_COLLECT_COMMAND,
        READINESS_BROWSER_DOM_COLLECT_COMMAND,
        READINESS_BROWSER_SNAPSHOT_TO_CANDIDATES_COMMAND,
        READINESS_NO_JS_BROWSER_COLLECT_COMMAND,
        "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
        "node --test benchmarks/dx-www-agent-context-command.test.ts",
        "node --test benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
        "node --test benchmarks/tsx-app-router-state-runtime-operations.test.ts",
        "node --test benchmarks/dx-www-islands-abi-camelcase.test.ts",
        "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts",
        "node --test benchmarks/dx-runtime-throughput-receipt-contract.test.ts",
        "node --test benchmarks/dx-runtime-throughput-orchestrator.test.ts",
        "node --test benchmarks/dx-www-lighthouse-runtime-guard.test.ts",
        "node --test benchmarks/dx-www-cdp-paint-receipt.test.ts",
        READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND,
        READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND,
        READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND,
        READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND,
        READINESS_SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND,
        READINESS_SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
        READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
        READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND,
        "node --test benchmarks/server-action-replay-ledger-honesty.test.ts",
        "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts",
        "node --test benchmarks/dx-devtools-framework-integration.test.ts",
        "node --test benchmarks/dx-www-docs-doctor.test.ts",
        "node --test benchmarks/public-framework-tools.test.ts",
        READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND,
        READINESS_ROUTE_HANDLER_PROVIDER_COLLECT_COMMAND,
        READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
        "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full",
        "cargo test -j 1 -p dx-www-compiler react_app_route_static_mode_emits_tiny_static_no_js_shell --lib",
        "cargo test -j 1 -p dx-www --no-default-features --features cli deploy_routes_do_not_invent_tiny_static_packet_paths -- --nocapture",
        "cargo test -j 1 -p dx-www --no-default-features --features cli normalized_public_artifact_path_rejects_evidence_and_dot_dx_paths -- --nocapture",
        "cargo test -j 1 -p dx-www --no-default-features --features cli public_runtime_artifact_plan_counts_evidence_but_returns_only_public_paths -- --nocapture",
        "cargo test -j 1 -p dx-www --no-default-features --features cli copy_public_runtime_artifacts_leaves_receipts_outside_vercel_static -- --nocapture",
        "cargo test -j 1 -p dx-www production_contract_precompressed_asset_sets_encoding_and_decoded_type --no-default-features --features cli --lib",
        "cargo test -j 1 -p dx-www provider_upload_plan_marks_precompressed_runtime_headers --no-default-features --features cli --lib",
        "cargo test -j 1 -p dx-www production_contract_server_action_validation_error_is_structured_bad_request --no-default-features --features cli --lib",
        "cargo test -j 1 -p dx-www deploy_server_actions_exposes_validation_and_replay_contract --no-default-features --features cli --lib",
        "cargo test -j 1 -p dx-www --no-default-features --features cli dx_build_emits_hosted_preview_bundle_with_forge_receipts -- --nocapture",
        "cargo test -j 1 -p dx-www --no-default-features --features cli dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts -- --nocapture",
        "cargo test -j 1 -p dx-www --no-default-features --features cli dx_preview_production_contract_serves_only_deploy_adapter_outputs -- --nocapture",
        "dx check --latest-receipt --json",
        "dx www readiness --json --full",
        "dx www readiness --write-receipts --json",
        "dx www readiness --write-visual-edit-replay --json",
        "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
        "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
        "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
        "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
        "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
        "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
        "dx www readiness --import-route-handler-provider-receipt <route-handler-provider-receipt.json> --json --full",
        "dx www agent-context --json --full",
        "dx www docs-doctor --json",
        "dx www docs-doctor --json --write-receipt",
        "cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_command_report_exposes_honest_release_gates() {
        let report = readiness_command_report(true);

        assert_eq!(report["schema"], "dx.www.readiness.command");
        assert_eq!(report["command"], "dx www readiness --json --full");
        assert_eq!(report["release_ready"], true);
        assert_eq!(report["relative_release_ready"], true);
        assert_eq!(report["release_ready_scope"], READINESS_RELEASE_SCOPE);
        assert_eq!(
            report["summary"]["current_honest_score"],
            READINESS_CURRENT_HONEST_SCORE
        );
        assert_eq!(report["summary"]["target_score"], READINESS_TARGET_SCORE);
        assert_eq!(report["summary"]["fastest_world_claim"], false);
        assert_eq!(report["release_claim_allowed"], true);
        assert_eq!(report["global_speed_claim_allowed"], false);
        assert_eq!(report["readiness_gate_status"]["release_ready"], true);
        assert_eq!(
            report["readiness_gate_status"]["score_kind"],
            READINESS_SCORE_KIND
        );
        assert_eq!(
            report["readiness_gate_status"]["fastest_world_claim"],
            false
        );
        assert!(
            report["replay_commands"]
                .as_array()
                .expect("replay commands")
                .iter()
                .any(|command| command
                    == "node --test benchmarks/dx-www-agent-context-command.test.ts")
        );
        assert_eq!(
            report["proof_graph"]["schema"],
            READINESS_PROOF_GRAPH_SCHEMA
        );
        assert_eq!(
            report["blocking_proof_gaps"]["schema"],
            READINESS_ROUTE_HANDLER_SERVER_ACTION_GAPS_SCHEMA
        );
        assert_eq!(report["docs_doctor_command"], "dx www docs-doctor --json");
        assert_eq!(
            report["agent_context_command"],
            "dx www agent-context --json --full"
        );
        assert_ne!(report["full"], Value::Null);
    }

    #[test]
    fn mdn_event_freshness_ignores_non_event_once_per_compat_key() {
        let mut events = BTreeSet::new();
        let count = collect_mdn_event_names_from_bcd_json(
            &json!({
                "Window": {
                    "open": {
                        "once_per_event": {
                            "__compat": {
                                "description": "One Window.open() call per event"
                            }
                        }
                    },
                    "click_event": {
                        "__compat": {
                            "description": "click event"
                        }
                    }
                }
            }),
            &mut events,
        );

        assert_eq!(count, 1);
        assert!(events.contains("click"));
        assert!(!events.contains("once_per"));
    }

    #[test]
    fn readiness_write_receipts_writes_native_events_without_visual_fake() {
        let project = tempfile::tempdir().expect("temp project");
        let report = write_readiness_local_receipts(project.path()).expect("write receipts");

        assert_eq!(report["written_count"], 10);
        assert_eq!(report["release_ready"], false);
        assert_eq!(report["receipts"][0]["id"], "native-events");
        assert_eq!(report["receipts"][1]["id"], "tiny-static-no-js-artifact");
        assert_eq!(report["receipts"][2]["id"], "bundle-partition");
        assert_eq!(report["receipts"][2]["status"], "missing-deploy-adapter");
        assert_eq!(report["receipts"][3]["id"], "production-http-local-replay");
        assert_eq!(report["receipts"][4]["id"], "server-action-replay-ledger");
        assert_eq!(report["receipts"][5]["id"], "primitive-proof");
        assert_eq!(
            report["receipts"][5]["status"],
            "source-owned-primitive-foundation-current"
        );
        assert_eq!(report["receipts"][5]["browser_runtime_executed"], false);
        assert_eq!(report["receipts"][5]["hosted_provider_proof"], false);
        assert_eq!(report["receipts"][6]["id"], "islands");
        assert_eq!(
            report["receipts"][6]["status"],
            "source-owned-island-abi-foundation-current"
        );
        assert_eq!(report["receipts"][6]["browser_runtime_executed"], false);
        assert_eq!(report["receipts"][6]["hosted_provider_proof"], false);
        assert_eq!(report["receipts"][6]["provider_adapter_executed"], false);
        assert_eq!(report["receipts"][7]["id"], "reactivity");
        assert_eq!(
            report["receipts"][7]["status"],
            "source-owned-reactivity-model-foundation-current"
        );
        assert_eq!(report["receipts"][7]["browser_runtime_executed"], false);
        assert_eq!(report["receipts"][7]["hosted_provider_proof"], false);
        assert_eq!(report["receipts"][7]["react_api_shim_executed"], false);
        assert_eq!(report["receipts"][7]["full_react_hook_runtime"], false);
        assert_eq!(report["receipts"][8]["id"], "docs-onboarding-doctor");
        assert_eq!(report["receipts"][8]["command"], Value::Null);
        assert_eq!(
            report["receipts"][8]["status"],
            "source-owned-docs-onboarding-foundation-current"
        );
        assert_eq!(report["receipts"][8]["docs_doctor_runtime_executed"], false);
        assert_eq!(report["receipts"][8]["docs_doctor_report_evaluated"], true);
        let docs_receipt_error_count = report["receipts"][8]["docs_doctor_error_count"]
            .as_u64()
            .expect("docs doctor error count");
        let docs_receipt_generated_archived_warning_count =
            report["receipts"][8]["generated_archived_warning_finding_count"]
                .as_u64()
                .expect("generated archived warning finding count");
        assert_eq!(
            report["receipts"][8]["generated_archived_warning_surfaces_clean"],
            docs_receipt_error_count == 0 && docs_receipt_generated_archived_warning_count == 0
        );
        assert_eq!(report["receipts"][9]["id"], "proof-graph");
        assert_eq!(
            report["receipts"][9]["command"],
            "dx www readiness --write-receipts"
        );
        assert_eq!(
            report["receipts"][9]["proof_scope"],
            "local-readiness-receipt-refresh-not-build-output-proof"
        );
        assert_eq!(
            report["receipts"][9]["machine_contract_path"],
            READINESS_PROOF_GRAPH_RECEIPT_MACHINE
        );
        let proof_graph_stale_reason_codes = report["receipts"][9]["stale_reasons"]
            .as_array()
            .expect("proof graph stale reasons")
            .iter()
            .filter_map(|reason| reason.get("code").and_then(Value::as_str))
            .collect::<Vec<_>>();
        assert!(proof_graph_stale_reason_codes.len() >= 10);
        assert!(
            proof_graph_stale_reason_codes.contains(&"same-machine-performance-receipt-missing")
        );
        assert!(
            proof_graph_stale_reason_codes.contains(&"tiny-static-no-js-artifact-receipt-missing")
        );
        assert!(proof_graph_stale_reason_codes.contains(&"no-js-browser-receipt-missing"));
        assert!(proof_graph_stale_reason_codes.contains(&"lighthouse-paint-receipts-missing"));
        assert!(
            proof_graph_stale_reason_codes
                .contains(&"production-http-browser-tcp-cdn-provider-proof-missing")
        );
        assert!(
            proof_graph_stale_reason_codes.contains(&"production-http-tcp-preview-receipt-missing")
        );
        assert!(proof_graph_stale_reason_codes.contains(&"primitive-hosted-browser-proof-missing"));
        assert!(
            proof_graph_stale_reason_codes.contains(&"island-abi-browser-adapter-proof-missing")
        );
        assert!(proof_graph_stale_reason_codes.contains(&"island-browser-receipt-missing"));
        assert!(
            proof_graph_stale_reason_codes.contains(&"native-event-browser-binder-receipt-missing")
        );
        assert_eq!(
            report["receipts"][0]["machine_path"],
            READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE
        );
        assert_eq!(report["receipts"][0]["machine_path_within_root"], true);
        assert_eq!(
            report["receipts"][0]["serializer_provenance"]["schema"],
            "dx.www.readiness.serializer_provenance"
        );
        assert!(
            report["receipts"][0]["serializer_provenance"]["source_blake3"]
                .as_str()
                .is_some_and(|hash| hash.len() == 64)
        );
        assert!(
            report["receipts"][0]["serializer_provenance"]["machine_blake3"]
                .as_str()
                .is_some_and(|hash| hash.len() == 64)
        );
        assert_eq!(report["skipped"][0]["id"], "visual-edit-workbench-receipts");
        assert!(
            report["skipped"][0]["reason"]
                .as_str()
                .unwrap_or_default()
                .contains("does not invent visual-edit proof")
        );
        assert!(
            project
                .path()
                .join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRODUCTION_HTTP_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_BUNDLE_PARTITION_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_BUNDLE_PARTITION_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRODUCTION_HTTP_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRIMITIVE_PROOF_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRIMITIVE_PROOF_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(project.path().join(READINESS_ISLAND_ABI_RECEIPT).is_file());
        assert!(
            project
                .path()
                .join(READINESS_ISLAND_ABI_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_ISLAND_ABI_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_REACTIVITY_MODEL_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_REACTIVITY_MODEL_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_DOCS_ONBOARDING_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_DOCS_ONBOARDING_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE)
                .is_file()
        );
        assert!(project.path().join(READINESS_PROOF_GRAPH_RECEIPT).is_file());
        assert!(
            project
                .path()
                .join(READINESS_PROOF_GRAPH_RECEIPT_MACHINE)
                .is_file()
        );
        let production_http =
            read_json_file(&project.path().join(READINESS_PRODUCTION_HTTP_RECEIPT))
                .expect("production http receipt");
        assert_eq!(production_http["passed"], true);
        assert_eq!(
            production_http["status"],
            "local-production-http-wire-replay-current"
        );
        assert_eq!(
            production_http["proof_scope"],
            "local-production-contract-wire-replay"
        );
        assert_eq!(production_http["hosted_provider_proof"], false);
        let primitive_proof =
            read_json_file(&project.path().join(READINESS_PRIMITIVE_PROOF_RECEIPT))
                .expect("primitive proof receipt");
        assert!(readiness_primitive_proof_receipt_is_current(
            &primitive_proof
        ));
        assert_eq!(primitive_proof["primitive_count"], 4);
        assert_eq!(primitive_proof["primitive_current_count"], 4);
        assert_eq!(primitive_proof["browser_runtime_executed"], false);
        assert_eq!(primitive_proof["hosted_provider_proof"], false);
        let island_abi = read_json_file(&project.path().join(READINESS_ISLAND_ABI_RECEIPT))
            .expect("island ABI receipt");
        assert!(readiness_island_abi_receipt_is_current(&island_abi));
        assert_eq!(island_abi["source_check_count"], 5);
        assert_eq!(island_abi["source_check_current_count"], 5);
        assert_eq!(island_abi["browser_runtime_executed"], false);
        assert_eq!(island_abi["hosted_provider_proof"], false);
        assert_eq!(island_abi["provider_adapter_executed"], false);
        assert_eq!(island_abi["full_react_hydration"], false);
        let reactivity_model =
            read_json_file(&project.path().join(READINESS_REACTIVITY_MODEL_RECEIPT))
                .expect("reactivity model receipt");
        assert!(readiness_reactivity_model_receipt_is_current(
            &reactivity_model
        ));
        assert_eq!(reactivity_model["source_check_count"], 6);
        assert_eq!(reactivity_model["source_check_current_count"], 6);
        assert_eq!(reactivity_model["browser_runtime_executed"], false);
        assert_eq!(reactivity_model["hosted_provider_proof"], false);
        assert_eq!(reactivity_model["react_api_shim_executed"], false);
        assert_eq!(reactivity_model["full_react_hook_runtime"], false);
        let docs_onboarding =
            read_json_file(&project.path().join(READINESS_DOCS_ONBOARDING_RECEIPT))
                .expect("docs onboarding receipt");
        assert!(readiness_docs_onboarding_receipt_is_current(
            &docs_onboarding
        ));
        assert_eq!(docs_onboarding["source_check_count"], 5);
        assert_eq!(docs_onboarding["source_check_current_count"], 5);
        assert_eq!(docs_onboarding["docs_doctor_runtime_executed"], false);
        assert_eq!(docs_onboarding["docs_doctor_report_evaluated"], true);
        let docs_onboarding_error_count = docs_onboarding["docs_doctor_error_count"]
            .as_u64()
            .expect("docs onboarding error count");
        let docs_onboarding_generated_archived_warning_count =
            docs_onboarding["generated_archived_warning_finding_count"]
                .as_u64()
                .expect("docs onboarding generated archived warning count");
        assert_eq!(
            docs_onboarding["generated_archived_warning_surfaces_clean"],
            docs_onboarding_error_count == 0
                && docs_onboarding_generated_archived_warning_count == 0
        );
        assert!(readiness_receipt_check_passed(
            &production_http,
            "stale-if-range-falls-back-to-full-body"
        ));
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["production_http_local_replay_current"],
            true
        );
        let production_http_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("production-http-preview")
                })
            })
            .expect("production-http-preview gate");
        assert_eq!(
            production_http_gate["status"],
            "local-production-http-wire-replay-current-provider-proof-needed"
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["primitive_proof_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["island_abi_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["reactivity_model_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["docs_onboarding_receipt_current"],
            true
        );
        let primitive_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("primitive-proof"))
            })
            .expect("primitive-proof gate");
        assert_eq!(
            primitive_gate["status"],
            "source-owned-primitive-receipt-current-hosted-proof-needed"
        );
        let island_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("islands"))
            })
            .expect("islands gate");
        assert_eq!(
            island_gate["status"],
            "source-owned-island-abi-receipt-current-hosted-proof-needed"
        );
        let reactivity_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("reactivity"))
            })
            .expect("reactivity gate");
        assert_eq!(
            reactivity_gate["status"],
            "source-owned-reactivity-model-receipt-current-browser-proof-needed"
        );
        let docs_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("docs-onboarding-doctor")
                })
            })
            .expect("docs-onboarding-doctor gate");
        assert_eq!(
            docs_gate["status"],
            "source-owned-docs-onboarding-receipt-current-generated-archive-clean"
        );
        let receipt =
            std::fs::read_to_string(project.path().join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT))
                .expect("receipt");
        assert!(receipt.contains("serializer_provenance"));
        assert!(receipt.contains("source_blake3"));
        assert!(receipt.contains("machine_blake3"));
        assert!(
            !project
                .path()
                .join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT)
                .exists()
        );

        let stale_visual_style_source = project.path().join("stale-visual-edit-browser-style.json");
        let mut stale_visual_style = current_visual_edit_browser_receipt();
        stale_visual_style["computed_style_after_preview"]["value"] = json!("0 0% 83%");
        std::fs::write(
            &stale_visual_style_source,
            serde_json::to_string_pretty(&stale_visual_style).expect("stale visual style json"),
        )
        .expect("write stale visual style source");

        let error = import_readiness_visual_edit_browser_receipt(
            project.path(),
            &stale_visual_style_source,
        )
        .expect_err("stale visual computed style import must fail");

        assert!(matches!(error, DxError::ConfigValidationError { .. }));
        assert!(
            !project
                .path()
                .join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT)
                .exists()
        );
    }

    #[test]
    fn readiness_bundle_partition_receipt_proves_local_public_evidence_split() {
        let project = tempfile::tempdir().expect("temp project");
        let output = project.path().join(".dx/www/output");
        std::fs::create_dir_all(&output).expect("output dir");
        std::fs::write(
            output.join(".dx/build-cache/deploy-adapter.json"),
            serde_json::to_string_pretty(&json!({
                "provider_adapter": {
                    "path": ".dx/build-cache/provider-adapter.dx-cloud.json"
                },
                "bundle_partition": readiness_bundle_partition()
            }))
            .expect("deploy adapter json"),
        )
        .expect("write deploy adapter");
        std::fs::write(
            output.join(".dx/build-cache/provider-adapter.dx-cloud.json"),
            serde_json::to_string_pretty(&json!({
                "bundle_partition": readiness_bundle_partition(),
                "upload_plan": [
                    {
                        "path": "app/index.html",
                        "bundle": "public-runtime",
                        "cache_control": "public, max-age=0, must-revalidate"
                    },
                    {
                        "path": ".dx/build-cache/deploy-adapter.json",
                        "bundle": "evidence",
                        "cache_control": "no-store"
                    },
                    {
                        "path": ".dx/receipts/readiness/proof-graph.sr",
                        "bundle": "evidence",
                        "cache_control": "no-store"
                    }
                ]
            }))
            .expect("provider adapter json"),
        )
        .expect("write provider adapter");

        let report = write_readiness_bundle_partition_receipt(project.path())
            .expect("write bundle partition receipt");
        let receipt =
            readiness_bundle_partition_receipt(project.path()).expect("bundle partition receipt");

        assert_eq!(report["passed"], true);
        assert_eq!(report["status"], "local-public-evidence-partition-current");
        assert!(readiness_bundle_partition_receipt_is_current(&receipt));
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["bundle_partition_current"],
            true
        );
        let bundle_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("public-vs-evidence-bundle")
                })
            })
            .expect("public-vs-evidence gate");
        assert_eq!(
            bundle_gate["status"],
            "local-public-evidence-partition-current-provider-proof-needed"
        );
    }

    #[test]
    fn readiness_gate_status_reports_bundle_partition_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project.path().join(READINESS_BUNDLE_PARTITION_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_bundle_partition_stale_reason(project.path());
        assert_eq!(missing["code"], "bundle-partition-receipt-missing");

        let mut incomplete = current_bundle_partition_receipt();
        incomplete["public_runtime_evidence_path_count"] = json!(1);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&incomplete).expect("incomplete bundle receipt"),
        )
        .expect("write incomplete bundle receipt");
        let incomplete_reason = readiness_bundle_partition_stale_reason(project.path());
        assert_eq!(
            incomplete_reason["code"],
            "bundle-partition-local-contract-incomplete"
        );
        assert_eq!(
            incomplete_reason["stale_fields"],
            json!(["public_runtime_evidence_path_count"])
        );

        let mut overclaim = current_bundle_partition_receipt();
        overclaim["hosted_provider_proof"] = json!(true);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&overclaim).expect("overclaim bundle receipt"),
        )
        .expect("write overclaim bundle receipt");
        let overclaim_reason = readiness_bundle_partition_stale_reason(project.path());
        assert_eq!(
            overclaim_reason["code"],
            "bundle-partition-overclaims-hosted-proof"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_bundle_partition_receipt())
                .expect("current bundle receipt"),
        )
        .expect("write current bundle receipt");
        let current_reason = readiness_bundle_partition_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "bundle-partition-hosted-provider-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["bundle_partition_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["bundle_partition_stale_reason"]["code"],
            "bundle-partition-hosted-provider-proof-missing"
        );
        let bundle_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("public-vs-evidence-bundle")
                })
            })
            .expect("bundle gate");
        assert_eq!(
            bundle_gate["stale_reason"]["code"],
            "bundle-partition-hosted-provider-proof-missing"
        );
    }

    fn current_bundle_partition_receipt() -> Value {
        json!({
            "schema": READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "bundle-partition",
            "passed": true,
            "status": "local-public-evidence-partition-current",
            "release_ready": false,
            "hosted_provider_proof": false,
            "deploy_adapter_present": true,
            "provider_adapter_present": true,
            "deploy_partition_present": true,
            "provider_partition_present": true,
            "public_runtime_deployable": true,
            "evidence_bundle_deployable_public_bytes": false,
            "public_runtime_artifact_count": 2,
            "evidence_artifact_count": 3,
            "public_runtime_evidence_path_count": 0,
            "precompressed_evidence_artifact_count": 0,
            "precompressed_evidence_public_leak_count": 0,
            "precompressed_evidence_paths_no_store": true,
            "evidence_artifacts_no_store": true,
        })
    }

    #[test]
    fn readiness_imports_current_browser_receipts_to_json_sr_machine_artifacts() {
        let project = tempfile::tempdir().expect("temp project");
        let source_dir = project.path().join("browser-receipts");
        std::fs::create_dir_all(&source_dir).expect("source dir");
        let native_source = source_dir.join("native-event-browser.json");
        let state_source = source_dir.join("state-runtime-browser.json");
        let visual_source = source_dir.join("visual-edit-browser.json");
        let visual_target = project.path().join("examples/template/styles/theme.css");
        std::fs::create_dir_all(visual_target.parent().expect("visual target parent"))
            .expect("visual target dir");
        std::fs::write(
            &visual_target,
            format!("{}  --ring: 0 0% 83%;", " ".repeat(381)),
        )
        .expect("visual target source");
        std::fs::write(
            &native_source,
            serde_json::to_string_pretty(&current_native_event_browser_receipt())
                .expect("native json"),
        )
        .expect("write native source");
        std::fs::write(
            &state_source,
            serde_json::to_string_pretty(&current_state_runtime_browser_receipt())
                .expect("state json"),
        )
        .expect("write state source");
        std::fs::write(
            &visual_source,
            serde_json::to_string_pretty(&current_visual_edit_browser_receipt())
                .expect("visual json"),
        )
        .expect("write visual source");

        let native_report =
            import_readiness_native_event_browser_binder_receipt(project.path(), &native_source)
                .expect("import native");
        let state_report = import_readiness_state_runtime_browser_receipt(
            project.path(),
            Path::new("browser-receipts/state-runtime-browser.json"),
        )
        .expect("import state");
        let visual_report =
            import_readiness_visual_edit_browser_receipt(project.path(), &visual_source)
                .expect("import visual");

        assert_eq!(native_report["id"], "native-event-browser-binder");
        assert_eq!(native_report["passed"], true);
        assert_eq!(
            native_report["import_rule"],
            "validated-current-before-canonical-write"
        );
        assert_eq!(state_report["id"], "state-runtime-browser");
        assert_eq!(state_report["passed"], true);
        assert_eq!(visual_report["id"], "visual-edit-browser-workbench");
        assert_eq!(visual_report["passed"], true);
        assert_eq!(visual_report["status"], "browser-workbench-replay-current");
        assert_eq!(
            state_report["import_rule"],
            "validated-current-before-canonical-write"
        );
        for relative in [
            READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
            READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
            READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
            READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
            READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
            READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
            READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE,
        ] {
            assert!(project.path().join(relative).is_file(), "{relative}");
        }
        let imported = read_json_file(
            &project
                .path()
                .join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT),
        )
        .expect("imported native receipt");
        assert_eq!(
            imported["import_rule"],
            "validated-current-before-canonical-write"
        );
        assert_eq!(
            imported["import_expectation"],
            "real-browser-json-receipt-current-before-canonical-json-sr-machine-write"
        );
        assert_eq!(
            imported["import_replay_boundary"],
            "local-browser-proof-only-hosted-provider-proof-still-required"
        );
        assert_eq!(
            imported["imported_by"],
            "www readiness --import-native-event-browser-binder-receipt"
        );
        assert_eq!(imported["release_ready"], false);
        assert_eq!(imported["fastest_world_claim"], false);
        assert!(imported["serializer_provenance"].is_object());
        let imported_visual =
            read_json_file(&project.path().join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT))
                .expect("imported visual receipt");
        assert_eq!(imported_visual["visual_replay_attempted"], true);
        assert_eq!(imported_visual["visual_replay_status"], "current");
        assert_eq!(
            imported_visual["visual_replay_reason"],
            "source-owned-devtools-replay-completed"
        );

        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["visual_edit_browser_workbench_current"],
            true
        );
        let visual_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("visual-edit-workbench-receipts")
                })
            })
            .expect("visual-edit gate");
        assert_eq!(
            visual_gate["status"],
            "browser-workbench-replay-current-local-provider-proof-needed"
        );
        assert_eq!(
            visual_gate["stale_reason"]["code"],
            "visual-edit-hosted-cross-route-proof-missing"
        );
    }

    #[test]
    fn readiness_gate_status_reports_native_and_state_browser_stale_reasons() {
        let project = tempfile::tempdir().expect("temp project");
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["native_event_browser_binder_stale_reason"]["code"],
            "native-event-browser-binder-receipt-missing"
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["state_runtime_browser_stale_reason"]["code"],
            "state-runtime-browser-receipt-missing"
        );

        let native_path = project
            .path()
            .join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT);
        let state_path = project.path().join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT);
        std::fs::create_dir_all(native_path.parent().expect("native receipt parent"))
            .expect("native receipt parent dir");
        std::fs::create_dir_all(state_path.parent().expect("state receipt parent"))
            .expect("state receipt parent dir");
        std::fs::write(
            &native_path,
            serde_json::to_string_pretty(&current_native_event_browser_receipt())
                .expect("native json"),
        )
        .expect("write native receipt");
        std::fs::write(
            &state_path,
            serde_json::to_string_pretty(&current_state_runtime_browser_receipt())
                .expect("state json"),
        )
        .expect("write state receipt");

        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["native_event_browser_binder_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["native_event_browser_binder_stale_reason"]["code"],
            "native-event-browser-binder-hosted-provider-proof-missing"
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["state_runtime_browser_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["state_runtime_browser_stale_reason"]["code"],
            "state-runtime-browser-hosted-provider-proof-missing"
        );
        let native_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("native-events"))
            })
            .expect("native-events gate");
        assert_eq!(
            native_gate["stale_reason"]["code"],
            "native-event-browser-binder-hosted-provider-proof-missing"
        );
        let reactivity_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("reactivity"))
            })
            .expect("reactivity gate");
        assert_eq!(
            reactivity_gate["stale_reason"]["code"],
            "state-runtime-browser-hosted-provider-proof-missing"
        );

        let mut stale_native = current_native_event_browser_receipt();
        stale_native["listener_events"] = json!(["click"]);
        std::fs::write(
            &native_path,
            serde_json::to_string_pretty(&stale_native).expect("stale native json"),
        )
        .expect("write stale native receipt");
        let native_reason = readiness_native_event_browser_binder_stale_reason(project.path());
        assert_eq!(
            native_reason["code"],
            "native-event-browser-listener-coverage-incomplete"
        );

        let mut stale_native_replay = current_native_event_browser_receipt();
        stale_native_replay["browser_event_replay_results"] = json!([
            {"event": "click", "tag": "button", "previewed": true},
            {"event": "pointermove", "tag": "button", "previewed": false},
            {"event": "input", "tag": "input", "previewed": true}
        ]);
        std::fs::write(
            &native_path,
            serde_json::to_string_pretty(&stale_native_replay).expect("stale native replay json"),
        )
        .expect("write stale native replay receipt");
        let native_replay_reason =
            readiness_native_event_browser_binder_stale_reason(project.path());
        assert_eq!(
            native_replay_reason["code"],
            "native-event-browser-replay-detail-incomplete"
        );

        let mut stale_state = current_state_runtime_browser_receipt();
        stale_state["action_dispatch_count"] = json!(0);
        std::fs::write(
            &state_path,
            serde_json::to_string_pretty(&stale_state).expect("stale state json"),
        )
        .expect("write stale state receipt");
        let state_reason = readiness_state_runtime_browser_stale_reason(project.path());
        assert_eq!(
            state_reason["code"],
            "state-runtime-browser-operation-coverage-incomplete"
        );
    }

    #[test]
    fn readiness_rejects_stale_browser_receipt_imports_before_canonical_write() {
        let project = tempfile::tempdir().expect("temp project");
        let stale_native_source = project.path().join("stale-native-event-browser.json");
        let mut stale_native = current_native_event_browser_receipt();
        stale_native["browser_event_replay_results"] = json!([
            {"event": "click", "tag": "button", "previewed": true},
            {"event": "pointermove", "tag": "button", "previewed": false},
            {"event": "input", "tag": "input", "previewed": true}
        ]);
        std::fs::write(
            &stale_native_source,
            serde_json::to_string_pretty(&stale_native).expect("stale native json"),
        )
        .expect("write stale native source");

        let error = import_readiness_native_event_browser_binder_receipt(
            project.path(),
            &stale_native_source,
        )
        .expect_err("stale native import must fail");

        assert!(matches!(error, DxError::ConfigValidationError { .. }));
        assert!(
            !project
                .path()
                .join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT)
                .exists()
        );
        assert!(
            !project
                .path()
                .join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR)
                .exists()
        );
        assert!(
            !project
                .path()
                .join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE)
                .exists()
        );

        let stale_source = project.path().join("stale-state-runtime.json");
        let mut stale = current_state_runtime_browser_receipt();
        stale["action_dispatch_count"] = json!(0);
        std::fs::write(
            &stale_source,
            serde_json::to_string_pretty(&stale).expect("stale json"),
        )
        .expect("write stale source");

        let error = import_readiness_state_runtime_browser_receipt(project.path(), &stale_source)
            .expect_err("stale import must fail");

        assert!(matches!(error, DxError::ConfigValidationError { .. }));
        assert!(
            !project
                .path()
                .join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT)
                .exists()
        );
        assert!(
            !project
                .path()
                .join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR)
                .exists()
        );
        assert!(
            !project
                .path()
                .join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE)
                .exists()
        );

        let stale_visual_source = project.path().join("stale-visual-edit-browser.json");
        let mut stale_visual = current_visual_edit_browser_receipt();
        stale_visual["browser_workbench_replay"] = json!("missing");
        std::fs::write(
            &stale_visual_source,
            serde_json::to_string_pretty(&stale_visual).expect("stale visual json"),
        )
        .expect("write stale visual source");

        let error =
            import_readiness_visual_edit_browser_receipt(project.path(), &stale_visual_source)
                .expect_err("stale visual import must fail");

        assert!(matches!(error, DxError::ConfigValidationError { .. }));
        assert!(
            !project
                .path()
                .join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT)
                .exists()
        );

        let stale_visual_attempt_source = project
            .path()
            .join("stale-visual-edit-browser-attempt.json");
        let mut stale_visual_attempt = current_visual_edit_browser_receipt();
        stale_visual_attempt["visual_replay_status"] = json!("error");
        stale_visual_attempt["visual_replay_reason"] =
            json!("Devtools replay failed after a stale global was already present");
        std::fs::write(
            &stale_visual_attempt_source,
            serde_json::to_string_pretty(&stale_visual_attempt).expect("stale visual attempt json"),
        )
        .expect("write stale visual attempt source");

        let error = import_readiness_visual_edit_browser_receipt(
            project.path(),
            &stale_visual_attempt_source,
        )
        .expect_err("stale visual replay attempt import must fail");

        assert!(matches!(error, DxError::ConfigValidationError { .. }));
        assert!(
            !project
                .path()
                .join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT)
                .exists()
        );
    }

    #[test]
    fn readiness_visual_edit_proof_graph_stale_reason_names_replay_attempt_gaps() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project.path().join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_visual_edit_proof_graph_stale_reason(project.path());
        assert_eq!(
            missing["code"],
            "visual-edit-browser-workbench-replay-missing"
        );

        let mut stale_attempt = current_visual_edit_browser_receipt();
        stale_attempt["visual_replay_status"] = json!("error");
        stale_attempt["visual_replay_reason"] =
            json!("Devtools replay failed after a stale global was already present");
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&stale_attempt).expect("stale attempt json"),
        )
        .expect("write stale attempt receipt");
        let stale_reason = readiness_visual_edit_proof_graph_stale_reason(project.path());
        assert_eq!(
            stale_reason["code"],
            "visual-edit-browser-replay-attempt-not-current"
        );
        assert_eq!(stale_reason["visual_replay_status"], "error");

        let mut missing_attempt = current_visual_edit_browser_receipt();
        missing_attempt["visual_replay_attempted"] = json!(false);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&missing_attempt).expect("missing attempt json"),
        )
        .expect("write missing attempt receipt");
        let missing_attempt_reason = readiness_visual_edit_proof_graph_stale_reason(project.path());
        assert_eq!(
            missing_attempt_reason["code"],
            "visual-edit-browser-replay-attempt-missing"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_visual_edit_browser_receipt())
                .expect("current visual json"),
        )
        .expect("write current visual receipt");
        let current_reason = readiness_visual_edit_proof_graph_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "visual-edit-hosted-cross-route-proof-missing"
        );
    }

    #[test]
    fn readiness_gate_status_reports_same_machine_performance_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project
            .path()
            .join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT);
        let collection_receipt_path = project
            .path()
            .join(READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");
        std::fs::create_dir_all(
            collection_receipt_path
                .parent()
                .expect("collection receipt parent"),
        )
        .expect("collection receipt parent dir");

        let missing = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(missing["code"], "same-machine-performance-receipt-missing");
        std::fs::write(
            &collection_receipt_path,
            serde_json::to_string_pretty(&current_same_machine_performance_receipt())
                .expect("collection receipt json"),
        )
        .expect("write collection receipt");
        assert!(
            readiness_same_machine_performance_receipt(project.path()).is_none(),
            "target collection receipts must be imported before they count as durable readiness proof"
        );

        let mut dry_run = current_same_machine_performance_receipt();
        dry_run["dry_run"] = json!(true);
        dry_run["measurement_executed"] = json!(false);
        dry_run["http_requests_executed"] = json!(false);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&dry_run).expect("dry-run receipt json"),
        )
        .expect("write dry-run receipt");
        let dry_run_reason = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            dry_run_reason["code"],
            "same-machine-performance-measurement-not-executed"
        );

        let mut missing_binary = current_same_machine_performance_receipt();
        missing_binary["dx_www_binary"]["sha256"] = json!(null);
        missing_binary["dx_www_binary"]["hash_status"] = json!("missing");
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&missing_binary).expect("missing binary receipt json"),
        )
        .expect("write missing binary receipt");
        let missing_binary_reason = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            missing_binary_reason["code"],
            "same-machine-performance-binary-hash-missing"
        );

        let mut failed_preflight = current_same_machine_performance_receipt();
        failed_preflight["output_fixtures"][1]["ok"] = json!(false);
        failed_preflight["output_fixtures"][1]["status"] = json!(500);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&failed_preflight).expect("failed preflight receipt json"),
        )
        .expect("write failed preflight receipt");
        let failed_preflight_reason =
            readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            failed_preflight_reason["code"],
            "same-machine-performance-preflight-failed"
        );
        assert_eq!(failed_preflight_reason["failed_targets"], json!(["next"]));

        let mut incomplete = current_same_machine_performance_receipt();
        incomplete["target_summaries"] = json!([same_machine_target_summary("www")]);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&incomplete).expect("incomplete receipt json"),
        )
        .expect("write incomplete receipt");
        let incomplete_reason = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            incomplete_reason["code"],
            "same-machine-performance-target-coverage-incomplete"
        );
        assert_eq!(
            incomplete_reason["missing_targets"],
            json!(["next", "svelte", "astro"])
        );

        let mut target_errors = current_same_machine_performance_receipt();
        target_errors["target_summaries"][2]["successful_round_count"] = json!(2);
        target_errors["target_summaries"][2]["errors_total"] = json!(1);
        target_errors["target_summaries"][2]["errors"] = json!(1);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&target_errors).expect("target error receipt json"),
        )
        .expect("write target error receipt");
        let target_error_reason = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            target_error_reason["code"],
            "same-machine-performance-target-errors"
        );
        assert_eq!(target_error_reason["failed_targets"], json!(["svelte"]));

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_same_machine_performance_receipt())
                .expect("current receipt json"),
        )
        .expect("write current receipt");
        let current_reason = readiness_same_machine_performance_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "same-machine-performance-paint-and-hosted-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["same_machine_performance_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["same_machine_performance_stale_reason"]["code"],
            "same-machine-performance-paint-and-hosted-proof-missing"
        );
    }

    #[test]
    fn readiness_imports_same_machine_performance_receipt_into_durable_contracts() {
        let project = tempfile::tempdir().expect("temp project");
        let source_path = project
            .path()
            .join(READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT);
        std::fs::create_dir_all(source_path.parent().expect("source parent"))
            .expect("source parent dir");
        std::fs::write(
            &source_path,
            serde_json::to_string_pretty(&current_same_machine_performance_receipt())
                .expect("source receipt json"),
        )
        .expect("write source receipt");

        let import = import_readiness_same_machine_performance_receipt(
            project.path(),
            Path::new(READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT),
        )
        .expect("import same-machine receipt");

        assert_eq!(import["id"], "same-machine-performance");
        assert_eq!(
            import["json_read_model_path"],
            READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT
        );
        assert_eq!(
            import["serializer_receipt_path"],
            READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR
        );
        assert_eq!(
            import["collection_receipt_path"],
            READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT
        );
        assert_eq!(import["release_ready"], false);
        assert_eq!(import["fastest_world_claim"], false);
        assert!(
            project
                .path()
                .join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR)
                .is_file()
        );
        assert!(
            project
                .path()
                .join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE)
                .is_file()
        );

        let canonical = readiness_same_machine_performance_receipt(project.path())
            .expect("canonical same-machine receipt");
        assert!(readiness_same_machine_performance_receipt_is_current(
            &canonical
        ));
        assert_eq!(
            canonical["imported_by"],
            "www readiness --import-same-machine-performance-receipt"
        );
        assert_eq!(
            canonical["collection_receipt_path"],
            READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT
        );
        assert_eq!(canonical["release_ready"], false);
        assert_eq!(canonical["fastest_world_claim"], false);
    }

    #[test]
    fn readiness_gate_status_reports_production_http_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project.path().join(READINESS_PRODUCTION_HTTP_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_production_http_local_replay_stale_reason(project.path());
        assert_eq!(
            missing["code"],
            "production-http-local-replay-receipt-missing"
        );

        let mut failed_check = current_production_http_receipt();
        failed_check["passed"] = json!(false);
        let checks = failed_check
            .get_mut("checks")
            .and_then(Value::as_array_mut)
            .expect("production checks");
        let range_check = checks
            .iter_mut()
            .find(|check| check.get("id").and_then(Value::as_str) == Some("range-206"))
            .expect("range check");
        range_check["passed"] = json!(false);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&failed_check).expect("failed receipt json"),
        )
        .expect("write failed receipt");
        let failed_reason = readiness_production_http_local_replay_stale_reason(project.path());
        assert_eq!(
            failed_reason["code"],
            "production-http-local-replay-checks-failed"
        );
        assert_eq!(failed_reason["missing_check_ids"], json!(["range-206"]));

        let mut overclaim = current_production_http_receipt();
        overclaim["browser_runtime_executed"] = json!(true);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&overclaim).expect("overclaim receipt json"),
        )
        .expect("write overclaim receipt");
        let overclaim_reason = readiness_production_http_local_replay_stale_reason(project.path());
        assert_eq!(
            overclaim_reason["code"],
            "production-http-local-replay-overclaims-proof-scope"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_production_http_receipt())
                .expect("current receipt json"),
        )
        .expect("write current receipt");
        let current_reason = readiness_production_http_local_replay_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "production-http-browser-tcp-cdn-provider-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["production_http_local_replay_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["production_http_stale_reason"]["code"],
            "production-http-browser-tcp-cdn-provider-proof-missing"
        );
        let production_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("production-http-preview")
                })
            })
            .expect("production HTTP gate");
        assert_eq!(
            production_gate["stale_reason"]["code"],
            "production-http-browser-tcp-cdn-provider-proof-missing"
        );

        let tcp_receipt_path = project
            .path()
            .join(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT);
        std::fs::write(
            &tcp_receipt_path,
            serde_json::to_string_pretty(&current_production_http_tcp_preview_receipt())
                .expect("current TCP preview receipt json"),
        )
        .expect("write TCP preview receipt");
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["production_http_tcp_preview_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["production_http_stale_reason"]["code"],
            "production-http-browser-cdn-provider-proof-missing"
        );
        assert!(
            !gate_status["local_replay_receipts"]["production_http_stale_reason"]
                ["remaining_external_proof_gap_ids"]
                .as_array()
                .expect("remaining gap ids")
                .iter()
                .any(|gap| gap.as_str() == Some("preview-tcp-server-parity"))
        );
    }

    #[test]
    fn readiness_gate_status_reports_server_action_replay_ledger_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project
            .path()
            .join(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_server_action_replay_ledger_stale_reason(project.path());
        assert_eq!(
            missing["code"],
            "server-action-replay-ledger-receipt-missing"
        );

        let mut boundary_invalid = current_server_action_replay_ledger_receipt();
        boundary_invalid["hosted_provider_proof"] = json!(true);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&boundary_invalid).expect("boundary receipt json"),
        )
        .expect("write boundary receipt");
        let boundary_reason = readiness_server_action_replay_ledger_stale_reason(project.path());
        assert_eq!(
            boundary_reason["code"],
            "server-action-replay-ledger-proof-boundary-invalid"
        );

        let mut not_current = current_server_action_replay_ledger_receipt();
        not_current["passed"] = json!(false);
        not_current["status"] = json!("missing-server-action-replay-ledger");
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&not_current).expect("not-current receipt json"),
        )
        .expect("write not-current receipt");
        let not_current_reason = readiness_server_action_replay_ledger_stale_reason(project.path());
        assert_eq!(
            not_current_reason["code"],
            "server-action-replay-ledger-status-not-current"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_server_action_replay_ledger_receipt())
                .expect("current receipt json"),
        )
        .expect("write current receipt");
        let current_reason = readiness_server_action_replay_ledger_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "server-action-provider-distributed-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["server_action_replay_ledger_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["server_action_replay_ledger_stale_reason"]["code"],
            "server-action-provider-distributed-proof-missing"
        );
        let route_action_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items.iter().find(|item| {
                    item.get("id").and_then(Value::as_str) == Some("route-action-runtime")
                })
            })
            .expect("route action gate");
        assert_eq!(
            route_action_gate["stale_reason"]["code"],
            "server-action-provider-distributed-proof-missing"
        );
    }

    fn current_server_action_replay_ledger_receipt() -> Value {
        json!({
            "schema": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "server-action-replay-ledger",
            "passed": true,
            "status": "local-replay-ledger-current-provider-proof-needed",
            "release_ready": false,
            "fastest_world_claim": false,
            "ledger_path": ".dx/www/output/.dx/build-cache/server-action-replay-ledger.json",
            "ledger_present": true,
            "ledger_schema": "dx.www.server_action.replay_ledger",
            "ledger_release_ready": false,
            "distributed": false,
            "provider_hosted": false,
            "hosted_provider_proof": false,
            "provider_proof_status": "not-run-local-preview-only",
            "production_proof_scope": "local-production-preview-only",
            "provider_hosted_replay_required": true,
            "provider_proof_gap_ids": READINESS_SERVER_ACTION_PROVIDER_GAP_IDS,
            "entry_count": 1,
            "conflict_count": 0,
            "duplicate_replay_count": 0,
            "privacy": "hash-only local production-preview ledger",
        })
    }

    fn current_production_http_receipt() -> Value {
        json!({
            "schema": READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "production-http-local-replay",
            "passed": true,
            "status": "local-production-http-wire-replay-current",
            "release_ready": false,
            "fastest_world_claim": false,
            "proof_scope": "local-production-contract-wire-replay",
            "wire_responder": "production_contract_wire_response",
            "tcp_preview_server_started": false,
            "browser_runtime_executed": false,
            "hosted_provider_proof": false,
            "provider_bound_cdn_executed": false,
            "external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS,
            "checks": readiness_production_http_expected_check_ids()
                .iter()
                .map(|check_id| json!({"id": check_id, "passed": true}))
                .collect::<Vec<_>>(),
        })
    }

    fn current_production_http_tcp_preview_receipt() -> Value {
        json!({
            "schema": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "production-http-tcp-preview",
            "collector": "dx-source-owned-production-preview-tcp-collector",
            "passed": true,
            "status": "local-production-http-tcp-preview-current",
            "release_ready": false,
            "fastest_world_claim": false,
            "proof_scope": "local-production-preview-tcp-server",
            "tcp_preview_server_started": true,
            "tcp_requests_executed": true,
            "browser_runtime_executed": false,
            "hosted_provider_proof": false,
            "provider_bound_cdn_executed": false,
            "cleared_external_proof_gap_ids": ["preview-tcp-server-parity"],
            "remaining_external_proof_gap_ids": production_http_external_gap_ids_without_tcp_preview(),
            "checks": readiness_production_http_expected_check_ids()
                .iter()
                .map(|check_id| json!({"id": check_id, "passed": true}))
                .collect::<Vec<_>>(),
        })
    }

    #[test]
    fn readiness_gate_status_reports_primitive_proof_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project.path().join(READINESS_PRIMITIVE_PROOF_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_primitive_proof_stale_reason(project.path());
        assert_eq!(missing["code"], "primitive-proof-receipt-missing");

        let workspace = tempfile::tempdir().expect("workspace");
        let repo = workspace.path().join("www");
        let app = repo.join("examples/template");
        std::fs::create_dir_all(repo.join(".dx/receipts/readiness")).expect("repo receipts");
        std::fs::create_dir_all(&app).expect("app dir");
        std::fs::write(
            repo.join(READINESS_PRIMITIVE_PROOF_RECEIPT),
            serde_json::to_string_pretty(&current_primitive_proof_receipt())
                .expect("root current primitive receipt"),
        )
        .expect("write root primitive receipt");
        let bridged_missing = readiness_primitive_proof_stale_reason(&app);
        assert_eq!(
            bridged_missing["code"],
            "primitive-proof-project-receipt-missing-root-current"
        );
        assert_eq!(
            bridged_missing["root_receipt_status"],
            "source-owned-primitive-foundation-current"
        );

        let mut incomplete = current_primitive_proof_receipt();
        incomplete["primitive_current_count"] = json!(3);
        let primitives = incomplete
            .get_mut("primitives")
            .and_then(Value::as_array_mut)
            .expect("primitive array");
        let wasm = primitives
            .iter_mut()
            .find(|primitive| primitive.get("id").and_then(Value::as_str) == Some("wasm"))
            .expect("wasm primitive");
        wasm["passed"] = json!(false);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&incomplete).expect("incomplete primitive receipt"),
        )
        .expect("write incomplete primitive receipt");
        let incomplete_reason = readiness_primitive_proof_stale_reason(project.path());
        assert_eq!(
            incomplete_reason["code"],
            "primitive-proof-source-coverage-incomplete"
        );
        assert_eq!(incomplete_reason["missing_primitives"], json!(["wasm"]));

        let mut overclaim = current_primitive_proof_receipt();
        overclaim["hosted_provider_proof"] = json!(true);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&overclaim).expect("overclaim primitive receipt"),
        )
        .expect("write overclaim primitive receipt");
        let overclaim_reason = readiness_primitive_proof_stale_reason(project.path());
        assert_eq!(
            overclaim_reason["code"],
            "primitive-proof-overclaims-hosted-or-browser-scope"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_primitive_proof_receipt())
                .expect("current primitive receipt"),
        )
        .expect("write current primitive receipt");
        let current_reason = readiness_primitive_proof_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "primitive-hosted-browser-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["primitive_proof_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["primitive_proof_stale_reason"]["code"],
            "primitive-hosted-browser-proof-missing"
        );
        let primitive_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("primitive-proof"))
            })
            .expect("primitive gate");
        assert_eq!(
            primitive_gate["stale_reason"]["code"],
            "primitive-hosted-browser-proof-missing"
        );
    }

    fn current_primitive_proof_receipt() -> Value {
        let primitives = ["image", "font", "script", "wasm"]
            .iter()
            .map(|id| json!({"id": id, "passed": true, "source_owned": true}))
            .collect::<Vec<_>>();
        json!({
            "schema": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "primitive-proof",
            "primitive_proof_schema": READINESS_PRIMITIVE_PROOF_SCHEMA,
            "passed": true,
            "status": "source-owned-primitive-foundation-current",
            "source_owned": true,
            "primitive_count": 4,
            "primitive_current_count": 4,
            "release_ready": false,
            "fastest_world_claim": false,
            "browser_runtime_executed": false,
            "hosted_provider_proof": false,
            "live_browser_executed": false,
            "proof_scope": "local-source-owned-primitive-foundation",
            "primitives": primitives,
        })
    }

    #[test]
    fn readiness_gate_status_reports_island_abi_stale_reason() {
        let project = tempfile::tempdir().expect("temp project");
        let receipt_path = project.path().join(READINESS_ISLAND_ABI_RECEIPT);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");

        let missing = readiness_island_abi_stale_reason(project.path());
        assert_eq!(missing["code"], "island-abi-receipt-missing");

        let mut incomplete = current_island_abi_receipt();
        incomplete["source_check_current_count"] = json!(4);
        incomplete["directives"] = json!(["clientLoad", "clientVisible", "clientIdle"]);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&incomplete).expect("incomplete island receipt"),
        )
        .expect("write incomplete island receipt");
        let incomplete_reason = readiness_island_abi_stale_reason(project.path());
        assert_eq!(
            incomplete_reason["code"],
            "island-abi-source-coverage-incomplete"
        );
        assert_eq!(
            incomplete_reason["missing_directives"],
            json!(["clientOnly"])
        );

        let mut overclaim = current_island_abi_receipt();
        overclaim["provider_adapter_executed"] = json!(true);
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&overclaim).expect("overclaim island receipt"),
        )
        .expect("write overclaim island receipt");
        let overclaim_reason = readiness_island_abi_stale_reason(project.path());
        assert_eq!(
            overclaim_reason["code"],
            "island-abi-overclaims-runtime-or-adapter-proof"
        );

        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&current_island_abi_receipt())
                .expect("current island receipt"),
        )
        .expect("write current island receipt");
        let current_reason = readiness_island_abi_stale_reason(project.path());
        assert_eq!(
            current_reason["code"],
            "island-abi-browser-adapter-proof-missing"
        );
        let gate_status = readiness_gate_status_for_project(Some(project.path()));
        assert_eq!(
            gate_status["local_replay_receipts"]["island_abi_receipt_current"],
            true
        );
        assert_eq!(
            gate_status["local_replay_receipts"]["island_abi_stale_reason"]["code"],
            "island-abi-browser-adapter-proof-missing"
        );
        let island_gate = gate_status["gate_summary"]
            .as_array()
            .and_then(|items| {
                items
                    .iter()
                    .find(|item| item.get("id").and_then(Value::as_str) == Some("islands"))
            })
            .expect("island gate");
        assert_eq!(
            island_gate["stale_reason"]["code"],
            "island-abi-browser-adapter-proof-missing"
        );
    }

    fn current_island_abi_receipt() -> Value {
        json!({
            "schema": READINESS_ISLAND_ABI_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "id": "islands",
            "passed": true,
            "status": "source-owned-island-abi-foundation-current",
            "source_owned": true,
            "source_owned_runtime": true,
            "directive_style_id": "camelCase-jsx-props",
            "directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly"],
            "core_directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly"],
            "supported_directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly", "clientMedia", "clientInteraction"],
            "unsupported_directive_syntax": ["client:load", "client:visible", "client:idle", "client:only"],
            "no_js_fallback_required": true,
            "node_modules_required": false,
            "full_react_hydration": false,
            "browser_runtime_executed": false,
            "hosted_provider_proof": false,
            "provider_adapter_executed": false,
            "release_ready": false,
            "fastest_world_claim": false,
            "route_unit_proof_metadata": "DxRouteReceipt.client_island_abi",
            "route_streaming_island_metadata": ["directive_style_id", "no_js_fallback_required"],
            "source_check_count": 5,
            "source_check_current_count": 5,
            "proof_scope": "local-source-owned-island-abi-foundation",
        })
    }

    fn current_native_event_browser_receipt() -> Value {
        json!({
            "schema": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "passed": true,
            "browser_runtime_executed": true,
            "binder_global_present": true,
            "unsupported_listener_attached": false,
            "supported_event_count": native_dom_event_names().len(),
            "catalog_hash": native_dom_event_catalog_integrity().catalog_hash,
            "preview_event_count": 3,
            "state_dispatch_count": 3,
            "required_events": ["click", "pointermove", "input"],
            "listener_events": ["click", "pointermove", "input"],
            "missing_listener_events": [],
            "missing_contract_events": [],
            "missing_replay_events": [],
            "browser_event_constructors": {
                "click": "MouseEvent",
                "pointermove": "PointerEvent",
                "input": "InputEvent"
            },
            "browser_event_replay_results": [
                {"event": "click", "tag": "button", "previewed": true},
                {"event": "pointermove", "tag": "button", "previewed": true},
                {"event": "input", "tag": "input", "previewed": true}
            ],
            "browser_snapshot_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "proof_scope": "local-in-app-browser-native-event-binder-replay",
            "release_ready": false,
            "fastest_world_claim": false,
            "react_synthetic_events": false,
            "full_react_event_parity": false,
        })
    }

    fn current_state_runtime_browser_receipt() -> Value {
        json!({
            "schema": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "passed": true,
            "browser_runtime_executed": true,
            "runtime_global_present": true,
            "full_react_hook_runtime": false,
            "react_api_shim_executed": false,
            "state_reflection_event_count": 3,
            "derived_reflection_event_count": 2,
            "effect_scheduled_event_count": 2,
            "action_dispatch_count": 3,
            "api_methods": [
                "getSnapshot",
                "setSlot",
                "dispatch",
                "refreshDerivedSlots",
                "scheduleEffectsForState"
            ],
            "missing_api_methods": [],
            "slot_count": 3,
            "event_count": 3,
            "runtime_schema": "dx.tsx.stateGraphRuntime",
            "route": "/",
            "browser_snapshot_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "proof_scope": "local-in-app-browser-state-runtime-replay",
            "release_ready": false,
            "fastest_world_claim": false,
        })
    }

    fn current_visual_edit_browser_receipt() -> Value {
        json!({
            "schema": READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT,
            "schema_revision": 1,
            "passed": true,
            "browser_runtime_executed": true,
            "visual_replay_attempted": true,
            "visual_replay_status": "current",
            "visual_replay_reason": "source-owned-devtools-replay-completed",
            "devtools_global_present": true,
            "browser_workbench_replay": "current",
            "proof_scope": "local-in-app-browser-visual-edit-workbench-replay",
            "workbench_phases": [
                "inspect",
                "cascade",
                "preview",
                "apply",
                "undo",
                "receipt"
            ],
            "missing_workbench_phases": [],
            "inspected_element_present": true,
            "cascade_inspected": true,
            "preview_source_mutated": false,
            "apply_source_mutated": true,
            "undo_source_restored": true,
            "safe_local_source_target_known": true,
            "apply_receipt_written": true,
            "undo_receipt_written": true,
            "receipt_durability": "json-sr-machine-written",
            "page_url": "http://127.0.0.1:3000/",
            "user_agent": "ReadinessBrowserHarness/1.0",
            "viewport": {
                "width": 1280,
                "height": 720
            },
            "inspected_selector": "[data-dx-component=\"state-runtime-probe\"]",
            "inspected_element_fingerprint": "state-runtime-probe:section.state-runtime-probe",
            "style_property": "--ring",
            "style_value": "0 0% 84%",
            "computed_style_before": {
                "property": "--ring",
                "value": "0 0% 83%"
            },
            "computed_style_after_preview": {
                "property": "--ring",
                "value": "0 0% 84%"
            },
            "computed_style_after_undo": {
                "property": "--ring",
                "value": "0 0% 83%"
            },
            "browser_snapshot_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "source_target": {
                "relativePath": "examples/template/styles/theme.css",
                "kind": "css-custom-property",
                "range": {
                    "startByte": 381,
                    "endByte": 400,
                    "expectedText": "  --ring: 0 0% 83%;"
                }
            },
            "source_path": "examples/template/styles/theme.css",
            "source_root": ".",
            "release_ready": false,
            "fastest_world_claim": false,
        })
    }

    fn current_same_machine_performance_receipt() -> Value {
        json!({
            "schema": READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA,
            "generated_at": "2026-05-30T00:00:00.000Z",
            "receipt_id": "dx-www-same-machine-test",
            "dry_run": false,
            "measurement_executed": true,
            "http_requests_executed": true,
            "preflight_http_requests_executed": true,
            "measurement_http_requests_executed": true,
            "round_count": 3,
            "request_count": 240,
            "concurrency": 16,
            "benchmark": {
                "script_path": "benchmarks/dx-runtime-throughput-benchmark.ts",
                "script_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "round_count": 3,
                "measurement_executed": true,
                "http_requests_executed": true
            },
            "same_machine_replay_required_for_speed_claim": true,
            "faster_than_upstream_claimed": false,
            "no_claims": {
                "no_claim_framework_absolute_superiority": true
            },
            "dx_www_binary": {
                "path": "target/release/dx-www.exe",
                "exists": true,
                "sha256": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
                "bytes": 123456,
                "hash_status": "captured"
            },
            "output_fixtures": [
                same_machine_output_fixture("www"),
                same_machine_output_fixture("next"),
                same_machine_output_fixture("svelte"),
                same_machine_output_fixture("astro")
            ],
            "target_summaries": [
                same_machine_target_summary("www"),
                same_machine_target_summary("next"),
                same_machine_target_summary("svelte"),
                same_machine_target_summary("astro")
            ]
        })
    }

    fn same_machine_target_summary(name: &str) -> Value {
        json!({
            "name": name,
            "target_name": name,
            "round_count": 3,
            "successful_round_count": 3,
            "errors": 0,
            "errors_total": 0,
            "requests_per_second": {
                "median": 1200.0
            }
        })
    }

    fn same_machine_output_fixture(name: &str) -> Value {
        let url = match name {
            "www" => "http://127.0.0.1:42104/fair-counter",
            "next" => "http://127.0.0.1:42101/",
            "svelte" => "http://127.0.0.1:42102/",
            "astro" => "http://127.0.0.1:42103/",
            _ => "http://127.0.0.1:0/",
        };
        json!({
            "name": name,
            "target_name": name,
            "url": url,
            "status": 200,
            "ok": true,
            "bytes": 1024,
            "sha256": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
            "captured_from": "preflight"
        })
    }
}

pub(crate) fn readiness_deploy_contract(manifest_hash: &str) -> Value {
    json!({
        "schema": "dx.www.readiness.deploy_contract",
        "schema_revision": 1,
        "manifest_hash": manifest_hash,
        "target_score": READINESS_TARGET_SCORE,
        "current_honest_score": READINESS_CURRENT_HONEST_SCORE,
        "fastest_world_claim": false,
        "proof_graph_receipt": READINESS_PROOF_GRAPH_RECEIPT,
        "bundle_partition": readiness_bundle_partition(),
        "production_http": readiness_production_http(),
        "route_action_runtime": readiness_route_action_runtime(),
        "primitive_proof": readiness_primitive_proof(),
        "score_breakdown": readiness_score_breakdown(),
        "delivery_tiers": readiness_delivery_tiers(),
        "native_event_catalog": readiness_native_event_catalog(false),
        "route_handler_server_action_gaps": readiness_route_handler_server_action_gaps(false),
        "release_rule": "public_runtime_bundle must stay deployable without evidence_bundle bytes; evidence stays no-store and receipt-addressable",
    })
}

pub(crate) fn write_readiness_proof_graph_receipt(
    output_dir: &Path,
    manifest_hash: &str,
) -> std::io::Result<Value> {
    write_readiness_proof_graph_receipt_with_command(
        output_dir,
        manifest_hash,
        "dx build",
        "written-by-dx-build-not-release-proof",
        "build-output-deploy-adapter-proof-graph",
    )
}

fn write_readiness_proof_graph_receipt_with_command(
    output_dir: &Path,
    manifest_hash: &str,
    command: &'static str,
    receipt_freshness: &'static str,
    proof_scope: &'static str,
) -> std::io::Result<Value> {
    let same_machine_performance_stale_reason =
        readiness_same_machine_performance_stale_reason(output_dir);
    let no_js_artifact_stale_reason = readiness_no_js_artifact_stale_reason(output_dir);
    let lighthouse_paint_receipts_status = readiness_lighthouse_paint_receipts_status(output_dir);
    let lighthouse_paint_stale_reason = lighthouse_paint_receipts_status
        .get("stale_reason")
        .cloned()
        .unwrap_or_else(|| {
            json!({
                "code": "lighthouse-paint-receipts-missing",
                "message": "Canonical dev/static browser paint receipts are missing from the generated proof graph."
            })
        });
    let no_js_browser_stale_reason = readiness_no_js_browser_stale_reason(output_dir);
    let bundle_partition_stale_reason = readiness_bundle_partition_stale_reason(output_dir);
    let bundle_provider_replay_stale_reason =
        readiness_bundle_provider_replay_stale_reason(output_dir);
    let production_http_stale_reason =
        readiness_production_http_local_replay_stale_reason(output_dir);
    let production_http_tcp_preview_stale_reason =
        readiness_production_http_tcp_preview_stale_reason(output_dir);
    let server_action_replay_ledger_stale_reason =
        readiness_server_action_replay_ledger_stale_reason(output_dir);
    let route_handler_provider_stale_reason =
        readiness_route_handler_provider_stale_reason(output_dir);
    let primitive_proof_stale_reason = readiness_primitive_proof_stale_reason(output_dir);
    let native_event_catalog_stale_reason = readiness_native_event_catalog_stale_reason(output_dir);
    let island_abi_stale_reason = readiness_island_abi_stale_reason(output_dir);
    let island_browser_stale_reason = readiness_island_browser_stale_reason(output_dir);
    let reactivity_model_stale_reason = readiness_reactivity_model_stale_reason(output_dir);
    let native_event_browser_binder_stale_reason =
        readiness_native_event_browser_binder_stale_reason(output_dir);
    let state_runtime_browser_stale_reason =
        readiness_state_runtime_browser_stale_reason(output_dir);
    let visual_edit_stale_reason = readiness_visual_edit_proof_graph_stale_reason(output_dir);
    let docs_onboarding_stale_reason =
        readiness_docs_onboarding_proof_graph_stale_reason(output_dir);
    let stale_reason_codes = readiness_proof_graph_stale_reason_codes(
        &[
            (
                &same_machine_performance_stale_reason,
                "same-machine-performance-receipt-missing",
            ),
            (
                &no_js_artifact_stale_reason,
                "tiny-static-no-js-artifact-receipt-missing",
            ),
            (&no_js_browser_stale_reason, "no-js-browser-receipt-missing"),
            (
                &lighthouse_paint_stale_reason,
                "lighthouse-paint-receipts-missing",
            ),
            (
                &bundle_partition_stale_reason,
                "bundle-partition-receipt-missing",
            ),
            (
                &bundle_provider_replay_stale_reason,
                "bundle-provider-replay-receipt-missing",
            ),
            (
                &production_http_stale_reason,
                "production-http-local-replay-receipt-missing",
            ),
            (
                &production_http_tcp_preview_stale_reason,
                "production-http-tcp-preview-receipt-missing",
            ),
            (
                &server_action_replay_ledger_stale_reason,
                "server-action-replay-ledger-receipt-missing",
            ),
            (
                &route_handler_provider_stale_reason,
                "route-handler-provider-replay-receipt-missing",
            ),
            (
                &primitive_proof_stale_reason,
                "primitive-proof-receipt-missing",
            ),
            (
                &native_event_catalog_stale_reason,
                "native-event-catalog-receipt-missing",
            ),
            (&island_abi_stale_reason, "island-abi-receipt-missing"),
            (
                &island_browser_stale_reason,
                "island-browser-receipt-missing",
            ),
            (
                &reactivity_model_stale_reason,
                "reactivity-model-receipt-not-current",
            ),
            (
                &native_event_browser_binder_stale_reason,
                "native-event-browser-binder-receipt-missing",
            ),
            (
                &state_runtime_browser_stale_reason,
                "state-runtime-browser-receipt-missing",
            ),
            (
                &docs_onboarding_stale_reason,
                "docs-onboarding-receipt-not-current",
            ),
            (
                &visual_edit_stale_reason,
                "visual-edit-browser-workbench-replay-missing",
            ),
        ],
        &[
            "browser-tcp-cdn-hosted-proof-missing",
            "hosted-provider-proof-missing",
        ],
    );
    let fields = readiness_proof_graph_sr_fields_for_command(
        manifest_hash,
        command,
        receipt_freshness,
        proof_scope,
        &stale_reason_codes,
    );
    let artifact = write_sr_artifact(output_dir, READINESS_PROOF_GRAPH_RECEIPT, &fields)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    Ok(json!({
        "id": "proof-graph",
        "schema": READINESS_PROOF_GRAPH_SCHEMA,
        "schema_revision": 1,
        "path": READINESS_PROOF_GRAPH_RECEIPT,
        "machine_contract_path": READINESS_PROOF_GRAPH_RECEIPT_MACHINE,
        "format": "sr",
        "command": command,
        "proof_scope": proof_scope,
        "inputs": [
            ".dx/build-cache/manifest.json",
            ".dx/build-cache/deploy-adapter.json",
            "readiness_gate_status",
            "proof_nodes",
            "same-machine-performance",
            "tiny-static-no-js-artifact",
            "tiny-static-no-js-browser",
            "lighthouse-paint-receipts",
            "production-http-local-replay",
            "production-http-tcp-preview",
            "server-action-replay-ledger",
            "primitive-proof",
            "native-event-catalog",
            "native-event-browser-binder",
            "island-abi",
            "island-browser",
            "reactivity-model",
            "state-runtime-browser",
            "bundle-partition",
            "docs-onboarding-receipt",
            "docs-onboarding-doctor",
            "visual-edit-workbench-receipts",
            "proof_graph",
        ],
        "output_hashes": {
            "manifest_hash": manifest_hash,
            "serializer_source_blake3": file_blake3_hex(&artifact.source),
            "serializer_machine_blake3": file_blake3_hex(&artifact.machine),
        },
        "receipt_freshness": receipt_freshness,
        "stale_reasons": [
            same_machine_performance_stale_reason,
            no_js_artifact_stale_reason,
            no_js_browser_stale_reason,
            lighthouse_paint_stale_reason,
            bundle_partition_stale_reason,
            production_http_stale_reason,
            production_http_tcp_preview_stale_reason,
            server_action_replay_ledger_stale_reason,
            primitive_proof_stale_reason,
            native_event_catalog_stale_reason,
            island_abi_stale_reason,
            island_browser_stale_reason,
            native_event_browser_binder_stale_reason,
            state_runtime_browser_stale_reason,
            docs_onboarding_stale_reason,
            visual_edit_stale_reason
        ],
        "replay_commands": readiness_replay_commands(),
        "machine_path": relative_artifact_path(output_dir, &artifact.machine),
        "machine_path_within_root": artifact_path_within_root(output_dir, &artifact.machine),
        "serializer_machine_generated": artifact.machine.is_file(),
        "release_ready": false,
        "manifest_hash": manifest_hash,
    }))
}

fn readiness_proof_graph_stale_reason_codes(
    stale_reasons: &[(&Value, &'static str)],
    extra_codes: &[&'static str],
) -> Vec<String> {
    let mut codes = Vec::new();
    for (stale_reason, fallback) in stale_reasons {
        let code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or(fallback);
        push_unique_string(&mut codes, code);
    }
    for code in extra_codes {
        push_unique_string(&mut codes, code);
    }
    codes
}

fn push_unique_string(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

fn readiness_no_js_artifact_stale_reason(root: &Path) -> Value {
    let Some(receipt) = read_json_file(&root.join(READINESS_NO_JS_ARTIFACT_RECEIPT)) else {
        return json!({
            "code": "tiny-static-no-js-artifact-receipt-missing",
            "message": "The tiny-static/no-JS artifact receipt is missing; regenerate it before using artifact-only no-JS proof in the proof graph.",
            "expected_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT,
            "serializer_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT_SR,
            "machine_contract_path": READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts"
        });
    };

    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "tiny-static-no-js-artifact-schema-mismatch",
            "message": "The tiny-static/no-JS artifact receipt exists, but its schema does not match the current readiness contract.",
            "expected_schema": READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT,
            "actual_schema": receipt.get("schema").and_then(Value::as_str),
            "expected_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json"
        });
    }

    let artifact_current = receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str) == Some("artifact-current")
        && receipt
            .get("meaningful_html_without_js")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("script_tag_count").and_then(Value::as_u64) == Some(0)
        && receipt
            .get("public_js_artifact_count")
            .and_then(Value::as_u64)
            == Some(0)
        && receipt
            .get("route_unit_no_js_proof_current")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false);
    if !artifact_current {
        return json!({
            "code": "tiny-static-no-js-artifact-receipt-not-current",
            "message": "The tiny-static/no-JS artifact receipt is present but does not prove a current zero-JS artifact.",
            "status": receipt.get("status").and_then(Value::as_str),
            "html_path": receipt.get("html_path").and_then(Value::as_str),
            "script_tag_count": receipt.get("script_tag_count").and_then(Value::as_u64),
            "public_js_artifact_count": receipt.get("public_js_artifact_count").and_then(Value::as_u64),
            "route_unit_no_js_proof_current": receipt.get("route_unit_no_js_proof_current").and_then(Value::as_bool),
            "expected_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json"
        });
    }

    json!({
        "code": "tiny-static-no-js-artifact-current-browser-and-astro-proof-needed",
        "message": "The tiny-static/no-JS artifact receipt is current; JS-disabled browser proof, Lighthouse paint, Astro parity, and hosted/provider proof remain required before release readiness.",
        "expected_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT,
        "browser_receipt_path": READINESS_NO_JS_BROWSER_RECEIPT,
        "browser_import_command": "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
        "local_contract_test_command": "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts"
    })
}

fn readiness_native_event_catalog_stale_reason(root: &Path) -> Value {
    let Some(receipt) = read_json_file(&root.join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT)) else {
        return json!({
            "code": "native-event-catalog-receipt-missing",
            "message": "The native DOM event catalog receipt is missing; regenerate it before using MDN/catalog freshness in the proof graph.",
            "expected_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
            "serializer_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
            "machine_contract_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-foundation.test.ts"
        });
    };

    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "native-event-catalog-schema-mismatch",
            "message": "The native DOM event catalog receipt exists, but its schema does not match the current readiness contract.",
            "expected_schema": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT,
            "actual_schema": receipt.get("schema").and_then(Value::as_str),
            "expected_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json"
        });
    }

    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("compiler-catalog-valid-mdn-current")
    {
        return json!({
            "code": "native-event-catalog-receipt-not-current",
            "message": "The native DOM event catalog receipt is present but not current against the local MDN/browser-compat-data source.",
            "status": receipt.get("status").and_then(Value::as_str),
            "mdn_snapshot_status": receipt.get("mdn_snapshot_status").cloned().unwrap_or(Value::Null),
            "source_freshness": receipt.get("source_freshness").cloned().unwrap_or(Value::Null),
            "expected_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json"
        });
    }

    json!({
        "code": "native-event-catalog-current-browser-binder-proof-needed",
        "message": "The native DOM event catalog receipt is current; real browser binder receipts and hosted/browser breadth proof remain required before release readiness.",
        "expected_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "browser_binder_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        "browser_binder_import_command": "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
        "local_contract_test_command": "node --test benchmarks/dx-www-native-dom-event-binder-replay.test.ts"
    })
}

fn readiness_reactivity_model_stale_reason(root: &Path) -> Value {
    let receipt = readiness_reactivity_model_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_reactivity_model_receipt_is_current)
    {
        json!({
            "code": "reactivity-model-browser-proof-missing",
            "message": "A source-owned DX-native reactivity model receipt is current; real browser state/effect/action replay, unsupported React API diagnostics, and hosted breadth proof are still required before release readiness.",
            "expected_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT,
            "serializer_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT_SR,
            "machine_contract_path": READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE,
            "browser_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-reactivity-receipts.test.ts"
        })
    } else {
        json!({
            "code": "reactivity-model-receipt-not-current",
            "message": "The source-owned DX-native reactivity JSON, .sr, and .machine receipt set is missing or stale.",
            "expected_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT,
            "serializer_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT_SR,
            "machine_contract_path": READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-reactivity-receipts.test.ts"
        })
    }
}

fn readiness_docs_onboarding_proof_graph_stale_reason(root: &Path) -> Value {
    let receipt = readiness_docs_onboarding_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_docs_onboarding_receipt_is_current)
    {
        let cleanup_done = receipt
            .as_ref()
            .is_some_and(readiness_docs_onboarding_cleanup_done);
        if cleanup_done {
            if docs_doctor::docs_doctor_command_replay_receipt(root)
                .as_ref()
                .is_some_and(docs_doctor::docs_doctor_command_replay_receipt_is_current)
            {
                return json!({
                    "code": "docs-onboarding-public-browser-provider-proof-needed",
                    "message": "Source-owned docs/onboarding receipts, generated/archive cleanup, and docs-doctor command replay are current; compatibility-surface cleanup and public onboarding browser/provider proof are still required before release readiness."
                });
            }
            return json!({
                "code": "docs-onboarding-command-replay-proof-needed",
                "message": "Source-owned docs/onboarding receipts and generated/archive cleanup are current; live docs-doctor replay, compatibility-surface cleanup, and public onboarding proof are still required before release readiness."
            });
        }
        json!({
            "code": "docs-onboarding-generated-archived-warning-cleanup",
            "message": "Docs-doctor now scans generated package docs and archived plans as warning-only release-readiness claim surfaces; cleanup or ownership promotion remains a release-readiness gate."
        })
    } else {
        json!({
            "code": "docs-onboarding-receipt-not-current",
            "message": "The source-owned docs/onboarding JSON, .sr, and .machine receipt set is missing or stale."
        })
    }
}

fn readiness_docs_onboarding_cleanup_done(receipt: &Value) -> bool {
    receipt
        .get("generated_archived_warning_surfaces_clean")
        .and_then(Value::as_bool)
        == Some(true)
        || receipt
            .get("generated_archived_warning_surfaces_promoted")
            .and_then(Value::as_bool)
            == Some(true)
}

fn readiness_native_event_browser_binder_stale_reason(root: &Path) -> Value {
    let receipt = native_event_browser_binder_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(native_event_browser_binder_receipt_is_current)
    {
        json!({
            "code": "native-event-browser-binder-hosted-provider-proof-missing",
            "message": "A local browser native-event binder receipt is current; hosted/provider and multi-browser event breadth proof is still required before release readiness.",
            "expected_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            "import_command": "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        native_event_browser_binder_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "native-event-browser-binder-receipt-missing",
            "message": "Native-event browser binder proof is missing; local catalog and MDN freshness receipts are not real browser listener execution proof.",
            "expected_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            "import_command": "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full"
        })
    }
}

fn native_event_browser_binder_stale_reason_from_receipt(receipt: &Value) -> Value {
    let expected_catalog_hash = native_dom_event_catalog_integrity().catalog_hash;
    let supported_event_count = receipt.get("supported_event_count").and_then(Value::as_u64);
    let expected_event_count = native_dom_event_names().len() as u64;
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "native-event-browser-binder-schema-mismatch",
            "message": "Native-event browser binder receipt uses the wrong schema contract.",
            "expected_schema": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        return json!({
            "code": "native-event-browser-binder-not-passing",
            "message": "Native-event browser binder receipt exists, but it does not report a passing replay."
        });
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return json!({
            "code": "native-event-browser-runtime-not-executed",
            "message": "Native-event browser binder receipt exists, but it does not prove real browser runtime execution."
        });
    }
    if receipt
        .get("binder_global_present")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return json!({
            "code": "native-event-binder-global-missing",
            "message": "Native-event browser binder receipt exists, but the generated binder global was not observed."
        });
    }
    if receipt
        .get("unsupported_listener_attached")
        .and_then(Value::as_bool)
        != Some(false)
    {
        return json!({
            "code": "native-event-unsupported-listener-attached",
            "message": "Native-event browser binder receipt attached an unsupported listener, so the event catalog boundary is stale."
        });
    }
    if supported_event_count != Some(expected_event_count)
        || receipt.get("catalog_hash").and_then(Value::as_str)
            != Some(expected_catalog_hash.as_str())
    {
        return json!({
            "code": "native-event-catalog-drift",
            "message": "Native-event browser binder receipt was captured against a stale event catalog.",
            "supported_event_count": supported_event_count,
            "expected_event_count": expected_event_count,
            "catalog_hash": receipt.get("catalog_hash").and_then(Value::as_str),
            "expected_catalog_hash": expected_catalog_hash
        });
    }
    if receipt
        .get("preview_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
        || receipt
            .get("state_dispatch_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 3)
        || !json_string_array_contains(receipt.get("listener_events"), "click")
        || !json_string_array_contains(receipt.get("listener_events"), "pointermove")
        || !json_string_array_contains(receipt.get("listener_events"), "input")
    {
        return json!({
            "code": "native-event-browser-listener-coverage-incomplete",
            "message": "Native-event browser binder receipt exists, but it does not cover the required click, pointermove, input, preview, and state-dispatch replay evidence.",
            "preview_event_count": receipt.get("preview_event_count").and_then(Value::as_u64),
            "state_dispatch_count": receipt.get("state_dispatch_count").and_then(Value::as_u64),
            "listener_events": receipt.get("listener_events").cloned().unwrap_or(Value::Null)
        });
    }
    if !json_array_is_empty(receipt.get("missing_listener_events"))
        || !json_array_is_empty(receipt.get("missing_contract_events"))
        || !json_array_is_empty(receipt.get("missing_replay_events"))
        || !json_object_string_at(
            receipt.get("browser_event_constructors"),
            "click",
            "MouseEvent",
        )
        || !json_object_string_at(
            receipt.get("browser_event_constructors"),
            "pointermove",
            "PointerEvent",
        )
        || !json_object_string_at(
            receipt.get("browser_event_constructors"),
            "input",
            "InputEvent",
        )
        || !json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "click",
            "previewed",
            true,
        )
        || !json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "pointermove",
            "previewed",
            true,
        )
        || !json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "input",
            "previewed",
            true,
        )
        || !json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
    {
        return json!({
            "code": "native-event-browser-replay-detail-incomplete",
            "message": "Native-event browser binder receipt exists, but it is missing browser-shaped event constructors, previewed replay records, replay gap arrays, or a stable browser snapshot hash.",
            "missing_listener_events": receipt.get("missing_listener_events").cloned().unwrap_or(Value::Null),
            "missing_contract_events": receipt.get("missing_contract_events").cloned().unwrap_or(Value::Null),
            "missing_replay_events": receipt.get("missing_replay_events").cloned().unwrap_or(Value::Null),
            "browser_event_constructors": receipt.get("browser_event_constructors").cloned().unwrap_or(Value::Null),
            "browser_event_replay_results": receipt.get("browser_event_replay_results").cloned().unwrap_or(Value::Null),
            "browser_snapshot_hash": receipt.get("browser_snapshot_hash").and_then(Value::as_str)
        });
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-native-event-binder-replay")
    {
        return json!({
            "code": "native-event-browser-proof-scope-invalid",
            "message": "Native-event browser binder receipt exists, but its proof scope is not the local in-app browser binder replay contract.",
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("react_synthetic_events")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("full_react_event_parity")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return json!({
            "code": "native-event-browser-receipt-overclaims-release",
            "message": "Native-event browser binder receipt overclaims release, speed, or React event parity proof."
        });
    }
    json!({
        "code": "native-event-browser-binder-receipt-stale",
        "message": "Native-event browser binder receipt exists, but at least one required freshness field is stale or invalid."
    })
}

fn readiness_state_runtime_browser_stale_reason(root: &Path) -> Value {
    let receipt = state_runtime_browser_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(state_runtime_browser_receipt_is_current)
    {
        json!({
            "code": "state-runtime-browser-hosted-provider-proof-missing",
            "message": "A local browser state-runtime receipt is current; hosted/provider browser breadth and unsupported React API diagnostics are still required before release readiness.",
            "expected_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
            "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
            "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
            "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
            "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
            "browser_runtime_executed_by_readiness": false,
            "import_command": "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        state_runtime_browser_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "state-runtime-browser-receipt-missing",
            "message": "State-runtime browser proof is missing; source-owned reactivity receipts are not real browser state, derived, effect, and action replay proof.",
            "expected_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
            "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
            "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
            "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
            "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
            "browser_runtime_executed_by_readiness": false,
            "import_command": "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full"
        })
    }
}

fn state_runtime_browser_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "state-runtime-browser-schema-mismatch",
            "message": "State-runtime browser receipt uses the wrong schema contract.",
            "expected_schema": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        return json!({
            "code": "state-runtime-browser-not-passing",
            "message": "State-runtime browser receipt exists, but it does not report a passing replay."
        });
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return json!({
            "code": "state-runtime-browser-runtime-not-executed",
            "message": "State-runtime browser receipt exists, but it does not prove real browser runtime execution."
        });
    }
    if receipt
        .get("runtime_global_present")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return json!({
            "code": "state-runtime-global-missing",
            "message": "State-runtime browser receipt exists, but the DX-native runtime global was not observed."
        });
    }
    if receipt
        .get("full_react_hook_runtime")
        .and_then(Value::as_bool)
        != Some(false)
        || receipt
            .get("react_api_shim_executed")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return json!({
            "code": "state-runtime-react-overclaim-present",
            "message": "State-runtime browser receipt overclaims full React hook runtime or React API shim execution."
        });
    }
    if receipt
        .get("state_reflection_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
        || receipt
            .get("derived_reflection_event_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 2)
        || receipt
            .get("effect_scheduled_event_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 2)
        || receipt
            .get("action_dispatch_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 3)
        || !json_string_array_contains(receipt.get("api_methods"), "getSnapshot")
        || !json_string_array_contains(receipt.get("api_methods"), "setSlot")
        || !json_string_array_contains(receipt.get("api_methods"), "dispatch")
        || !json_string_array_contains(receipt.get("api_methods"), "refreshDerivedSlots")
        || !json_string_array_contains(receipt.get("api_methods"), "scheduleEffectsForState")
    {
        return json!({
            "code": "state-runtime-browser-operation-coverage-incomplete",
            "message": "State-runtime browser receipt exists, but it does not cover the required state, derived, effect, action, and API method replay evidence.",
            "state_reflection_event_count": receipt.get("state_reflection_event_count").and_then(Value::as_u64),
            "derived_reflection_event_count": receipt.get("derived_reflection_event_count").and_then(Value::as_u64),
            "effect_scheduled_event_count": receipt.get("effect_scheduled_event_count").and_then(Value::as_u64),
            "action_dispatch_count": receipt.get("action_dispatch_count").and_then(Value::as_u64),
            "api_methods": receipt.get("api_methods").cloned().unwrap_or(Value::Null)
        });
    }
    if !json_array_is_empty(receipt.get("missing_api_methods"))
        || receipt
            .get("slot_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 1)
        || receipt
            .get("event_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count < 1)
        || !json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
    {
        return json!({
            "code": "state-runtime-browser-replay-detail-incomplete",
            "message": "State-runtime browser receipt exists, but it is missing API gap details, route program counts, or a stable browser snapshot hash.",
            "missing_api_methods": receipt.get("missing_api_methods").cloned().unwrap_or(Value::Null),
            "slot_count": receipt.get("slot_count").and_then(Value::as_u64),
            "event_count": receipt.get("event_count").and_then(Value::as_u64),
            "browser_snapshot_hash": receipt.get("browser_snapshot_hash").and_then(Value::as_str)
        });
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-state-runtime-replay")
    {
        return json!({
            "code": "state-runtime-browser-proof-scope-invalid",
            "message": "State-runtime browser receipt exists, but its proof scope is not the local in-app browser state runtime replay contract.",
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
    {
        return json!({
            "code": "state-runtime-browser-receipt-overclaims-release",
            "message": "State-runtime browser receipt overclaims release or speed proof."
        });
    }
    json!({
        "code": "state-runtime-browser-receipt-stale",
        "message": "State-runtime browser receipt exists, but at least one required freshness field is stale or invalid."
    })
}

fn readiness_visual_edit_proof_graph_stale_reason(root: &Path) -> Value {
    let receipt = read_json_file(&root.join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT));
    if receipt.as_ref().is_some_and(|receipt| {
        visual_edit_browser_workbench_receipt_value_is_current(root, receipt)
    }) {
        json!({
            "code": "visual-edit-hosted-cross-route-proof-missing",
            "message": "A local browser visual-edit workbench receipt is current; hosted/provider and cross-route visual-edit replay proof is still required before release readiness."
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_visual_edit_browser_stale_reason_from_receipt(root, receipt)
    } else {
        json!({
            "code": "visual-edit-browser-workbench-replay-missing",
            "message": "Visual-edit browser workbench proof is still missing; local source receipts prove only loopback preview/apply/undo plumbing."
        })
    }
}

fn readiness_visual_edit_browser_stale_reason_from_receipt(
    project: &Path,
    receipt: &Value,
) -> Value {
    let visual_replay_status = receipt
        .get("visual_replay_status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if receipt
        .get("visual_replay_attempted")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return json!({
            "code": "visual-edit-browser-replay-attempt-missing",
            "message": "Visual-edit browser workbench proof exists, but it does not prove that the source-owned Devtools replay entrypoint was invoked.",
            "visual_replay_status": visual_replay_status
        });
    }
    if visual_replay_status != "current" {
        return json!({
            "code": "visual-edit-browser-replay-attempt-not-current",
            "message": "Visual-edit browser workbench proof exists, but the source-owned Devtools replay entrypoint did not complete as current.",
            "visual_replay_status": visual_replay_status
        });
    }
    if receipt
        .get("browser_workbench_replay")
        .and_then(Value::as_str)
        != Some("current")
    {
        return json!({
            "code": "visual-edit-browser-workbench-replay-missing",
            "message": "Visual-edit browser workbench proof exists, but the workbench replay payload is not current.",
            "visual_replay_status": visual_replay_status
        });
    }
    if !json_array_is_empty(receipt.get("missing_workbench_phases")) {
        return json!({
            "code": "visual-edit-browser-workbench-phase-coverage-incomplete",
            "message": "Visual-edit browser workbench proof exists, but it does not cover every required inspect, cascade, preview, apply, undo, and receipt phase.",
            "missing_workbench_phases": receipt.get("missing_workbench_phases").cloned().unwrap_or(Value::Null),
            "workbench_phases": receipt.get("workbench_phases").cloned().unwrap_or(Value::Null)
        });
    }
    if !visual_edit_source_target_is_current(project, receipt) {
        return json!({
            "code": "visual-edit-source-target-not-replayable",
            "message": "Visual-edit browser workbench proof exists, but its source target path/range/expected text is not replayable against the current project files.",
            "source_root": receipt.get("source_root").and_then(Value::as_str),
            "source_target": receipt.get("source_target").cloned().unwrap_or(Value::Null)
        });
    }
    json!({
        "code": "visual-edit-browser-workbench-receipt-stale",
        "message": "Visual-edit browser workbench proof exists, but at least one required release-readiness receipt field is stale or invalid.",
        "visual_replay_status": visual_replay_status
    })
}

fn write_readiness_local_receipts(project: &Path) -> DxResult<Value> {
    let native_event_receipt = write_readiness_native_event_catalog_receipt(project)?;
    let no_js_artifact_receipt = write_readiness_no_js_artifact_receipt(project)?;
    let bundle_partition_receipt = write_readiness_bundle_partition_receipt(project)?;
    let production_http_receipt = write_readiness_production_http_local_replay_receipt(project)?;
    let primitive_proof_receipt = write_readiness_primitive_proof_receipt(project)?;
    let island_abi_receipt = write_readiness_island_abi_receipt(project)?;
    let reactivity_model_receipt = write_readiness_reactivity_model_receipt(project)?;
    let docs_onboarding_receipt = write_readiness_docs_onboarding_receipt(project)?;
    let proof_graph_receipt = write_readiness_local_proof_graph_receipt(project)?;
    let browser_binder_receipt = write_readiness_native_event_browser_binder_sr_receipt(project)?;
    let state_runtime_browser_sr_receipt =
        write_readiness_state_runtime_browser_sr_receipt(project)?;
    let island_browser_sr_receipt = write_readiness_island_browser_sr_receipt(project)?;
    let server_action_replay_ledger_receipt =
        write_readiness_server_action_replay_ledger_receipt(project)?;
    let mut receipts = vec![
        native_event_receipt,
        no_js_artifact_receipt,
        bundle_partition_receipt,
    ];
    receipts.push(production_http_receipt);
    receipts.push(server_action_replay_ledger_receipt);
    receipts.push(primitive_proof_receipt);
    receipts.push(island_abi_receipt);
    receipts.push(reactivity_model_receipt);
    receipts.push(docs_onboarding_receipt);
    receipts.push(proof_graph_receipt);
    if let Some(browser_binder_receipt) = browser_binder_receipt {
        receipts.push(browser_binder_receipt);
    }
    if let Some(state_runtime_browser_sr_receipt) = state_runtime_browser_sr_receipt {
        receipts.push(state_runtime_browser_sr_receipt);
    }
    if let Some(island_browser_sr_receipt) = island_browser_sr_receipt {
        receipts.push(island_browser_sr_receipt);
    }
    let written_count = receipts.len();
    let mut skipped = vec![json!({
        "id": "visual-edit-workbench-receipts",
        "reason": "requires real browser workbench replay receipts; dx www readiness does not invent visual-edit proof"
    })];
    if native_event_browser_binder_receipt(project).is_none() {
        skipped.push(json!({
            "id": "native-event-browser-binder-receipts",
            "reason": "requires a real browser replay receipt at .dx/receipts/readiness/native-event-browser-binder-latest.json; Node VM replay is not browser proof"
        }));
    }
    if state_runtime_browser_receipt(project).is_none() {
        skipped.push(json!({
            "id": "state-runtime-browser-receipts",
            "reason": "requires a real browser replay receipt at .dx/receipts/readiness/state-runtime-browser-latest.json; Node VM fake-DOM replay is not browser proof"
        }));
    }
    if readiness_island_browser_receipt(project).is_none() {
        skipped.push(json!({
            "id": "island-browser-receipts",
            "reason": "requires a real browser source-owned island replay receipt at .dx/receipts/readiness/island-browser-latest.json; source ABI markers are not browser execution proof"
        }));
    }
    Ok(json!({
        "schema": "dx.www.readiness.local_receipts",
        "schema_revision": 1,
        "written_count": written_count,
        "release_ready": false,
        "receipts": receipts,
        "skipped": skipped,
    }))
}

fn write_readiness_local_proof_graph_receipt(project: &Path) -> DxResult<Value> {
    let manifest_hash = readiness_local_build_manifest_hash(project);
    write_readiness_proof_graph_receipt_with_command(
        project,
        &manifest_hash,
        "dx www readiness --write-receipts",
        "written-by-dx-www-readiness-not-release-proof",
        "local-readiness-receipt-refresh-not-build-output-proof",
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_PROOF_GRAPH_RECEIPT)),
        message: error.to_string(),
    })
}

fn readiness_local_build_manifest_hash(project: &Path) -> String {
    [
        ".dx/www/output/.dx/build-cache/manifest.json",
        "examples/template/.dx/www/output/.dx/build-cache/manifest.json",
    ]
    .iter()
    .find_map(|relative| file_blake3_hex(&project.join(relative)))
    .unwrap_or_else(|| "missing-build-manifest".to_string())
}

fn readiness_bundle_partition_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_BUNDLE_PARTITION_RECEIPT))
}

fn readiness_bundle_partition_receipt_is_current(receipt: &Value) -> bool {
    receipt["schema"] == READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT
        && receipt["id"] == "bundle-partition"
        && receipt["passed"] == true
        && receipt["status"] == "local-public-evidence-partition-current"
        && receipt["release_ready"] == false
        && receipt["hosted_provider_proof"] == false
        && receipt["public_runtime_artifact_count"]
            .as_u64()
            .is_some_and(|count| count > 0)
        && receipt["evidence_artifact_count"]
            .as_u64()
            .is_some_and(|count| count > 0)
        && receipt["public_runtime_evidence_path_count"] == 0
        && receipt["evidence_artifacts_no_store"] == true
        && receipt["public_runtime_deployable"] == true
        && receipt["evidence_bundle_deployable_public_bytes"] == false
        && receipt["deploy_partition_present"] == true
        && receipt["provider_adapter_present"] == true
}

fn readiness_bundle_partition_stale_reason(root: &Path) -> Value {
    let receipt = readiness_bundle_partition_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_bundle_partition_receipt_is_current)
    {
        json!({
            "code": "bundle-partition-hosted-provider-proof-missing",
            "message": "A local public-runtime/evidence bundle partition receipt is current; hosted multi-provider upload replay is still required before production cleanliness can count toward release readiness.",
            "expected_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT,
            "serializer_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT_SR,
            "machine_contract_path": READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        bundle_partition_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "bundle-partition-receipt-missing",
            "message": "Public-runtime/evidence bundle partition proof is missing; production output cleanliness remains source-only until the receipt is regenerated.",
            "expected_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT,
            "serializer_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT_SR,
            "machine_contract_path": READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts"
        })
    }
}

fn bundle_partition_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "bundle-partition-schema-mismatch",
            "message": "Bundle partition receipt uses the wrong schema contract.",
            "expected_schema": READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return json!({
            "code": "bundle-partition-overclaims-hosted-proof",
            "message": "Bundle partition receipt overclaims release or hosted provider proof. Local deploy/provider adapter partition proof must stay separate from hosted upload replay.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool)
        });
    }
    let stale_fields = bundle_partition_stale_fields(receipt);
    if !stale_fields.is_empty() {
        return json!({
            "code": "bundle-partition-local-contract-incomplete",
            "message": "Bundle partition receipt exists, but public runtime and evidence bundle separation is incomplete or stale.",
            "stale_fields": stale_fields,
            "public_runtime_artifact_count": receipt.get("public_runtime_artifact_count").and_then(Value::as_u64),
            "evidence_artifact_count": receipt.get("evidence_artifact_count").and_then(Value::as_u64),
            "public_runtime_evidence_path_count": receipt.get("public_runtime_evidence_path_count").and_then(Value::as_u64),
            "precompressed_evidence_public_leak_count": receipt.get("precompressed_evidence_public_leak_count").and_then(Value::as_u64)
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("local-public-evidence-partition-current")
    {
        return json!({
            "code": "bundle-partition-status-not-current",
            "message": "Bundle partition receipt has complete local partition fields, but its top-level passed/status fields are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str)
        });
    }
    json!({
        "code": "bundle-partition-unknown-stale-state",
        "message": "Bundle partition receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT
    })
}

fn bundle_partition_stale_fields(receipt: &Value) -> Vec<&'static str> {
    let mut fields = Vec::new();
    if receipt
        .get("public_runtime_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        fields.push("public_runtime_artifact_count");
    }
    if receipt
        .get("evidence_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        fields.push("evidence_artifact_count");
    }
    if receipt
        .get("public_runtime_evidence_path_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        fields.push("public_runtime_evidence_path_count");
    }
    if receipt
        .get("precompressed_evidence_public_leak_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        fields.push("precompressed_evidence_public_leak_count");
    }
    for (field, expected) in [
        ("evidence_artifacts_no_store", true),
        ("precompressed_evidence_paths_no_store", true),
        ("public_runtime_deployable", true),
        ("deploy_partition_present", true),
        ("provider_adapter_present", true),
    ] {
        if receipt.get(field).and_then(Value::as_bool) != Some(expected) {
            fields.push(field);
        }
    }
    if receipt
        .get("evidence_bundle_deployable_public_bytes")
        .and_then(Value::as_bool)
        != Some(false)
    {
        fields.push("evidence_bundle_deployable_public_bytes");
    }
    fields
}

fn readiness_bundle_partition_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        (
            "schema",
            sr_string(READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT),
        ),
        ("id", sr_string("bundle-partition")),
        (
            "passed",
            sr_bool(receipt["passed"].as_bool().unwrap_or(false)),
        ),
        (
            "status",
            sr_string(receipt["status"].as_str().unwrap_or("unknown")),
        ),
        ("release_ready", sr_bool(false)),
        ("hosted_provider_proof", sr_bool(false)),
        (
            "deploy_adapter_path",
            sr_string(receipt["deploy_adapter_path"].as_str().unwrap_or("missing")),
        ),
        (
            "provider_adapter_path",
            sr_string(
                receipt["provider_adapter_path"]
                    .as_str()
                    .unwrap_or("missing"),
            ),
        ),
        (
            "public_runtime_artifact_count",
            sr_number(
                receipt["public_runtime_artifact_count"]
                    .as_u64()
                    .unwrap_or_default(),
            ),
        ),
        (
            "evidence_artifact_count",
            sr_number(
                receipt["evidence_artifact_count"]
                    .as_u64()
                    .unwrap_or_default(),
            ),
        ),
        (
            "public_runtime_evidence_path_count",
            sr_number(
                receipt["public_runtime_evidence_path_count"]
                    .as_u64()
                    .unwrap_or_default(),
            ),
        ),
        (
            "precompressed_evidence_artifact_count",
            sr_number(
                receipt["precompressed_evidence_artifact_count"]
                    .as_u64()
                    .unwrap_or_default(),
            ),
        ),
        (
            "precompressed_evidence_public_leak_count",
            sr_number(
                receipt["precompressed_evidence_public_leak_count"]
                    .as_u64()
                    .unwrap_or_default(),
            ),
        ),
        (
            "precompressed_evidence_paths_no_store",
            sr_bool(
                receipt["precompressed_evidence_paths_no_store"]
                    .as_bool()
                    .unwrap_or(false),
            ),
        ),
        (
            "evidence_artifacts_no_store",
            sr_bool(
                receipt["evidence_artifacts_no_store"]
                    .as_bool()
                    .unwrap_or(false),
            ),
        ),
        (
            "rule",
            sr_string(
                receipt["rule"]
                    .as_str()
                    .unwrap_or("local deploy/provider adapter partition proof only"),
            ),
        ),
    ]
}

fn readiness_route_handler_provider_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT))
}

fn readiness_route_handler_provider_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("hosted-route-handler-provider-replay-current")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("provider_replay_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("hosted_provider_requested")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("local_base_url").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("base_url")
            .and_then(Value::as_str)
            .is_some_and(readiness_route_handler_provider_base_url_is_hosted)
        && receipt.get("matrix_schema").and_then(Value::as_str)
            == Some("dx.www.deploy.route_handler_conformance_matrix")
        && receipt
            .get("case_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0
        && receipt
            .get("failed_case_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            == 0
}

fn readiness_route_handler_provider_stale_reason(root: &Path) -> Value {
    let receipt = readiness_route_handler_provider_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_route_handler_provider_receipt_is_current)
    {
        json!({
            "code": "route-handler-provider-replay-current-server-action-provider-proof-needed",
            "message": "A hosted provider route-handler replay receipt is current; distributed server-action replay, provider CSRF/session integration, cancellation, and multi-provider adapter proof remain required before release readiness.",
            "expected_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT,
            "serializer_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR,
            "machine_contract_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_MACHINE,
            "collect_command": READINESS_ROUTE_HANDLER_PROVIDER_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-route-handler-provider-receipt <route-handler-provider-receipt.json> --json --full",
            "remaining_provider_gap_ids": READINESS_SERVER_ACTION_PROVIDER_GAP_IDS
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_route_handler_provider_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "route-handler-provider-replay-receipt-missing",
            "message": "Hosted provider route-handler replay receipt is missing; run the route-handler provider replay collector against a real hosted URL, then import the JSON receipt.",
            "expected_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT,
            "serializer_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR,
            "machine_contract_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_MACHINE,
            "collect_command": READINESS_ROUTE_HANDLER_PROVIDER_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-route-handler-provider-receipt <route-handler-provider-receipt.json> --json --full"
        })
    }
}

fn readiness_route_handler_provider_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "route-handler-provider-replay-schema-mismatch",
            "message": "Route-handler provider replay receipt uses the wrong schema contract.",
            "expected_schema": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
    {
        return json!({
            "code": "route-handler-provider-replay-overclaims-release",
            "message": "Route-handler provider replay receipts must never claim release readiness or global speed leadership by themselves.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool)
        });
    }
    if receipt
        .get("provider_replay_executed")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt
            .get("hosted_provider_requested")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt.get("local_base_url").and_then(Value::as_bool) != Some(false)
        || !receipt
            .get("base_url")
            .and_then(Value::as_str)
            .is_some_and(readiness_route_handler_provider_base_url_is_hosted)
    {
        return json!({
            "code": "route-handler-provider-replay-not-hosted-proof",
            "message": "Route-handler provider replay receipt must be collected with --hosted-provider against a non-local http(s) URL before it can count as provider proof.",
            "provider_replay_executed": receipt.get("provider_replay_executed").and_then(Value::as_bool),
            "hosted_provider_requested": receipt.get("hosted_provider_requested").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "local_base_url": receipt.get("local_base_url").and_then(Value::as_bool),
            "base_url": receipt.get("base_url").and_then(Value::as_str)
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("hosted-route-handler-provider-replay-current")
        || receipt
            .get("failed_case_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            != 0
        || receipt
            .get("case_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            == 0
    {
        return json!({
            "code": "route-handler-provider-replay-status-not-current",
            "message": "Route-handler provider replay receipt exists, but its passed/status/case counters are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "case_count": receipt.get("case_count").and_then(Value::as_u64),
            "failed_case_count": receipt.get("failed_case_count").and_then(Value::as_u64)
        });
    }
    json!({
        "code": "route-handler-provider-replay-unknown-stale-state",
        "message": "Route-handler provider replay receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT
    })
}

fn readiness_route_handler_provider_base_url_is_hosted(base_url: &str) -> bool {
    let lower = base_url.trim().to_ascii_lowercase();
    let Some((scheme, rest)) = lower.split_once("://") else {
        return false;
    };
    if scheme != "http" && scheme != "https" {
        return false;
    }
    let authority = rest
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .rsplit('@')
        .next()
        .unwrap_or_default();
    let host = if let Some(stripped) = authority.strip_prefix('[') {
        stripped.split(']').next().unwrap_or_default().to_string()
    } else {
        authority.split(':').next().unwrap_or_default().to_string()
    };
    if host.is_empty()
        || host == "localhost"
        || host.ends_with(".localhost")
        || host == "0.0.0.0"
        || host == "::1"
        || host == "127.0.0.1"
        || host.starts_with("127.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
    {
        return false;
    }
    if let Some(second) = host
        .strip_prefix("172.")
        .and_then(|value| value.split('.').next())
        .and_then(|value| value.parse::<u8>().ok())
    {
        return !(16..=31).contains(&second);
    }
    true
}

fn readiness_route_handler_provider_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-route-handler-provider-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "provider_id",
            sr_string(
                receipt
                    .get("provider_id")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "provider_replay_executed",
            sr_bool(
                receipt
                    .get("provider_replay_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "hosted_provider_proof",
            sr_bool(
                receipt
                    .get("hosted_provider_proof")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "local_base_url",
            sr_bool(
                receipt
                    .get("local_base_url")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "base_url",
            sr_string(
                receipt
                    .get("base_url")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "matrix_schema",
            sr_string(
                receipt
                    .get("matrix_schema")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "case_count",
            sr_number(
                receipt
                    .get("case_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "failed_case_count",
            sr_number(
                receipt
                    .get("failed_case_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "rule",
            sr_string(
                "hosted route-handler provider replay only; release readiness still requires distributed server-action and multi-provider adapter proof",
            ),
        ),
    ]
}

fn import_readiness_route_handler_provider_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-route-handler-provider-receipt",
    )?;
    if !readiness_route_handler_provider_receipt_is_current(&receipt) {
        let stale_reason = readiness_route_handler_provider_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("route-handler-provider-replay-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported route-handler provider receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-route-handler-provider-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR,
        &readiness_route_handler_provider_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "import_source_path".to_string(),
            json!(readiness_import_source_path(project, &source_path)),
        );
        object.insert(
            "import_source_within_project".to_string(),
            json!(artifact_path_within_root(project, &source_path)),
        );
        object.insert(
            "imported_by".to_string(),
            json!("www readiness --import-route-handler-provider-receipt"),
        );
        object.insert(
            "import_rule".to_string(),
            json!("validated-hosted-provider-current-before-canonical-write"),
        );
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert("fastest_world_claim".to_string(), json!(false));
    }
    write_readiness_json_receipt(
        project,
        READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT,
        &receipt,
        "route-handler provider replay release readiness import receipt",
    )?;

    Ok(json!({
        "id": "route-handler-provider-replay",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT,
        "serializer_receipt_path": READINESS_ROUTE_HANDLER_PROVIDER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "hosted-route-handler-provider-replay-current",
        "release_ready": false,
        "fastest_world_claim": false,
        "hosted_provider_proof": true,
        "provider_replay_executed": true,
        "import_rule": "validated-hosted-provider-current-before-canonical-write",
    }))
}

fn readiness_bundle_provider_replay_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT))
}

fn readiness_bundle_provider_replay_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("bundle-provider-replay")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("hosted-public-evidence-bundle-replay-current")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("provider_replay_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("hosted_provider_requested")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("local_base_url").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("base_url")
            .and_then(Value::as_str)
            .is_some_and(readiness_route_handler_provider_base_url_is_hosted)
        && receipt
            .get("public_runtime_artifact_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0
        && receipt
            .get("evidence_artifact_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0
        && receipt
            .get("public_failure_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            == 0
        && receipt
            .get("evidence_public_leak_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            == 0
}

fn readiness_bundle_provider_replay_stale_reason(root: &Path) -> Value {
    let receipt = readiness_bundle_provider_replay_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_bundle_provider_replay_receipt_is_current)
    {
        json!({
            "code": "bundle-provider-replay-current-multi-provider-breadth-needed",
            "message": "A hosted provider public/evidence bundle replay receipt is current; multi-provider adapter breadth and CDN/object-store replay remain required before release readiness.",
            "expected_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
            "serializer_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
            "machine_contract_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
            "collect_command": READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_bundle_provider_replay_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "bundle-provider-replay-receipt-missing",
            "message": "Hosted provider public/evidence bundle replay receipt is missing; run the bundle replay collector against a real hosted URL, then import the JSON receipt.",
            "expected_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
            "serializer_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
            "machine_contract_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
            "collect_command": READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full"
        })
    }
}

fn readiness_bundle_provider_replay_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "bundle-provider-replay-schema-mismatch",
            "message": "Bundle provider replay receipt uses the wrong schema contract.",
            "expected_schema": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
    {
        return json!({
            "code": "bundle-provider-replay-overclaims-release",
            "message": "Bundle provider replay receipts must never claim release readiness or global speed leadership by themselves.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool)
        });
    }
    if receipt
        .get("provider_replay_executed")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt
            .get("hosted_provider_requested")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt.get("local_base_url").and_then(Value::as_bool) != Some(false)
        || !receipt
            .get("base_url")
            .and_then(Value::as_str)
            .is_some_and(readiness_route_handler_provider_base_url_is_hosted)
    {
        return json!({
            "code": "bundle-provider-replay-not-hosted-proof",
            "message": "Bundle provider replay receipt must be collected with --hosted-provider against a non-local http(s) URL before it can count as provider proof.",
            "provider_replay_executed": receipt.get("provider_replay_executed").and_then(Value::as_bool),
            "hosted_provider_requested": receipt.get("hosted_provider_requested").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "local_base_url": receipt.get("local_base_url").and_then(Value::as_bool),
            "base_url": receipt.get("base_url").and_then(Value::as_str)
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("hosted-public-evidence-bundle-replay-current")
        || receipt
            .get("public_runtime_artifact_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            == 0
        || receipt
            .get("evidence_artifact_count")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            == 0
        || receipt
            .get("public_failure_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            != 0
        || receipt
            .get("evidence_public_leak_count")
            .and_then(Value::as_u64)
            .unwrap_or(1)
            != 0
    {
        return json!({
            "code": "bundle-provider-replay-status-not-current",
            "message": "Bundle provider replay receipt exists, but its passed/status/artifact counters are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "public_runtime_artifact_count": receipt.get("public_runtime_artifact_count").and_then(Value::as_u64),
            "evidence_artifact_count": receipt.get("evidence_artifact_count").and_then(Value::as_u64),
            "public_failure_count": receipt.get("public_failure_count").and_then(Value::as_u64),
            "evidence_public_leak_count": receipt.get("evidence_public_leak_count").and_then(Value::as_u64)
        });
    }
    json!({
        "code": "bundle-provider-replay-unknown-stale-state",
        "message": "Bundle provider replay receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT
    })
}

fn readiness_bundle_provider_replay_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-bundle-provider-replay-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "provider_id",
            sr_string(
                receipt
                    .get("provider_id")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "hosted_provider_proof",
            sr_bool(
                receipt
                    .get("hosted_provider_proof")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "local_base_url",
            sr_bool(
                receipt
                    .get("local_base_url")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "base_url",
            sr_string(
                receipt
                    .get("base_url")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "public_runtime_artifact_count",
            sr_number(
                receipt
                    .get("public_runtime_artifact_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "evidence_artifact_count",
            sr_number(
                receipt
                    .get("evidence_artifact_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "public_failure_count",
            sr_number(
                receipt
                    .get("public_failure_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "evidence_public_leak_count",
            sr_number(
                receipt
                    .get("evidence_public_leak_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "rule",
            sr_string(
                "hosted public/evidence bundle replay only; release readiness still requires multi-provider adapter and CDN object-store proof",
            ),
        ),
    ]
}

fn import_readiness_bundle_provider_replay_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-bundle-provider-replay-receipt",
    )?;
    if !readiness_bundle_provider_replay_receipt_is_current(&receipt) {
        let stale_reason = readiness_bundle_provider_replay_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("bundle-provider-replay-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported bundle provider replay receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-bundle-provider-replay-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
        &readiness_bundle_provider_replay_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "import_source_path".to_string(),
            json!(readiness_import_source_path(project, &source_path)),
        );
        object.insert(
            "import_source_within_project".to_string(),
            json!(artifact_path_within_root(project, &source_path)),
        );
        object.insert(
            "imported_by".to_string(),
            json!("www readiness --import-bundle-provider-replay-receipt"),
        );
        object.insert(
            "import_rule".to_string(),
            json!("validated-hosted-bundle-provider-current-before-canonical-write"),
        );
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert("fastest_world_claim".to_string(), json!(false));
    }
    write_readiness_json_receipt(
        project,
        READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
        &receipt,
        "bundle provider replay release readiness import receipt",
    )?;

    Ok(json!({
        "id": "bundle-provider-replay",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
        "serializer_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "hosted-public-evidence-bundle-replay-current",
        "release_ready": false,
        "fastest_world_claim": false,
        "hosted_provider_proof": true,
        "provider_replay_executed": true,
        "import_rule": "validated-hosted-bundle-provider-current-before-canonical-write",
    }))
}

fn write_readiness_server_action_replay_ledger_receipt(project: &Path) -> DxResult<Value> {
    let ledger_path = readiness_server_action_replay_ledger_path(project);
    let ledger = read_json_file(&project.join(&ledger_path));
    let passed = ledger
        .as_ref()
        .is_some_and(server_action_replay_ledger_artifact_is_current);
    let status = ledger
        .as_ref()
        .map(server_action_replay_ledger_artifact_status)
        .unwrap_or("missing-server-action-replay-ledger");
    let receipt = json!({
        "schema": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "server-action-replay-ledger",
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "ledger_path": ledger_path,
        "ledger_present": ledger.is_some(),
        "ledger_schema": ledger.as_ref().and_then(|value| value.get("schema")).and_then(Value::as_str),
        "ledger_release_ready": ledger.as_ref().and_then(|value| value.get("release_ready")).and_then(Value::as_bool),
        "distributed": ledger.as_ref().and_then(|value| value.get("distributed")).and_then(Value::as_bool),
        "provider_hosted": ledger.as_ref().and_then(|value| value.get("provider_hosted")).and_then(Value::as_bool),
        "hosted_provider_proof": ledger.as_ref().and_then(|value| value.get("hosted_provider_proof")).and_then(Value::as_bool),
        "provider_proof_status": ledger.as_ref().and_then(|value| value.get("provider_proof_status")).and_then(Value::as_str),
        "production_proof_scope": ledger.as_ref().and_then(|value| value.get("production_proof_scope")).and_then(Value::as_str),
        "provider_hosted_replay_required": ledger.as_ref().and_then(|value| value.get("provider_hosted_replay_required")).and_then(Value::as_bool),
        "provider_proof_gap_ids": ledger.as_ref().and_then(|value| value.get("provider_proof_gap_ids")).cloned().unwrap_or(Value::Null),
        "entry_count": ledger.as_ref().and_then(|value| value.get("entry_count")).and_then(Value::as_u64),
        "conflict_count": ledger.as_ref().and_then(|value| value.get("conflict_count")).and_then(Value::as_u64),
        "duplicate_replay_count": ledger.as_ref().and_then(|value| value.get("duplicate_replay_count")).and_then(Value::as_u64),
        "privacy": ledger.as_ref().and_then(|value| value.get("privacy")).and_then(Value::as_str),
        "rule": "This release-readiness receipt validates the local production-preview server-action replay ledger only; distributed idempotency, hosted CSRF/session integration, and provider replay storage remain unproven.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
        &readiness_server_action_replay_ledger_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }

    let json_path = project.join(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json_text =
        serde_json::to_string_pretty(&receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!(
                "Failed to render server-action replay release readiness receipt: {error}"
            ),
            field: Some("www readiness".to_string()),
        })?;
    std::fs::write(&json_path, json_text).map_err(|error| DxError::IoError {
        path: Some(json_path.clone()),
        message: error.to_string(),
    })?;

    Ok(json!({
        "id": "server-action-replay-ledger",
        "json_read_model_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
        "serializer_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
    }))
}

fn readiness_server_action_replay_ledger_path(project: &Path) -> String {
    let root_output = ".dx/www/output/.dx/build-cache/server-action-replay-ledger.json";
    if project.join(root_output).is_file() {
        root_output.to_string()
    } else {
        "examples/template/.dx/www/output/.dx/build-cache/server-action-replay-ledger.json".to_string()
    }
}

fn server_action_replay_ledger_artifact_status(ledger: &Value) -> &'static str {
    if server_action_replay_ledger_artifact_is_current(ledger) {
        "local-replay-ledger-current-provider-proof-needed"
    } else if ledger.get("schema").and_then(Value::as_str)
        != Some("dx.www.server_action.replay_ledger")
    {
        "invalid-server-action-replay-ledger-schema"
    } else if ledger.get("release_ready").and_then(Value::as_bool) != Some(false) {
        "unsafe-server-action-ledger-release-claim"
    } else if ledger.get("hosted_provider_proof").and_then(Value::as_bool) != Some(false)
        || ledger.get("provider_proof_status").and_then(Value::as_str)
            != Some("not-run-local-preview-only")
        || ledger.get("production_proof_scope").and_then(Value::as_str)
            != Some("local-production-preview-only")
        || ledger
            .get("provider_hosted_replay_required")
            .and_then(Value::as_bool)
            != Some(true)
        || !server_action_replay_ledger_has_provider_gap_ids(ledger)
    {
        "missing-provider-proof-boundary"
    } else {
        "server-action-replay-ledger-not-current"
    }
}

fn server_action_replay_ledger_artifact_is_current(ledger: &Value) -> bool {
    ledger.get("schema").and_then(Value::as_str) == Some("dx.www.server_action.replay_ledger")
        && ledger.get("release_ready").and_then(Value::as_bool) == Some(false)
        && ledger.get("distributed").and_then(Value::as_bool) == Some(false)
        && ledger.get("provider_hosted").and_then(Value::as_bool) == Some(false)
        && ledger.get("hosted_provider_proof").and_then(Value::as_bool) == Some(false)
        && ledger.get("provider_proof_status").and_then(Value::as_str)
            == Some("not-run-local-preview-only")
        && ledger.get("production_proof_scope").and_then(Value::as_str)
            == Some("local-production-preview-only")
        && ledger
            .get("provider_hosted_replay_required")
            .and_then(Value::as_bool)
            == Some(true)
        && server_action_replay_ledger_has_provider_gap_ids(ledger)
        && ledger
            .get("privacy")
            .and_then(Value::as_str)
            .is_some_and(|privacy| privacy.contains("hash-only"))
        && ledger
            .get("rule")
            .and_then(Value::as_str)
            .is_some_and(|rule| rule.contains("local production-preview evidence only"))
}

fn server_action_replay_ledger_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT))
}

fn server_action_replay_ledger_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("local-replay-ledger-current-provider-proof-needed")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("provider_proof_status").and_then(Value::as_str)
            == Some("not-run-local-preview-only")
        && receipt
            .get("production_proof_scope")
            .and_then(Value::as_str)
            == Some("local-production-preview-only")
        && receipt
            .get("provider_hosted_replay_required")
            .and_then(Value::as_bool)
            == Some(true)
        && server_action_replay_ledger_has_provider_gap_ids(receipt)
}

fn server_action_replay_ledger_has_provider_gap_ids(value: &Value) -> bool {
    READINESS_SERVER_ACTION_PROVIDER_GAP_IDS
        .iter()
        .all(|gap_id| json_string_array_contains(value.get("provider_proof_gap_ids"), gap_id))
}

fn readiness_server_action_replay_ledger_stale_reason(root: &Path) -> Value {
    let receipt = server_action_replay_ledger_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(server_action_replay_ledger_receipt_is_current)
    {
        json!({
            "code": "server-action-provider-distributed-proof-missing",
            "message": "A local production-preview server-action replay ledger receipt is current; distributed idempotency, hosted CSRF/session integration, provider replay storage, cancellation, and provider-hosted route-handler breadth are still required before release readiness.",
            "expected_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
            "serializer_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
            "machine_contract_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/server-action-replay-ledger-honesty.test.ts",
            "remaining_provider_gap_ids": READINESS_SERVER_ACTION_PROVIDER_GAP_IDS
        })
    } else if let Some(receipt) = receipt.as_ref() {
        server_action_replay_ledger_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "server-action-replay-ledger-receipt-missing",
            "message": "Server-action replay ledger receipt is missing; route/action runtime remains foundation-only until the local production-preview ledger receipt is regenerated.",
            "expected_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
            "serializer_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
            "machine_contract_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/server-action-replay-ledger-honesty.test.ts"
        })
    }
}

fn server_action_replay_ledger_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "server-action-replay-ledger-schema-mismatch",
            "message": "Server-action replay ledger receipt uses the wrong schema contract.",
            "expected_schema": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt.get("provider_proof_status").and_then(Value::as_str)
            != Some("not-run-local-preview-only")
        || receipt
            .get("production_proof_scope")
            .and_then(Value::as_str)
            != Some("local-production-preview-only")
        || receipt
            .get("provider_hosted_replay_required")
            .and_then(Value::as_bool)
            != Some(true)
        || !server_action_replay_ledger_has_provider_gap_ids(receipt)
    {
        return json!({
            "code": "server-action-replay-ledger-proof-boundary-invalid",
            "message": "Server-action replay ledger receipt overclaims release/provider proof or does not preserve the local-production-preview proof boundary.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "provider_proof_status": receipt.get("provider_proof_status").and_then(Value::as_str),
            "production_proof_scope": receipt.get("production_proof_scope").and_then(Value::as_str),
            "provider_hosted_replay_required": receipt.get("provider_hosted_replay_required").and_then(Value::as_bool),
            "provider_proof_gap_ids": receipt.get("provider_proof_gap_ids").cloned().unwrap_or(Value::Null)
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("local-replay-ledger-current-provider-proof-needed")
    {
        return json!({
            "code": "server-action-replay-ledger-status-not-current",
            "message": "Server-action replay ledger receipt exists, but its top-level passed/status fields are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "ledger_path": receipt.get("ledger_path").and_then(Value::as_str),
            "ledger_present": receipt.get("ledger_present").and_then(Value::as_bool)
        });
    }
    json!({
        "code": "server-action-replay-ledger-unknown-stale-state",
        "message": "Server-action replay ledger receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT
    })
}

fn readiness_production_http_local_replay_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_PRODUCTION_HTTP_RECEIPT))
}

fn readiness_production_http_local_replay_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("local-production-http-wire-replay-current")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-production-contract-wire-replay")
        && receipt.get("wire_responder").and_then(Value::as_str)
            == Some("production_contract_wire_response")
        && receipt
            .get("tcp_preview_server_started")
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
        && receipt
            .get("provider_bound_cdn_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && production_http_local_replay_has_external_gap_ids(receipt)
        && readiness_production_http_expected_check_ids()
            .iter()
            .all(|check_id| readiness_receipt_check_passed(receipt, check_id))
}

fn readiness_production_http_local_replay_stale_reason(root: &Path) -> Value {
    let receipt = readiness_production_http_local_replay_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_production_http_local_replay_receipt_is_current)
    {
        json!({
            "code": "production-http-browser-tcp-cdn-provider-proof-missing",
            "message": "A production HTTP local wire replay receipt is current; Browser proof, TCP preview server proof, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof are still required before release readiness.",
            "expected_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT,
            "serializer_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT_SR,
            "machine_contract_path": READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
            "remaining_external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_production_http_local_replay_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "production-http-local-replay-receipt-missing",
            "message": "Production HTTP local wire replay proof is missing; ETag, 304, Range, method guard, and precompressed asset behavior are source-only until the receipt is regenerated.",
            "expected_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT,
            "serializer_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT_SR,
            "machine_contract_path": READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
            "remaining_external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS
        })
    }
}

fn readiness_production_http_stale_reason_for_gate(
    root: &Path,
    tcp_preview_current: bool,
) -> Value {
    let stale_reason = readiness_production_http_local_replay_stale_reason(root);
    if tcp_preview_current
        && stale_reason.get("code").and_then(Value::as_str)
            == Some("production-http-browser-tcp-cdn-provider-proof-missing")
    {
        return json!({
            "code": "production-http-browser-cdn-provider-proof-missing",
            "message": "Production HTTP local wire replay and TCP preview receipts are current; browser runtime, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof are still required before release readiness.",
            "expected_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT,
            "serializer_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT_SR,
            "machine_contract_path": READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
            "tcp_preview_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
            "tcp_preview_contract_test_command": "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
            "remaining_external_proof_gap_ids": production_http_external_gap_ids_without_tcp_preview()
        });
    }
    stale_reason
}

fn production_http_external_gap_ids_without_tcp_preview() -> Vec<&'static str> {
    READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS
        .iter()
        .copied()
        .filter(|gap_id| *gap_id != "preview-tcp-server-parity")
        .collect()
}

fn readiness_production_http_local_replay_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "production-http-local-replay-schema-mismatch",
            "message": "Production HTTP local replay receipt uses the wrong schema contract.",
            "expected_schema": READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("tcp_preview_server_started")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("provider_bound_cdn_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || !production_http_local_replay_has_external_gap_ids(receipt)
    {
        return json!({
            "code": "production-http-local-replay-overclaims-proof-scope",
            "message": "Production HTTP local replay receipt overclaims release, browser, TCP, CDN, or hosted-provider proof. Local wire replay must stay explicitly scoped until those proofs are separate receipts.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
            "tcp_preview_server_started": receipt.get("tcp_preview_server_started").and_then(Value::as_bool),
            "browser_runtime_executed": receipt.get("browser_runtime_executed").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "provider_bound_cdn_executed": receipt.get("provider_bound_cdn_executed").and_then(Value::as_bool),
            "external_proof_gap_ids": receipt.get("external_proof_gap_ids").cloned().unwrap_or(Value::Null),
            "remaining_external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS
        });
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-production-contract-wire-replay")
        || receipt.get("wire_responder").and_then(Value::as_str)
            != Some("production_contract_wire_response")
    {
        return json!({
            "code": "production-http-local-replay-provenance-invalid",
            "message": "Production HTTP local replay receipt exists, but it does not prove the canonical local production-contract wire responder.",
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str),
            "wire_responder": receipt.get("wire_responder").and_then(Value::as_str)
        });
    }
    let missing_check_ids = readiness_production_http_missing_check_ids(receipt);
    if !missing_check_ids.is_empty() {
        return json!({
            "code": "production-http-local-replay-checks-failed",
            "message": "Production HTTP local replay receipt exists, but one or more required ETag, 304, Range, method guard, or precompressed asset checks are missing or failing.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "missing_check_ids": missing_check_ids
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("local-production-http-wire-replay-current")
    {
        return json!({
            "code": "production-http-local-replay-status-not-current",
            "message": "Production HTTP local replay receipt has all expected check ids, but its top-level passed/status fields are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str)
        });
    }
    json!({
        "code": "production-http-local-replay-unknown-stale-state",
        "message": "Production HTTP local replay receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT
    })
}

fn readiness_production_http_missing_check_ids(receipt: &Value) -> Vec<&'static str> {
    readiness_production_http_expected_check_ids()
        .iter()
        .copied()
        .filter(|check_id| !readiness_receipt_check_passed(receipt, check_id))
        .collect()
}

fn production_http_local_replay_has_external_gap_ids(value: &Value) -> bool {
    READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS
        .iter()
        .all(|gap_id| json_string_array_contains(value.get("external_proof_gap_ids"), gap_id))
}

fn readiness_production_http_tcp_preview_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT))
}

fn readiness_production_http_tcp_preview_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("local-production-http-tcp-preview-current")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-production-preview-tcp-server")
        && receipt
            .get("tcp_preview_server_started")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("tcp_requests_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("provider_bound_cdn_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && json_string_array_contains(
            receipt.get("cleared_external_proof_gap_ids"),
            "preview-tcp-server-parity",
        )
        && !json_string_array_contains(
            receipt.get("remaining_external_proof_gap_ids"),
            "preview-tcp-server-parity",
        )
        && readiness_production_http_expected_check_ids()
            .iter()
            .all(|check_id| readiness_receipt_check_passed(receipt, check_id))
}

fn readiness_production_http_tcp_preview_stale_reason(root: &Path) -> Value {
    let receipt = readiness_production_http_tcp_preview_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_production_http_tcp_preview_receipt_is_current)
    {
        json!({
            "code": "production-http-tcp-preview-current-browser-cdn-provider-proof-missing",
            "message": "A production HTTP TCP preview receipt is current; browser runtime, live Axum/server transport parity, provider-bound CDN, and hosted-provider proof are still required before release readiness.",
            "expected_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
            "serializer_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
            "machine_contract_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE,
            "collect_command": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
            "remaining_external_proof_gap_ids": [
                "browser-js-enabled-runtime-replay",
                "browser-js-disabled-runtime-replay",
                "axum-static-responder-parity",
                "provider-bound-cdn-cache-replay",
                "hosted-provider-adapter-replay"
            ]
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_production_http_tcp_preview_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "production-http-tcp-preview-receipt-missing",
            "message": "Production HTTP TCP preview proof is missing; local wire replay does not prove the running dx preview TCP server.",
            "expected_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
            "serializer_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
            "machine_contract_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE,
            "collect_command": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
            "local_contract_test_command": "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts"
        })
    }
}

fn readiness_production_http_tcp_preview_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "production-http-tcp-preview-schema-mismatch",
            "message": "Production HTTP TCP preview receipt uses the wrong schema contract.",
            "expected_schema": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("provider_bound_cdn_executed")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return json!({
            "code": "production-http-tcp-preview-overclaims-proof-scope",
            "message": "Production HTTP TCP preview receipt overclaims release, browser, CDN, or hosted-provider proof. TCP preview proof must stay explicitly scoped.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
            "browser_runtime_executed": receipt.get("browser_runtime_executed").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "provider_bound_cdn_executed": receipt.get("provider_bound_cdn_executed").and_then(Value::as_bool)
        });
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-production-preview-tcp-server")
    {
        return json!({
            "code": "production-http-tcp-preview-provenance-invalid",
            "message": "Production HTTP TCP preview receipt exists, but it does not prove the canonical local production preview TCP server.",
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str)
        });
    }
    if receipt
        .get("tcp_preview_server_started")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt
            .get("tcp_requests_executed")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return json!({
            "code": "production-http-tcp-preview-not-live",
            "message": "Production HTTP TCP preview receipt exists, but it did not start the preview server and execute TCP requests.",
            "tcp_preview_server_started": receipt.get("tcp_preview_server_started").and_then(Value::as_bool),
            "tcp_requests_executed": receipt.get("tcp_requests_executed").and_then(Value::as_bool)
        });
    }
    let missing_check_ids = readiness_production_http_missing_check_ids(receipt);
    if !missing_check_ids.is_empty() {
        return json!({
            "code": "production-http-tcp-preview-checks-failed",
            "message": "Production HTTP TCP preview receipt exists, but one or more required ETag, 304, Range, method guard, or precompressed asset checks are missing or failing.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "missing_check_ids": missing_check_ids
        });
    }
    json!({
        "code": "production-http-tcp-preview-unknown-stale-state",
        "message": "Production HTTP TCP preview receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT
    })
}

fn readiness_production_http_expected_check_ids() -> &'static [&'static str] {
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

fn readiness_receipt_check_passed(receipt: &Value, expected_id: &str) -> bool {
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

fn write_readiness_production_http_local_replay_receipt(project: &Path) -> DxResult<Value> {
    let fixture_dir = project.join(".dx/receipts/readiness/production-http-local-replay-fixture");
    write_readiness_production_http_fixture(&fixture_dir)?;

    let first =
        readiness_preview_response_text(&fixture_dir, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let etag = readiness_response_header(first.as_deref().unwrap_or_default(), "ETag");
    let last_modified =
        readiness_response_header(first.as_deref().unwrap_or_default(), "Last-Modified");
    let conditional_etag = etag.as_ref().map(|etag| {
        readiness_preview_response_text(
            &fixture_dir,
            &format!("GET / HTTP/1.1\r\nHost: localhost\r\nIf-None-Match: {etag}\r\n\r\n"),
        )
    });
    let conditional_last_modified = last_modified.as_ref().map(|last_modified| {
        readiness_preview_response_text(
            &fixture_dir,
            &format!(
                "GET / HTTP/1.1\r\nHost: localhost\r\nIf-Modified-Since: {last_modified}\r\n\r\n"
            ),
        )
    });
    let head =
        readiness_preview_response_text(&fixture_dir, "HEAD / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let range = readiness_preview_response_text(
        &fixture_dir,
        "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\n\r\n",
    );
    let invalid_range = readiness_preview_response_text(
        &fixture_dir,
        "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=99-120\r\n\r\n",
    );
    let if_range = etag.as_ref().map(|etag| {
        readiness_preview_response_text(
            &fixture_dir,
            &format!(
                "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: {etag}\r\n\r\n"
            ),
        )
    });
    let stale_if_range = readiness_preview_response_text(
        &fixture_dir,
        "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: \"dx-stale\"\r\n\r\n",
    );
    let br_asset = readiness_preview_response_text(
        &fixture_dir,
        "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: gzip;q=0.5, br\r\n\r\n",
    );
    let gzip_asset = readiness_preview_response_text(
        &fixture_dir,
        "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: br;q=0, gzip;q=1\r\n\r\n",
    );
    let plain_asset = readiness_preview_response_text(
        &fixture_dir,
        "GET /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    let options_route = readiness_preview_response_text(
        &fixture_dir,
        "OPTIONS / HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    let post_asset = readiness_preview_response_text(
        &fixture_dir,
        "POST /chunks/app.mjs HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
    );
    let precompressed_decoded_content_type =
        [&br_asset, &gzip_asset, &plain_asset]
            .iter()
            .all(|response| {
                response.as_ref().is_ok_and(|text| {
                    text.contains("Content-Type: application/javascript; charset=utf-8")
                })
            });

    let checks = vec![
        json!({
            "id": "etag-present",
            "passed": etag.as_ref().is_some_and(|value| value.starts_with("\"dx-")),
        }),
        json!({
            "id": "if-none-match-304",
            "passed": match (conditional_etag.as_ref(), etag.as_ref()) {
                (Some(Ok(response)), Some(etag)) => response.contains("HTTP/1.1 304 Not Modified") && response.contains("Content-Length: 0") && response.contains(&format!("ETag: {etag}")),
                _ => false,
            },
        }),
        json!({
            "id": "if-modified-since-304",
            "passed": match (conditional_last_modified.as_ref(), last_modified.as_ref()) {
                (Some(Ok(response)), Some(last_modified)) => response.contains("HTTP/1.1 304 Not Modified") && response.contains("Content-Length: 0") && response.contains(&format!("Last-Modified: {last_modified}")),
                _ => false,
            },
        }),
        json!({
            "id": "head-omits-body",
            "passed": head
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 200 OK") && response.contains("Content-Length: 6") && response.ends_with("\r\n\r\n") && !response.contains("abcdef")),
        }),
        json!({
            "id": "range-206",
            "passed": range
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 206 Partial Content") && response.contains("Content-Range: bytes 1-3/6") && response.ends_with("\r\n\r\nbcd")),
        }),
        json!({
            "id": "range-416",
            "passed": invalid_range
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 416 Range Not Satisfiable") && response.contains("Content-Range: bytes */6")),
        }),
        json!({
            "id": "if-range-206",
            "passed": if_range
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .is_some_and(|response| response.contains("HTTP/1.1 206 Partial Content") && response.contains("Content-Range: bytes 1-3/6")),
        }),
        json!({
            "id": "stale-if-range-falls-back-to-full-body",
            "passed": stale_if_range
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 200 OK") && !response.contains("Content-Range:") && response.ends_with("\r\n\r\nabcdef")),
        }),
        json!({
            "id": "br-negotiation",
            "passed": br_asset
                .as_ref()
                .is_ok_and(|response| response.contains("Content-Encoding: br") && response.ends_with("\r\n\r\nbr-js")),
        }),
        json!({
            "id": "gzip-negotiation",
            "passed": gzip_asset
                .as_ref()
                .is_ok_and(|response| response.contains("Content-Encoding: gzip") && response.ends_with("\r\n\r\ngzip-js")),
        }),
        json!({
            "id": "plain-asset-vary",
            "passed": plain_asset
                .as_ref()
                .is_ok_and(|response| !response.contains("Content-Encoding:") && response.contains("Vary: Accept-Encoding") && response.ends_with("\r\n\r\nplain-js")),
        }),
        json!({
            "id": "static-options-204-allow-header",
            "passed": options_route
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 204 No Content") && response.contains("Allow: GET, HEAD, OPTIONS") && response.contains("Content-Length: 0")),
        }),
        json!({
            "id": "static-post-405-allow-header",
            "passed": post_asset
                .as_ref()
                .is_ok_and(|response| response.contains("HTTP/1.1 405 Method Not Allowed") && response.contains("Allow: GET, HEAD, OPTIONS") && response.contains(r#""error":"static-asset-method-not-allowed""#)),
        }),
        json!({
            "id": "precompressed-decoded-content-type",
            "passed": precompressed_decoded_content_type,
        }),
    ];
    let passed = checks
        .iter()
        .all(|check| check.get("passed").and_then(Value::as_bool) == Some(true));
    let status = if passed {
        "local-production-http-wire-replay-current"
    } else {
        "local-production-http-wire-replay-failed"
    };
    let mut receipt = json!({
        "schema": READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "production-http-local-replay",
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "fixture_path": relative_artifact_path(project, &fixture_dir),
        "fixture_within_project": artifact_path_within_root(project, &fixture_dir),
        "proof_scope": "local-production-contract-wire-replay",
        "wire_responder": "production_contract_wire_response",
        "tcp_preview_server_started": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "provider_bound_cdn_executed": false,
        "external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS,
        "checks": checks,
        "etag": etag,
        "last_modified": last_modified,
        "rule": "This receipt executes the local production-contract wire responder against a fixture deploy-adapter only; it is not Browser, TCP preview server, Axum, CDN, or hosted-provider proof.",
    });

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_PRODUCTION_HTTP_RECEIPT_SR,
        &readiness_production_http_local_replay_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_PRODUCTION_HTTP_RECEIPT_SR)),
        message: format!("{error:#}"),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_PRODUCTION_HTTP_RECEIPT,
        &receipt,
        "production HTTP local replay release readiness receipt",
    )?;

    Ok(json!({
        "id": "production-http-local-replay",
        "json_read_model_path": READINESS_PRODUCTION_HTTP_RECEIPT,
        "serializer_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "proof_scope": "local-production-contract-wire-replay",
        "hosted_provider_proof": false,
        "external_proof_gap_ids": READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS,
    }))
}

fn write_readiness_production_http_fixture(fixture_dir: &Path) -> DxResult<()> {
    std::fs::create_dir_all(fixture_dir.join("app")).map_err(|error| DxError::IoError {
        path: Some(fixture_dir.join("app")),
        message: error.to_string(),
    })?;
    std::fs::create_dir_all(fixture_dir.join("chunks")).map_err(|error| DxError::IoError {
        path: Some(fixture_dir.join("chunks")),
        message: error.to_string(),
    })?;
    write_readiness_fixture_bytes_if_changed(&fixture_dir.join("app/index.html"), b"abcdef")?;
    write_readiness_fixture_bytes_if_changed(&fixture_dir.join("chunks/app.mjs"), b"plain-js")?;
    write_readiness_fixture_bytes_if_changed(&fixture_dir.join("chunks/app.mjs.br"), b"br-js")?;
    write_readiness_fixture_bytes_if_changed(&fixture_dir.join("chunks/app.mjs.gz"), b"gzip-js")?;
    let contract = json!({
        "routes": [{"path": "/", "html": "app/index.html"}],
        "immutable_assets": [
            {"path": "chunks/app.mjs", "cache_control": "public, max-age=31536000, immutable"},
            {"path": "chunks/app.mjs.br", "cache_control": "public, max-age=31536000, immutable"},
            {"path": "chunks/app.mjs.gz", "cache_control": "public, max-age=31536000, immutable"}
        ],
        "server_actions": [],
        "health_checks": []
    });
    write_readiness_json_receipt(
        fixture_dir,
        ".dx/build-cache/deploy-adapter.json",
        &contract,
        "production HTTP local replay fixture deploy adapter",
    )
}

fn write_readiness_fixture_bytes_if_changed(path: &Path, bytes: &[u8]) -> DxResult<()> {
    if std::fs::read(path).is_ok_and(|existing| existing == bytes) {
        return Ok(());
    }
    std::fs::write(path, bytes).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn readiness_preview_response_text(build_dir: &Path, request: &str) -> Result<String, String> {
    String::from_utf8(preview_contract::production_contract_wire_response(
        build_dir,
        request,
        readiness_production_http_noop_server_action,
    ))
    .map_err(|error| format!("preview response was not UTF-8: {error}"))
}

fn readiness_production_http_noop_server_action(
    _build_dir: &Path,
    _contract: &Value,
    _request: &super::dev_http::DxCliHttpRequest,
    _action_id: &str,
) -> Result<String, String> {
    Err("release-readiness production HTTP fixture does not execute server actions".to_string())
}

fn readiness_response_header(response: &str, name: &str) -> Option<String> {
    let prefix = format!("{name}: ");
    response
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(str::to_string)
}

fn write_readiness_no_js_artifact_receipt(project: &Path) -> DxResult<Value> {
    let artifact_paths = readiness_no_js_artifact_paths(project);
    let html_path = project.join(&artifact_paths.html_relative);
    let html = std::fs::read_to_string(&html_path).ok();
    let artifact_html_blake3 = file_blake3_hex(&html_path).map(|hash| format!("blake3:{hash}"));
    let html_lower = html.as_deref().unwrap_or_default().to_ascii_lowercase();
    let script_tag_count = html_lower.matches("<script").count();
    let tiny_static_marker = html
        .as_deref()
        .is_some_and(|source| source.contains(r#"data-dx-output-mode="tiny-static""#));
    let no_js_marker = html
        .as_deref()
        .is_some_and(|source| source.contains(r#"data-dx-js="none""#));
    let main_present = html_lower.contains("<main");
    let heading_present =
        html_lower.contains("<h1") || html_lower.contains("<h2") || html_lower.contains("<h3");
    let semantic_landmark_present = html_semantic_landmark_present(&html_lower);
    let link_count = html_tag_count(&html_lower, "a");
    let form_count = html_tag_count(&html_lower, "form");
    let seo_title_present = html_lower.contains("<title")
        || html_lower.contains("property=\"og:title\"")
        || html_lower.contains("name=\"twitter:title\"");
    let accessibility_signal_count = html_accessibility_signal_count(&html_lower);
    let visible_text_present = html
        .as_deref()
        .map(strip_html_tags)
        .is_some_and(|text| !text.trim().is_empty());
    let packet_present = project.join(&artifact_paths.packet_relative).is_file();
    let public_js_artifacts =
        readiness_no_js_public_js_artifacts(project, &artifact_paths.html_relative);
    let public_js_artifact_count = public_js_artifacts.len();
    let route_unit = read_json_file(&project.join(&artifact_paths.route_unit_relative));
    let route_unit_tiny_static_proof = route_unit
        .as_ref()
        .and_then(|value| value.pointer("/runtime_report/tiny_static_route_proof"));
    let route_unit_no_js_capable = route_unit_tiny_static_proof
        .and_then(|value| value.get("no_js_capable"))
        .and_then(Value::as_bool);
    let route_unit_script_tag_count = route_unit_tiny_static_proof
        .and_then(|value| value.get("script_tag_count"))
        .and_then(Value::as_u64);
    let route_unit_runtime_required = route_unit_tiny_static_proof
        .and_then(|value| value.get("runtime_required"))
        .and_then(Value::as_bool);
    let route_unit_browser_api_required = route_unit_tiny_static_proof
        .and_then(|value| value.get("browser_api_required"))
        .and_then(Value::as_bool);
    let route_unit_semantic_landmark_present = route_unit_tiny_static_proof
        .and_then(|value| value.get("semantic_landmark_present"))
        .and_then(Value::as_bool);
    let route_unit_link_count = route_unit_tiny_static_proof
        .and_then(|value| value.get("link_count"))
        .and_then(Value::as_u64);
    let route_unit_form_count = route_unit_tiny_static_proof
        .and_then(|value| value.get("form_count"))
        .and_then(Value::as_u64);
    let route_unit_seo_title_present = route_unit_tiny_static_proof
        .and_then(|value| value.get("seo_title_present"))
        .and_then(Value::as_bool);
    let route_unit_accessibility_signal_count = route_unit_tiny_static_proof
        .and_then(|value| value.get("accessibility_signal_count"))
        .and_then(Value::as_u64);
    let route_unit_output_mode = route_unit_tiny_static_proof
        .and_then(|value| value.get("output_mode"))
        .and_then(Value::as_str);
    let route_unit_js = route_unit_tiny_static_proof
        .and_then(|value| value.get("js"))
        .and_then(Value::as_str);
    let route_unit_present = route_unit.is_some();
    let route_unit_no_js_capable_current = route_unit_no_js_capable == Some(true)
        && route_unit_script_tag_count == Some(0)
        && route_unit_runtime_required == Some(false)
        && route_unit_browser_api_required == Some(false)
        && route_unit_output_mode == Some("tiny-static")
        && route_unit_js == Some("none");
    let link_form_navigation_proof_current = link_count > 0 && form_count > 0;
    let route_unit_link_form_navigation_current = route_unit_link_count.unwrap_or_default() > 0
        && route_unit_form_count.unwrap_or_default() > 0;
    let html_css_links_forms_seo_accessibility_current = semantic_landmark_present
        && link_form_navigation_proof_current
        && seo_title_present
        && accessibility_signal_count > 0
        && route_unit_semantic_landmark_present == Some(true)
        && route_unit_link_form_navigation_current
        && route_unit_seo_title_present == Some(true)
        && route_unit_accessibility_signal_count.unwrap_or_default() > 0;
    let links_forms_seo_accessibility_fact_status =
        if html_css_links_forms_seo_accessibility_current {
            "artifact-facts-current-not-browser-proof"
        } else {
            "artifact-facts-incomplete-not-browser-proof"
        };
    let passed = html.is_some()
        && tiny_static_marker
        && no_js_marker
        && script_tag_count == 0
        && public_js_artifact_count == 0
        && main_present
        && visible_text_present
        && !packet_present
        && route_unit_present
        && route_unit_no_js_capable_current;
    let status = if passed {
        "artifact-current"
    } else if html.is_none() {
        "missing-output-html"
    } else if !route_unit_present {
        "missing-route-unit-proof"
    } else if !route_unit_no_js_capable_current {
        "route-unit-not-no-js-capable"
    } else if public_js_artifact_count > 0 {
        "public-js-artifact-present"
    } else if packet_present {
        "public-packet-present"
    } else {
        "artifact-not-proven"
    };
    let receipt = json!({
        "schema": READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "tiny-static-no-js-artifact",
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "artifact_root": artifact_paths.artifact_root,
        "artifact_source": artifact_paths.artifact_source,
        "artifact_path_resolution": "root-output-first-then-canonical-starter",
        "html_path": artifact_paths.html_relative,
        "artifact_html_blake3": artifact_html_blake3,
        "html_present": html.is_some(),
        "html_bytes": html.as_ref().map(|source| source.len()),
        "script_tag_count": script_tag_count,
        "data_dx_output_mode_tiny_static": tiny_static_marker,
        "data_dx_js_none": no_js_marker,
        "main_present": main_present,
        "heading_present": heading_present,
        "semantic_landmark_present": semantic_landmark_present,
        "link_count": link_count,
        "form_count": form_count,
        "seo_title_present": seo_title_present,
        "accessibility_signal_count": accessibility_signal_count,
        "link_form_navigation_proof_current": link_form_navigation_proof_current,
        "route_unit_link_form_navigation_current": route_unit_link_form_navigation_current,
        "html_css_links_forms_seo_accessibility_current": html_css_links_forms_seo_accessibility_current,
        "visible_text_present": visible_text_present,
        "packet_path": artifact_paths.packet_relative,
        "public_packet_present": packet_present,
        "public_js_artifact_count": public_js_artifact_count,
        "public_js_artifacts": public_js_artifacts,
        "route_unit_path": artifact_paths.route_unit_relative,
        "route_unit_present": route_unit_present,
        "route_unit_output_mode": route_unit_output_mode,
        "route_unit_js": route_unit_js,
        "route_unit_script_tag_count": route_unit_script_tag_count,
        "route_unit_runtime_required": route_unit_runtime_required,
        "route_unit_browser_api_required": route_unit_browser_api_required,
        "route_unit_semantic_landmark_present": route_unit_semantic_landmark_present,
        "route_unit_link_count": route_unit_link_count,
        "route_unit_form_count": route_unit_form_count,
        "route_unit_seo_title_present": route_unit_seo_title_present,
        "route_unit_accessibility_signal_count": route_unit_accessibility_signal_count,
        "route_unit_no_js_capable": route_unit_no_js_capable,
        "route_unit_no_js_proof_current": route_unit_no_js_capable_current,
        "meaningful_html_without_js": passed,
        "links_forms_seo_accessibility_fact_status": links_forms_seo_accessibility_fact_status,
        "live_browser_executed": false,
        "javascript_disabled_browser": false,
        "astro_parity_claimed": false,
        "live_astro_parity_receipt": "missing",
        "rule": "This receipt validates already-produced no-JS build artifacts only; it does not run a browser or claim Astro payload/paint/throughput parity.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_NO_JS_ARTIFACT_RECEIPT_SR,
        &readiness_no_js_artifact_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_NO_JS_ARTIFACT_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }

    let json_path = project.join(READINESS_NO_JS_ARTIFACT_RECEIPT);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json_text =
        serde_json::to_string_pretty(&receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to render no-JS release readiness receipt: {error}"),
            field: Some("www readiness".to_string()),
        })?;
    std::fs::write(&json_path, json_text).map_err(|error| DxError::IoError {
        path: Some(json_path.clone()),
        message: error.to_string(),
    })?;

    Ok(json!({
        "id": "tiny-static-no-js-artifact",
        "json_read_model_path": READINESS_NO_JS_ARTIFACT_RECEIPT,
        "serializer_receipt_path": READINESS_NO_JS_ARTIFACT_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "live_browser_executed": false,
        "javascript_disabled_browser": false,
        "astro_parity_claimed": false,
    }))
}

fn html_tag_count(html_lower: &str, tag_name: &str) -> usize {
    html_lower.matches(&format!("<{tag_name}")).count()
}

fn html_semantic_landmark_present(html_lower: &str) -> bool {
    [
        "<main", "<article", "<section", "<nav", "<header", "<footer",
    ]
    .iter()
    .any(|marker| html_lower.contains(marker))
}

fn html_accessibility_signal_count(html_lower: &str) -> usize {
    [
        "<main",
        "<h1",
        "<nav",
        " aria-label=",
        " aria-labelledby=",
        " alt=",
        " role=",
    ]
    .iter()
    .filter(|marker| html_lower.contains(**marker))
    .count()
}

fn readiness_no_js_public_js_artifacts(project: &Path, html_relative: &str) -> Vec<String> {
    let output_root = readiness_no_js_public_output_root(project, html_relative);
    if !output_root.is_dir() {
        return Vec::new();
    }
    let mut artifacts = walkdir::WalkDir::new(&output_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| readiness_no_js_public_js_artifact(project, &output_root, entry.path()))
        .collect::<Vec<_>>();
    artifacts.sort();
    artifacts
}

fn readiness_no_js_public_output_root(project: &Path, html_relative: &str) -> PathBuf {
    let normalized = html_relative.replace('\\', "/");
    if let Some(start) = normalized.find(".dx/www/output") {
        let end = start + ".dx/www/output".len();
        return project.join(&normalized[..end]);
    }
    Path::new(html_relative)
        .parent()
        .map(|parent| project.join(parent))
        .unwrap_or_else(|| project.to_path_buf())
}

fn readiness_no_js_public_js_artifact(
    project: &Path,
    output_root: &Path,
    path: &Path,
) -> Option<String> {
    let relative_to_output = path
        .strip_prefix(output_root)
        .ok()?
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    if readiness_bundle_evidence_only_path(&relative_to_output) {
        return None;
    }
    let decoded_path = readiness_decoded_precompressed_artifact_path(path);
    if !path_has_public_js_extension(&decoded_path) {
        return None;
    }
    Some(relative_artifact_path(project, path))
}

fn readiness_decoded_precompressed_artifact_path(path: &Path) -> PathBuf {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return path.to_path_buf();
    };
    if !matches!(extension.to_ascii_lowercase().as_str(), "br" | "gz") {
        return path.to_path_buf();
    }
    path.file_stem()
        .map(|file_stem| path.with_file_name(file_stem))
        .unwrap_or_else(|| path.to_path_buf())
}

fn path_has_public_js_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "js" | "mjs" | "cjs"
            )
        })
}

fn write_readiness_primitive_proof_receipt(project: &Path) -> DxResult<Value> {
    let source_root = readiness_source_owned_repo_root(project);
    let primitives = vec![
        readiness_primitive_source_check(
            &source_root,
            "image",
            &[
                (
                    "dx-www/src/cli/app_router_execution/source_render.rs",
                    &["next_image_component_names", "next-image-static-img"],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
                    &[
                        "apply_next_image_static_attributes",
                        "data-dx-framework-component",
                    ],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
                    &[
                        "data-dx-image-boundary",
                        "next-image-static-optimized-metadata",
                    ],
                ),
            ],
            &[
                "static-safe next/image lowers to <img>",
                "data-dx-framework-component and data-dx-image-boundary markers",
            ],
            &[
                "hosted optimizer service",
                "remote image loader parity",
                "responsive srcset generation for every loader",
            ],
        ),
        readiness_primitive_source_check(
            &source_root,
            "font",
            &[
                (
                    "dx-www/src/cli/app_router_execution/next_custom_transforms/font_loaders.rs",
                    &[
                        "collect_font_loader_detections",
                        "css_variable_receipt",
                        "generated_css_import",
                    ],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
                    &["apply_next_font_static_attributes"],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
                    &["data-dx-next-font", "next_font_binding_for_expression"],
                ),
            ],
            &[
                "module-scope font loader detection",
                "CSS variable/class metadata receipts",
                "static fallback font-family attributes without remote font requests",
            ],
            &[
                "font binary downloading",
                "full Next font manifest parity",
                "cross-provider hosted font cache proof",
            ],
        ),
        readiness_primitive_source_check(
            &source_root,
            "script",
            &[
                (
                    "dx-www/src/cli/app_router_execution/source_render.rs",
                    &["next_script_component_names", "next-script-static-script"],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
                    &["apply_next_script_static_attributes"],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
                    &[
                        "data-dx-next-script-strategy",
                        "next-script-static-script-metadata",
                    ],
                ),
            ],
            &[
                "static-safe next/script lowers to <script>",
                "strategy is converted to data-dx-next-script-strategy instead of invalid HTML",
                "afterInteractive/lazyOnload default to defer when async/defer is absent",
            ],
            &[
                "full Next script lifecycle ordering",
                "worker strategy runtime",
                "onReady/onLoad callback execution without a client runtime",
            ],
        ),
        readiness_primitive_source_check(
            &source_root,
            "wasm",
            &[
                (
                    "dx-www/src/cli/deploy_adapter_contract.rs",
                    &["chunks/app.wasm.gz", "content_encoding", "encoded_from"],
                ),
                (
                    "dx-www/src/cli/preview_contract.rs",
                    &["application/wasm", "production_contract_content_type"],
                ),
                (
                    "dx-www/src/cli/mod_parts/next_familiar_template.rs",
                    &["wasm/bindgen", "useWasmBindgenModule", "WasmBindgenFactory"],
                ),
            ],
            &[
                ".wasm and .wasm.gz are immutable runtime assets",
                "precompressed wasm metadata carries Content-Encoding, encoded_from, and decoded application/wasm content type",
                "wasm/bindgen source-guard receipts expose app-owned generated-Wasm boundaries",
            ],
            &[
                "automatic Rust-to-wasm app build pipeline",
                "generated wasm-bindgen glue execution for arbitrary apps",
                "browser proof for app-owned wasm modules",
            ],
        ),
    ];
    let primitive_count = primitives.len();
    let primitive_current_count = primitives
        .iter()
        .filter(|primitive| primitive.get("passed").and_then(Value::as_bool) == Some(true))
        .count();
    let passed = primitive_count == 4 && primitive_current_count == primitive_count;
    let status = if passed {
        "source-owned-primitive-foundation-current"
    } else {
        "source-owned-primitive-foundation-stale"
    };
    let receipt = json!({
        "schema": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "primitive-proof",
        "primitive_proof_schema": READINESS_PRIMITIVE_PROOF_SCHEMA,
        "passed": passed,
        "status": status,
        "source_root": source_root.display().to_string(),
        "source_owned": true,
        "primitive_count": primitive_count,
        "primitive_current_count": primitive_current_count,
        "primitives": primitives,
        "release_ready": false,
        "fastest_world_claim": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "live_browser_executed": false,
        "proof_scope": "local-source-owned-primitive-foundation",
        "next_proof": "hosted Image optimizer, hosted Font cache, Script lifecycle browser matrix, and app-owned Wasm browser execution receipts",
        "rule": "This receipt validates source-owned primitive lowering code paths only; it does not run a browser, execute generated Wasm, fetch remote fonts, optimize remote images, or claim hosted provider parity.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
        &readiness_primitive_proof_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_PRIMITIVE_PROOF_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_PRIMITIVE_PROOF_RECEIPT,
        &receipt,
        "primitive release readiness proof receipt",
    )?;

    Ok(json!({
        "id": "primitive-proof",
        "json_read_model_path": READINESS_PRIMITIVE_PROOF_RECEIPT,
        "serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
    }))
}

fn readiness_source_owned_repo_root(project: &Path) -> PathBuf {
    if project.join("dx-www/src/cli/readiness.rs").is_file() {
        return project.to_path_buf();
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

fn readiness_primitive_source_check(
    source_root: &Path,
    id: &'static str,
    file_markers: &[(&'static str, &[&'static str])],
    implemented: &[&'static str],
    not_yet_claimed: &[&'static str],
) -> Value {
    let file_checks = file_markers
        .iter()
        .map(|(relative_path, markers)| {
            let path = source_root.join(relative_path);
            let source = std::fs::read_to_string(&path).ok();
            let missing_markers = markers
                .iter()
                .filter(|marker| {
                    !source
                        .as_deref()
                        .is_some_and(|text| text.contains(**marker))
                })
                .copied()
                .collect::<Vec<_>>();
            json!({
                "path": relative_path,
                "present": source.is_some(),
                "required_markers": markers,
                "missing_markers": missing_markers,
                "passed": source.is_some() && missing_markers.is_empty(),
            })
        })
        .collect::<Vec<_>>();
    let passed = file_checks
        .iter()
        .all(|check| check.get("passed").and_then(Value::as_bool) == Some(true));
    json!({
        "id": id,
        "passed": passed,
        "source_owned": true,
        "implemented": implemented,
        "not_yet_claimed": not_yet_claimed,
        "file_checks": file_checks,
    })
}

fn readiness_primitive_proof_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_PRIMITIVE_PROOF_RECEIPT))
}

fn readiness_primitive_proof_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("primitive-proof")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-primitive-foundation-current")
        && receipt.get("source_owned").and_then(Value::as_bool) == Some(true)
        && receipt.get("primitive_count").and_then(Value::as_u64) == Some(4)
        && receipt
            .get("primitive_current_count")
            .and_then(Value::as_u64)
            == Some(4)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn readiness_primitive_proof_stale_reason(root: &Path) -> Value {
    let receipt = readiness_primitive_proof_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_primitive_proof_receipt_is_current)
    {
        json!({
            "code": "primitive-hosted-browser-proof-missing",
            "message": "A source-owned Image/Font/Script/Wasm primitive foundation receipt is current; hosted image optimization, hosted font cache, script lifecycle browser matrix, app-owned Wasm browser execution, and provider proof remain required before release readiness.",
            "expected_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT,
            "serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
            "machine_contract_path": READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        primitive_proof_stale_reason_from_receipt(receipt)
    } else if let Some(root_receipt) = readiness_parent_primitive_proof_receipt(root) {
        json!({
            "code": "primitive-proof-project-receipt-missing-root-current",
            "message": "This WWW project is missing its local primitive foundation receipt, but a current source-owned repo-root primitive proof exists. Regenerate project-local receipts before treating this app as current.",
            "expected_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT,
            "root_receipt_path": root_receipt.path.display().to_string(),
            "root_receipt_status": root_receipt.receipt.get("status").and_then(Value::as_str),
            "serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
            "machine_contract_path": READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts"
        })
    } else {
        json!({
            "code": "primitive-proof-receipt-missing",
            "message": "Primitive foundation proof is missing; Image, Font, Script, and Wasm claims remain source-only until the receipt is regenerated.",
            "expected_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT,
            "serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
            "machine_contract_path": READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts"
        })
    }
}

struct ParentPrimitiveProofReceipt {
    path: PathBuf,
    receipt: Value,
}

fn readiness_parent_primitive_proof_receipt(root: &Path) -> Option<ParentPrimitiveProofReceipt> {
    root.ancestors().skip(1).take(8).find_map(|parent| {
        let path = parent.join(READINESS_PRIMITIVE_PROOF_RECEIPT);
        let receipt = read_json_file(&path)?;
        readiness_primitive_proof_receipt_is_current(&receipt)
            .then_some(ParentPrimitiveProofReceipt { path, receipt })
    })
}

fn primitive_proof_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "primitive-proof-schema-mismatch",
            "message": "Primitive proof receipt uses the wrong schema contract.",
            "expected_schema": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("live_browser_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt.get("proof_scope").and_then(Value::as_str)
            != Some("local-source-owned-primitive-foundation")
    {
        return json!({
            "code": "primitive-proof-overclaims-hosted-or-browser-scope",
            "message": "Primitive proof receipt overclaims hosted, browser, or release proof. Source-owned primitive foundation receipts must keep those as separate gates.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
            "browser_runtime_executed": receipt.get("browser_runtime_executed").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "live_browser_executed": receipt.get("live_browser_executed").and_then(Value::as_bool),
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str)
        });
    }
    let missing_primitives = primitive_proof_missing_primitives(receipt);
    if !missing_primitives.is_empty()
        || receipt.get("primitive_count").and_then(Value::as_u64) != Some(4)
        || receipt
            .get("primitive_current_count")
            .and_then(Value::as_u64)
            != Some(4)
    {
        return json!({
            "code": "primitive-proof-source-coverage-incomplete",
            "message": "Primitive proof receipt exists, but Image, Font, Script, and Wasm source-owned coverage is incomplete or stale.",
            "primitive_count": receipt.get("primitive_count").and_then(Value::as_u64),
            "primitive_current_count": receipt.get("primitive_current_count").and_then(Value::as_u64),
            "missing_primitives": missing_primitives
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("source-owned-primitive-foundation-current")
    {
        return json!({
            "code": "primitive-proof-status-not-current",
            "message": "Primitive proof receipt has complete primitive coverage, but its top-level passed/status fields are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str)
        });
    }
    json!({
        "code": "primitive-proof-unknown-stale-state",
        "message": "Primitive proof receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT
    })
}

fn primitive_proof_missing_primitives(receipt: &Value) -> Vec<&'static str> {
    ["image", "font", "script", "wasm"]
        .iter()
        .copied()
        .filter(|expected_id| !primitive_proof_has_current_primitive(receipt, expected_id))
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

fn write_readiness_island_abi_receipt(project: &Path) -> DxResult<Value> {
    let source_root = readiness_source_owned_repo_root(project);
    let source_checks = vec![
        readiness_island_abi_source_check(
            &source_root,
            "compiler-capabilities",
            &[(
                "core/src/delivery/client_island.rs",
                &[
                    "dx.react.clientIsland.abi.capabilities",
                    "dx.react.clientIsland.abi",
                    "source_owned_runtime: true",
                    "node_modules_required: false",
                    "full_react_hydration: false",
                    "no_js_fallback_required: true",
                    "readiness_release_ready: false",
                    "browser_proof_status: \"foundation-not-release-proof\"",
                    "camelCase-jsx-props",
                    "clientLoad",
                    "clientVisible",
                    "clientIdle",
                    "clientOnly",
                    "client:load",
                    "client:visible",
                    "client:idle",
                    "client:only",
                    "framework-adapter-client-only",
                    "framework adapters are preview-only",
                ],
            )],
        ),
        readiness_island_abi_source_check(
            &source_root,
            "build-output-markers",
            &[(
                "dx-www/src/cli/app_router_build_output.rs",
                &[
                    "data-dx-client-island-bridge=\"source-owned\"",
                    "data-dx-client-island-abi=\"camelCase\"",
                    "data-dx-no-js-fallback=\"preserved\"",
                    "data-dx-client-only-adapters=\"preview-only\"",
                    "data-dx-client-media-support=\"recognized-not-executed\"",
                    "data-dx-client-interaction-support=\"recognized-not-executed\"",
                    "data-dx-browser-proof=\"not-claimed\"",
                    "data-dx-browser-runtime-proof=\"not-claimed\"",
                    "data-dx-provider-runtime-proof=\"not-claimed\"",
                    "data-dx-provider-adapters=\"not-executed\"",
                    "data-dx-island-hydration-strategy=",
                    "data-dx-island-directives=",
                    "data-dx-client-only-adapter=",
                    "data-dx-client-media=",
                    "data-dx-client-interaction=",
                ],
            )],
        ),
        readiness_island_abi_source_check(
            &source_root,
            "source-render-manifest",
            &[(
                "dx-www/src/cli/app_router_execution/source_render.rs",
                &[
                    "\"client_islands\": client_islands",
                    "Bind generated state/event runtime operations to the real rendered DOM for client islands.",
                    "full_jsx_execution\": false",
                    "Source-owned TSX render surface, not full JSX execution.",
                ],
            )],
        ),
        readiness_island_abi_source_check(
            &source_root,
            "client-component-runtime-boundary",
            &[(
                "dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs",
                &[
                    "\"node_modules_required\": false",
                    "\"full_react_hydration\": false",
                    "\"react_synthetic_events\": false",
                    "Does not execute arbitrary client components, effects, context providers, or full React hydration.",
                    "generated-client-islands-ready",
                ],
            )],
        ),
        readiness_island_abi_source_check(
            &source_root,
            "route-unit-no-js-proof",
            &[
                (
                    "core/src/delivery/app_route.rs",
                    &[
                        "DxReactClientIslandInput",
                        "compile_react_client_islands(DxReactClientIslandInput",
                        "react_client_island_abi_capabilities()",
                        "directive_style_id",
                        "hydration_strategy",
                        "browser_proof_status",
                        "framework_adapter",
                        "local-source-owned-island-abi-foundation",
                    ],
                ),
                (
                    "core/src/delivery/contract.rs",
                    &[
                        "DxRouteClientIslandAbiReceipt",
                        "client_island_abi",
                        "core_directives",
                        "browser_proof_status",
                        "fastest_world_claim",
                    ],
                ),
                (
                    "core/src/delivery/route_unit.rs",
                    &[
                        "let lowercase_html = fallback.html.to_ascii_lowercase();",
                        "let script_tag_count = lowercase_html.matches(\"<script\").count();",
                        "client_island_abi: route_client_island_abi_receipt(streaming)",
                        "fn route_client_island_abi_receipt",
                        "core_directives: vec![",
                        "\"clientLoad\".to_string()",
                        "\"clientVisible\".to_string()",
                        "\"clientIdle\".to_string()",
                        "\"clientOnly\".to_string()",
                        "browser_proof_status: \"foundation-not-release-proof\".to_string()",
                        "no_js_capable",
                        "runtime_required",
                        "browser_api_required",
                        "astro_parity_status: \"not_yet_claimed\"",
                        "source-only no-JS shell; Astro parity is not claimed by route-unit proof",
                    ],
                ),
            ],
        ),
    ];
    let source_check_count = source_checks.len();
    let source_check_current_count = source_checks
        .iter()
        .filter(|check| check.get("passed").and_then(Value::as_bool) == Some(true))
        .count();
    let passed = source_check_count == 5 && source_check_current_count == source_check_count;
    let status = if passed {
        "source-owned-island-abi-foundation-current"
    } else {
        "source-owned-island-abi-foundation-stale"
    };
    let receipt = json!({
        "schema": READINESS_ISLAND_ABI_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "islands",
        "island_abi_schema": "dx.www.readiness.island_abi",
        "compiler_abi_schema": "dx.react.clientIsland.abi",
        "compiler_capabilities_schema": "dx.react.clientIsland.abi.capabilities",
        "passed": passed,
        "status": status,
        "source_root": source_root.display().to_string(),
        "source_owned": true,
        "source_owned_runtime": true,
        "directive_style_id": "camelCase-jsx-props",
        "attribute_style": "camelCase",
        "directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly"],
        "core_directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly"],
        "supported_directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly", "clientMedia", "clientInteraction"],
        "additional_supported_directives": ["clientMedia", "clientInteraction"],
        "release_core_directives": ["clientLoad", "clientVisible", "clientIdle", "clientOnly"],
        "unsupported_directive_syntax": ["client:load", "client:visible", "client:idle", "client:only"],
        "no_js_fallback_required": true,
        "node_modules_required": false,
        "full_react_hydration": false,
        "react_synthetic_events": false,
        "framework_adapter_boundary": "source-owned client islands by default; explicit framework adapters only through clientOnly",
        "framework_adapter_policy": "clientOnly adapters are preview-only until executable framework adapter receipts exist",
        "route_unit_proof_metadata": "DxRouteReceipt.client_island_abi",
        "route_streaming_island_metadata": ["directive_style_id", "directives", "hydration_strategy", "no_js_fallback_required", "browser_proof_status", "framework_adapter"],
        "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "provider_adapter_executed": false,
        "live_browser_executed": false,
        "release_ready": false,
        "readiness_release_ready": false,
        "fastest_world_claim": false,
        "browser_proof_status": "foundation-not-release-proof",
        "proof_scope": "local-source-owned-island-abi-foundation",
        "source_check_count": source_check_count,
        "source_check_current_count": source_check_current_count,
        "source_checks": source_checks,
        "next_proof": "per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts",
        "rule": "This receipt validates the source-owned islands ABI foundation only; no browser/provider adapter execution is claimed by this source-owned islands ABI receipt.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_ISLAND_ABI_RECEIPT_SR,
        &readiness_island_abi_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_ISLAND_ABI_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_ISLAND_ABI_RECEIPT,
        &receipt,
        "island ABI release readiness proof receipt",
    )?;

    Ok(json!({
        "id": "islands",
        "json_read_model_path": READINESS_ISLAND_ABI_RECEIPT,
        "serializer_receipt_path": READINESS_ISLAND_ABI_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "provider_adapter_executed": false,
    }))
}

fn readiness_island_abi_source_check(
    source_root: &Path,
    id: &'static str,
    file_markers: &[(&'static str, &[&'static str])],
) -> Value {
    let file_checks = file_markers
        .iter()
        .map(|(relative_path, markers)| {
            let path = source_root.join(relative_path);
            let source = std::fs::read_to_string(&path).ok();
            let missing_markers = markers
                .iter()
                .filter(|marker| {
                    !source
                        .as_deref()
                        .is_some_and(|text| text.contains(**marker))
                })
                .copied()
                .collect::<Vec<_>>();
            json!({
                "path": relative_path,
                "present": source.is_some(),
                "required_markers": markers,
                "missing_markers": missing_markers,
                "passed": source.is_some() && missing_markers.is_empty(),
            })
        })
        .collect::<Vec<_>>();
    let passed = file_checks
        .iter()
        .all(|check| check.get("passed").and_then(Value::as_bool) == Some(true));
    json!({
        "id": id,
        "passed": passed,
        "source_owned": true,
        "file_checks": file_checks,
    })
}

fn readiness_island_abi_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_ISLAND_ABI_RECEIPT))
}

fn readiness_island_abi_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(READINESS_ISLAND_ABI_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("islands")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-island-abi-foundation-current")
        && receipt.get("source_owned").and_then(Value::as_bool) == Some(true)
        && receipt.get("directive_style_id").and_then(Value::as_str) == Some("camelCase-jsx-props")
        && receipt
            .get("no_js_fallback_required")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("node_modules_required")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("full_react_hydration").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("provider_adapter_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("route_unit_proof_metadata")
            .and_then(Value::as_str)
            == Some("DxRouteReceipt.client_island_abi")
        && json_string_array_contains(
            receipt.get("route_streaming_island_metadata"),
            "directive_style_id",
        )
        && json_string_array_contains(
            receipt.get("route_streaming_island_metadata"),
            "no_js_fallback_required",
        )
        && receipt.get("source_check_count").and_then(Value::as_u64) == Some(5)
        && receipt
            .get("source_check_current_count")
            .and_then(Value::as_u64)
            == Some(5)
}

fn readiness_island_abi_stale_reason(root: &Path) -> Value {
    let receipt = readiness_island_abi_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_island_abi_receipt_is_current)
    {
        json!({
            "code": "island-abi-browser-adapter-proof-missing",
            "message": "A source-owned camelCase islands ABI receipt is current; per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts remain required before release readiness.",
            "expected_receipt_path": READINESS_ISLAND_ABI_RECEIPT,
            "serializer_receipt_path": READINESS_ISLAND_ABI_RECEIPT_SR,
            "machine_contract_path": READINESS_ISLAND_ABI_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-islands-abi-camelcase.test.ts"
        })
    } else if let Some(receipt) = receipt.as_ref() {
        island_abi_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "island-abi-receipt-missing",
            "message": "Islands ABI proof is missing; camelCase clientLoad/clientVisible/clientIdle/clientOnly support remains source-only until the receipt is regenerated.",
            "expected_receipt_path": READINESS_ISLAND_ABI_RECEIPT,
            "serializer_receipt_path": READINESS_ISLAND_ABI_RECEIPT_SR,
            "machine_contract_path": READINESS_ISLAND_ABI_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts --json",
            "local_contract_test_command": "node --test benchmarks/dx-www-islands-abi-camelcase.test.ts"
        })
    }
}

fn island_abi_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str) != Some(READINESS_ISLAND_ABI_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "island-abi-schema-mismatch",
            "message": "Islands ABI receipt uses the wrong schema contract.",
            "expected_schema": READINESS_ISLAND_ABI_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("provider_adapter_executed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt.get("full_react_hydration").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("node_modules_required")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt.get("proof_scope").and_then(Value::as_str)
            != Some("local-source-owned-island-abi-foundation")
    {
        return json!({
            "code": "island-abi-overclaims-runtime-or-adapter-proof",
            "message": "Islands ABI receipt overclaims browser/runtime/provider adapter proof or drifts away from the source-owned local foundation boundary.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "browser_runtime_executed": receipt.get("browser_runtime_executed").and_then(Value::as_bool),
            "hosted_provider_proof": receipt.get("hosted_provider_proof").and_then(Value::as_bool),
            "provider_adapter_executed": receipt.get("provider_adapter_executed").and_then(Value::as_bool),
            "full_react_hydration": receipt.get("full_react_hydration").and_then(Value::as_bool),
            "node_modules_required": receipt.get("node_modules_required").and_then(Value::as_bool),
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str)
        });
    }
    let missing_directives = island_abi_missing_directives(receipt);
    let missing_unsupported_syntax = island_abi_missing_unsupported_syntax(receipt);
    if !missing_directives.is_empty()
        || !missing_unsupported_syntax.is_empty()
        || receipt.get("directive_style_id").and_then(Value::as_str) != Some("camelCase-jsx-props")
        || receipt
            .get("no_js_fallback_required")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt.get("source_check_count").and_then(Value::as_u64) != Some(5)
        || receipt
            .get("source_check_current_count")
            .and_then(Value::as_u64)
            != Some(5)
    {
        return json!({
            "code": "island-abi-source-coverage-incomplete",
            "message": "Islands ABI receipt exists, but directive coverage, unsupported syntax diagnostics, no-JS fallback metadata, or source checks are incomplete.",
            "directive_style_id": receipt.get("directive_style_id").and_then(Value::as_str),
            "no_js_fallback_required": receipt.get("no_js_fallback_required").and_then(Value::as_bool),
            "source_check_count": receipt.get("source_check_count").and_then(Value::as_u64),
            "source_check_current_count": receipt.get("source_check_current_count").and_then(Value::as_u64),
            "missing_directives": missing_directives,
            "missing_unsupported_syntax": missing_unsupported_syntax
        });
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("source-owned-island-abi-foundation-current")
    {
        return json!({
            "code": "island-abi-status-not-current",
            "message": "Islands ABI receipt has complete source coverage, but its top-level passed/status fields are not current.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str)
        });
    }
    json!({
        "code": "island-abi-unknown-stale-state",
        "message": "Islands ABI receipt did not satisfy the current contract, but no specific stale condition matched.",
        "expected_receipt_path": READINESS_ISLAND_ABI_RECEIPT
    })
}

fn island_abi_missing_directives(receipt: &Value) -> Vec<&'static str> {
    let directives = receipt.get("directives");
    let core_directives = receipt.get("core_directives");
    let supported_directives = receipt.get("supported_directives");
    [
        "clientLoad",
        "clientVisible",
        "clientIdle",
        "clientOnly",
        "clientMedia",
        "clientInteraction",
    ]
    .iter()
    .copied()
    .filter(|directive| {
        if matches!(*directive, "clientMedia" | "clientInteraction") {
            !json_string_array_contains(supported_directives, directive)
        } else {
            !json_string_array_contains(directives, directive)
                || !json_string_array_contains(core_directives, directive)
                || !json_string_array_contains(supported_directives, directive)
        }
    })
    .collect()
}

fn island_abi_missing_unsupported_syntax(receipt: &Value) -> Vec<&'static str> {
    let unsupported = receipt.get("unsupported_directive_syntax");
    [
        "client:load",
        "client:visible",
        "client:idle",
        "client:only",
    ]
    .iter()
    .copied()
    .filter(|syntax| !json_string_array_contains(unsupported, syntax))
    .collect()
}

fn write_readiness_reactivity_model_receipt(project: &Path) -> DxResult<Value> {
    let source_root = readiness_source_owned_repo_root(project);
    let source_checks = vec![
        readiness_reactivity_source_check(
            &source_root,
            "capability-contract",
            &[(
                "dx-www/src/cli/app_router_execution/state_runtime.rs",
                &[
                    "dx.tsx.dxNativeReactivityCapabilities",
                    "state()",
                    "derived()",
                    "effect()",
                    "action()",
                    "react_hook_policy",
                    "dx-native state()/derived()/effect()/action() runtime policy",
                    "DX-native state() slots, app-global store slots, and explicit state graph slots",
                    "\"full_react_hook_runtime\": false",
                    "unsupported_react_api_policy",
                    "dx.react-hook.useEffect.adapter-boundary-required",
                    "dx.react-hook.useReducer.adapter-boundary-required",
                    "dx.react-hook.useContext.adapter-boundary-required",
                    "adapter_boundary_required",
                ],
            )],
        ),
        readiness_reactivity_source_check(
            &source_root,
            "generated-state-runtime",
            &[(
                "dx-www/src/cli/app_router_execution/state_runtime.rs",
                &[
                    "__DX_STATE_GRAPH_RUNTIME__",
                    "reflectStateSlotToDom",
                    "setRuntimeSlot",
                    "refreshDerivedSlots",
                    "scheduleEffectsForState",
                    "dx:state-dom-reflection",
                    "dx:derived-state-slot",
                    "dx:effect-scheduled",
                    "dx:state-runtime-ready",
                ],
            )],
        ),
        readiness_reactivity_source_check(
            &source_root,
            "unsupported-react-diagnostics",
            &[(
                "dx-www/src/cli/app_router_execution/state_runtime.rs",
                &[
                    "dx:state-runtime-diagnostic",
                    "dx.state-runtime.operation.unsupported-react-like-operation",
                    "unsupported-react-like-state-operation",
                    "react_api_shim_executed: false",
                    "adapter_boundary_required: true",
                    "full_react_hook_runtime: false",
                ],
            )],
        ),
        readiness_reactivity_source_check(
            &source_root,
            "react-hook-lowering-diagnostics",
            &[(
                "dx-www/src/cli/app_router_semantics.rs",
                &[
                    "state_graph_has_exact_use_state_lowering",
                    "source_events_have_exact_use_state_lowering",
                    "event_handler_has_exact_use_state_setter_operation",
                    "is_lowerable_use_state_setter_argument",
                    "dx.react-hook.useState.exact-dx-state-slot-lowering",
                    "dx.react-hook.useState.missing-exact-state-slot",
                    "useState is compatibility sugar only when the compiler can lower it exactly into DX state slots.",
                    "\"adapter_boundary_required\": status != \"compatibility-lowered\"",
                    "react-effect-boundary",
                    "react-semantic-boundary",
                    "callback bodies and cleanup are not executed with hidden React semantics",
                ],
            )],
        ),
        readiness_reactivity_source_check(
            &source_root,
            "dom-action-state-bridge",
            &[(
                "dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs",
                &[
                    "dispatchDomActionPreviewToStateRuntime",
                    "state_runtime_dispatcher",
                    "dx:state-runtime-dispatch",
                    "full_react_hook_parity: false",
                    "react_synthetic_events: false",
                    "generated-dom-action-binder",
                ],
            )],
        ),
        readiness_reactivity_source_check(
            &source_root,
            "state-dom-reflection-markers",
            &[
                (
                    "dx-www/src/cli/app_router_execution/source_render.rs",
                    &[
                        "state_dom_reflection",
                        "dom_action_binder",
                        "client_islands",
                        "Bind generated state/event runtime operations to the real rendered DOM for client islands.",
                        "Define effect ordering and advanced hook lowering before claiming React runtime parity.",
                    ],
                ),
                (
                    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
                    &[
                        "data-dx-state-read",
                        "data-dx-state-value",
                        "data-dx-state-checked",
                        "data-dx-state-aria-*",
                        "The generated runtime hook updates only elements carrying compiler-owned data-dx-state-* markers.",
                    ],
                ),
            ],
        ),
    ];
    let source_check_count = source_checks.len();
    let source_check_current_count = source_checks
        .iter()
        .filter(|check| check.get("passed").and_then(Value::as_bool) == Some(true))
        .count();
    let passed = source_check_count == 6 && source_check_current_count == source_check_count;
    let status = if passed {
        "source-owned-reactivity-model-foundation-current"
    } else {
        "source-owned-reactivity-model-foundation-stale"
    };
    let receipt = json!({
        "schema": READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "reactivity",
        "reactivity_model_schema": READINESS_REACTIVITY_MODEL_SCHEMA,
        "runtime_capabilities_schema": "dx.tsx.dxNativeReactivityCapabilities",
        "runtime_capabilities": super::app_router_execution::dx_native_reactivity_capabilities(),
        "passed": passed,
        "status": status,
        "source_root": source_root.display().to_string(),
        "source_owned": true,
        "source_owned_runtime": true,
        "public_runtime": "DX-native fine-grained state",
        "dx_native_api": ["state()", "derived()", "effect()", "action()"],
        "runtime_policy": "dx-native state()/derived()/effect()/action() runtime policy",
        "react_familiar_authoring": true,
        "react_hook_policy": super::app_router_execution::dx_native_reactivity_capabilities()["react_hook_policy"].clone(),
        "compatibility_lowering": ["React useState exact DX-state-slot adapter syntax only"],
        "react_hook_inventory_api": ["useState"],
        "exact_lowering_required": true,
        "use_state_lowering_rule": "lower useState only when state_graph_has_exact_use_state_lowering proves every binding maps to a compiler-owned DX state slot",
        "unsupported_unlowerable_use_state_diagnostic": "dx.react-hook.useState.missing-exact-state-slot",
        "adapter_boundary_required_when_unlowerable": true,
        "unsupported_react_hooks": ["useEffect", "useReducer", "useContext", "useTransition"],
        "unsupported_react_api_policy": "React hooks are adapter-only authoring syntax; unsupported hooks must emit diagnostics or require adapter-boundary islands",
        "react_api_shim_executed": false,
        "full_react_hook_runtime": false,
        "node_modules_required": false,
        "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "live_browser_executed": false,
        "release_ready": false,
        "readiness_release_ready": false,
        "fastest_world_claim": false,
        "browser_proof_status": "foundation-not-release-proof",
        "proof_scope": "local-source-owned-reactivity-model-foundation",
        "node_vm_state_runtime_replay_status": "source-guarded-not-real-browser-proof",
        "node_vm_state_runtime_replay_test": "benchmarks/tsx-app-router-state-runtime-operations.test.ts",
        "browser_replay_receipt_contract": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
        "browser_replay_receipt": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        "browser_replay_receipt_sr": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        "browser_replay_receipt_machine": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
        "source_check_count": source_check_count,
        "source_check_current_count": source_check_current_count,
        "source_checks": source_checks,
        "next_proof": "real browser state, derived, effect, action replay and unsupported React API diagnostic proof",
        "rule": "This receipt validates the source-owned DX-native reactivity model only; React hooks are adapter-only inventory unless exact DX state-slot lowering proves a safe compatibility case, and this receipt does not run a browser, execute arbitrary React hooks, or claim hosted provider parity.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_REACTIVITY_MODEL_RECEIPT_SR,
        &readiness_reactivity_model_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_REACTIVITY_MODEL_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_REACTIVITY_MODEL_RECEIPT,
        &receipt,
        "reactivity model release readiness proof receipt",
    )?;

    Ok(json!({
        "id": "reactivity",
        "json_read_model_path": READINESS_REACTIVITY_MODEL_RECEIPT,
        "serializer_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "browser_runtime_executed": false,
        "hosted_provider_proof": false,
        "react_api_shim_executed": false,
        "full_react_hook_runtime": false,
    }))
}

fn readiness_reactivity_source_check(
    source_root: &Path,
    id: &'static str,
    file_markers: &[(&'static str, &[&'static str])],
) -> Value {
    let file_checks = file_markers
        .iter()
        .map(|(relative_path, markers)| {
            let path = source_root.join(relative_path);
            let source = std::fs::read_to_string(&path).ok();
            let missing_markers = markers
                .iter()
                .filter(|marker| {
                    !source
                        .as_deref()
                        .is_some_and(|text| text.contains(**marker))
                })
                .copied()
                .collect::<Vec<_>>();
            json!({
                "path": relative_path,
                "present": source.is_some(),
                "required_markers": markers,
                "missing_markers": missing_markers,
                "passed": source.is_some() && missing_markers.is_empty(),
            })
        })
        .collect::<Vec<_>>();
    let passed = file_checks
        .iter()
        .all(|check| check.get("passed").and_then(Value::as_bool) == Some(true));
    json!({
        "id": id,
        "passed": passed,
        "source_owned": true,
        "file_checks": file_checks,
    })
}

fn readiness_reactivity_model_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_REACTIVITY_MODEL_RECEIPT))
}

fn readiness_reactivity_model_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("reactivity")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-reactivity-model-foundation-current")
        && receipt.get("source_owned").and_then(Value::as_bool) == Some(true)
        && receipt.get("source_owned_runtime").and_then(Value::as_bool) == Some(true)
        && json_string_array_contains(receipt.get("dx_native_api"), "state()")
        && json_string_array_contains(receipt.get("dx_native_api"), "derived()")
        && json_string_array_contains(receipt.get("dx_native_api"), "effect()")
        && json_string_array_contains(receipt.get("dx_native_api"), "action()")
        && json_string_array_contains(
            receipt.get("compatibility_lowering"),
            "React useState exact DX-state-slot adapter syntax only",
        )
        && json_string_array_contains(receipt.get("react_hook_inventory_api"), "useState")
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
        && json_string_array_contains(receipt.get("unsupported_react_hooks"), "useReducer")
        && json_string_array_contains(receipt.get("unsupported_react_hooks"), "useContext")
        && json_string_array_contains(receipt.get("unsupported_react_hooks"), "useEffect")
        && json_string_array_contains(receipt.get("unsupported_react_hooks"), "useTransition")
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
            == Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT)
        && receipt
            .get("browser_replay_receipt")
            .and_then(Value::as_str)
            == Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT)
        && receipt
            .get("browser_replay_receipt_sr")
            .and_then(Value::as_str)
            == Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR)
        && receipt
            .get("browser_replay_receipt_machine")
            .and_then(Value::as_str)
            == Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE)
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

fn write_readiness_docs_onboarding_receipt(project: &Path) -> DxResult<Value> {
    let source_root = readiness_source_owned_repo_root(project);
    let source_checks = vec![
        readiness_docs_onboarding_source_check(
            &source_root,
            "docs-doctor-command-contract",
            &[(
                "dx-www/src/cli/docs_doctor.rs",
                &[
                    "dx.www.docs_doctor",
                    "dx www docs-doctor --json",
                    "MONITORED_PUBLIC_DOCS",
                    "MONITORED_COMPATIBILITY_SURFACES",
                    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
                    "DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS",
                    "DOCS_DOCTOR_UNRESOLVED_DOC_MACROS",
                    "DOCS_DOCTOR_ALLOWLISTS",
                    "READINESS_REQUIRED_RECEIPTS",
                    "STARTER_CHECK_RECEIPT_PATH",
                    "docs_doctor_required_receipt_findings",
                    "docs_doctor_config_snippet_findings",
                    "docs_doctor_unresolved_doc_macro_findings",
                    "generated_archived_claim_surface_policy",
                    "generated-archived-stale-claim",
                ],
            )],
        ),
        readiness_docs_onboarding_source_check(
            &source_root,
            "getting-started-current-workflow",
            &[(
                "docs/getting-started.md",
                &[
                    "dx new",
                    "dx dev",
                    "dx build",
                    "dx check",
                    "dx www readiness --json --full",
                    "dx www agent-context --json --full",
                    "dx www docs-doctor --json",
                    ".dx/www/output",
                    "not full React or Next.js runtime parity",
                ],
            )],
        ),
        readiness_docs_onboarding_source_check(
            &source_root,
            "dx-www-readme-config-contract",
            &[(
                "dx-www/README.md",
                &[
                    "project(name=dx-www-template",
                    "www(",
                    "output_dir=.dx/www/output",
                    "dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)",
                    "imports(",
                    "aliases=#imports,#components",
                    "check(score_scale=500 lighthouse=true)",
                ],
            )],
        ),
        readiness_docs_onboarding_source_check(
            &source_root,
            "docs-doctor-source-tests",
            &[(
                "benchmarks/dx-www-docs-doctor.test.ts",
                &[
                    "public WWW docs stay on the current app router and proof workflow",
                    "docs doctor separates public-doc failures from compatibility-surface warnings",
                    "docs doctor proves public starter score and inventory claims against current artifacts",
                    "docs doctor rejects stale config snippets and unresolved architecture placeholders",
                    "node --test benchmarks/dx-www-docs-doctor.test.ts",
                ],
            )],
        ),
        readiness_docs_onboarding_source_check(
            &source_root,
            "developer-contract-current-model",
            &[(
                "docs/dx-www-developer-contract.md",
                &[
                    "www should feel familiar to React and Next.js developers",
                    "app/",
                    "components/",
                    "styles/",
                    "Strict apps should keep hand-authored and forge-owned source",
                    "Generated caches, opaque dependency folders, and install artifacts are not part of the strict contract.",
                    "This is not a universal npm replacement claim.",
                ],
            )],
        ),
    ];
    let source_check_count = source_checks.len();
    let source_check_current_count = source_checks
        .iter()
        .filter(|check| check.get("passed").and_then(Value::as_bool) == Some(true))
        .count();
    let passed = source_check_count == 5 && source_check_current_count == source_check_count;
    let docs_doctor_report = docs_doctor::build_docs_doctor_report(&source_root);
    let docs_doctor_error_count = docs_doctor_report
        .get("error_count")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let docs_doctor_warning_count = docs_doctor_report
        .get("warning_count")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let generated_archived_warning_findings =
        readiness_docs_onboarding_generated_archived_warning_findings(&docs_doctor_report);
    let generated_archived_warning_surfaces_clean =
        docs_doctor_error_count == 0 && generated_archived_warning_findings.is_empty();
    let generated_archived_warning_finding_ids = generated_archived_warning_findings
        .iter()
        .filter_map(|finding| finding.get("id").and_then(Value::as_str))
        .map(str::to_string)
        .collect::<Vec<_>>();
    let generated_archived_warning_sample_paths = generated_archived_warning_findings
        .iter()
        .flat_map(|finding| {
            finding
                .get("sample_paths")
                .and_then(Value::as_array)
                .into_iter()
                .flat_map(|paths| paths.iter())
                .filter_map(Value::as_str)
                .map(str::to_string)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let status = if passed {
        "source-owned-docs-onboarding-foundation-current"
    } else {
        "source-owned-docs-onboarding-foundation-stale"
    };
    let receipt = json!({
        "schema": READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "docs-onboarding-doctor",
        "docs_onboarding_schema": READINESS_DOCS_ONBOARDING_SCHEMA,
        "docs_doctor_schema": "dx.www.docs_doctor",
        "passed": passed,
        "status": status,
        "source_root": source_root.display().to_string(),
        "source_owned": true,
        "docs_doctor_command": "dx www docs-doctor --json",
        "docs_doctor_report_evaluated": true,
        "docs_doctor_runtime_executed": false,
        "docs_doctor_error_count": docs_doctor_error_count,
        "docs_doctor_warning_count": docs_doctor_warning_count,
        "public_docs_source_guarded": true,
        "compatibility_surfaces_warning_only": true,
        "generated_archived_warning_surfaces_clean": generated_archived_warning_surfaces_clean,
        "generated_archived_warning_surfaces_promoted": false,
        "generated_archived_warning_finding_count": generated_archived_warning_findings.len(),
        "generated_archived_warning_finding_ids": generated_archived_warning_finding_ids,
        "generated_archived_warning_sample_paths": generated_archived_warning_sample_paths,
        "release_ready": false,
        "readiness_release_ready": false,
        "fastest_world_claim": false,
        "proof_scope": "local-source-owned-docs-onboarding-foundation",
        "source_check_count": source_check_count,
        "source_check_current_count": source_check_current_count,
        "source_checks": source_checks,
        "next_proof": "docs-doctor command replay, public docs freshness receipt, and generated/archived warning cleanup or ownership promotion",
        "rule": "This receipt validates the source-owned docs/onboarding guardrail and evaluates the docs-doctor report for generated/archive warning cleanup; it does not execute an external docs-doctor command or claim release readiness.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_DOCS_ONBOARDING_RECEIPT_SR,
        &readiness_docs_onboarding_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_DOCS_ONBOARDING_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_DOCS_ONBOARDING_RECEIPT,
        &receipt,
        "docs onboarding release readiness proof receipt",
    )?;

    Ok(json!({
        "id": "docs-onboarding-doctor",
        "json_read_model_path": READINESS_DOCS_ONBOARDING_RECEIPT,
        "serializer_receipt_path": READINESS_DOCS_ONBOARDING_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "fastest_world_claim": false,
        "docs_doctor_runtime_executed": false,
        "docs_doctor_report_evaluated": true,
        "docs_doctor_error_count": docs_doctor_error_count,
        "docs_doctor_warning_count": docs_doctor_warning_count,
        "generated_archived_warning_surfaces_clean": generated_archived_warning_surfaces_clean,
        "generated_archived_warning_finding_count": generated_archived_warning_findings.len(),
    }))
}

fn readiness_docs_onboarding_generated_archived_warning_findings(report: &Value) -> Vec<Value> {
    report
        .get("findings")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|findings| findings.iter())
        .filter(|finding| {
            matches!(
                finding.get("code").and_then(Value::as_str),
                Some("generated-archived-stale-claim")
                    | Some("missing-generated-archived-claim-surface")
            )
        })
        .cloned()
        .collect()
}

fn readiness_docs_onboarding_source_check(
    source_root: &Path,
    id: &'static str,
    file_markers: &[(&'static str, &[&'static str])],
) -> Value {
    let file_checks = file_markers
        .iter()
        .map(|(relative_path, markers)| {
            let path = source_root.join(relative_path);
            let source = std::fs::read_to_string(&path).ok();
            let missing_markers = markers
                .iter()
                .filter(|marker| {
                    !source
                        .as_deref()
                        .is_some_and(|text| text.contains(**marker))
                })
                .copied()
                .collect::<Vec<_>>();
            json!({
                "path": relative_path,
                "present": source.is_some(),
                "required_markers": markers,
                "missing_markers": missing_markers,
                "passed": source.is_some() && missing_markers.is_empty(),
            })
        })
        .collect::<Vec<_>>();
    let passed = file_checks
        .iter()
        .all(|check| check.get("passed").and_then(Value::as_bool) == Some(true));
    json!({
        "id": id,
        "passed": passed,
        "source_owned": true,
        "file_checks": file_checks,
    })
}

fn readiness_docs_onboarding_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_DOCS_ONBOARDING_RECEIPT))
}

fn readiness_docs_onboarding_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("docs-onboarding-doctor")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-docs-onboarding-foundation-current")
        && receipt
            .get("docs_onboarding_schema")
            .and_then(Value::as_str)
            == Some(READINESS_DOCS_ONBOARDING_SCHEMA)
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

fn write_readiness_bundle_partition_receipt(project: &Path) -> DxResult<Value> {
    let artifacts = readiness_bundle_partition_artifacts(project);
    let upload_plan = artifacts
        .provider_adapter
        .as_ref()
        .and_then(|adapter| adapter["upload_plan"].as_array())
        .cloned()
        .unwrap_or_default();
    let public_runtime_artifact_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "public-runtime")
        .count();
    let evidence_artifact_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "evidence")
        .count();
    let public_evidence_path_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "public-runtime")
        .filter_map(|artifact| artifact["path"].as_str())
        .filter(|path| readiness_bundle_evidence_only_path(path))
        .count();
    let precompressed_evidence_artifact_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "evidence")
        .filter_map(|artifact| artifact["path"].as_str())
        .filter(|path| readiness_precompressed_artifact_path(path))
        .filter(|path| readiness_bundle_evidence_only_path(path))
        .count();
    let precompressed_evidence_public_leak_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "public-runtime")
        .filter_map(|artifact| artifact["path"].as_str())
        .filter(|path| readiness_precompressed_artifact_path(path))
        .filter(|path| readiness_bundle_evidence_only_path(path))
        .count();
    let precompressed_evidence_path_samples = readiness_precompressed_evidence_path_samples();
    let precompressed_evidence_paths_classified = precompressed_evidence_path_samples
        .iter()
        .all(|path| readiness_bundle_evidence_only_path(path));
    let evidence_no_store = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "evidence")
        .all(|artifact| artifact["cache_control"] == "no-store");
    let precompressed_evidence_paths_no_store = precompressed_evidence_paths_classified
        && upload_plan
            .iter()
            .filter(|artifact| artifact["bundle"] == "evidence")
            .filter(|artifact| {
                artifact["path"]
                    .as_str()
                    .is_some_and(readiness_precompressed_artifact_path)
            })
            .filter(|artifact| {
                artifact["path"]
                    .as_str()
                    .is_some_and(readiness_bundle_evidence_only_path)
            })
            .all(|artifact| artifact["cache_control"] == "no-store");
    let deploy_partition_present = artifacts.deploy_adapter.as_ref().is_some_and(|adapter| {
        adapter["bundle_partition"]["schema"] == READINESS_BUNDLE_PARTITION_SCHEMA
            || (adapter["bundle_partition"]["schema"] == "dx.www.deploy.bundle_partition"
                && adapter["bundle_partition"]["separation_enforced"] == true)
    });
    let provider_partition_present = artifacts.provider_adapter.as_ref().is_some_and(|adapter| {
        adapter["bundle_partition"]["schema"] == READINESS_BUNDLE_PARTITION_SCHEMA
    });
    let evidence_not_public = artifacts.deploy_adapter.as_ref().is_some_and(|adapter| {
        adapter["bundle_partition"]["evidence_bundle"]["deployable_public_bytes"] == false
    });
    let public_runtime_deployable = artifacts.deploy_adapter.as_ref().is_some_and(|adapter| {
        adapter["bundle_partition"]["public_runtime_bundle"]["deployable"] == true
    });
    let passed = artifacts.deploy_adapter.is_some()
        && artifacts.provider_adapter.is_some()
        && deploy_partition_present
        && public_runtime_artifact_count > 0
        && evidence_artifact_count > 0
        && public_evidence_path_count == 0
        && precompressed_evidence_public_leak_count == 0
        && precompressed_evidence_paths_no_store
        && evidence_no_store
        && evidence_not_public
        && public_runtime_deployable;
    let status = if passed {
        "local-public-evidence-partition-current"
    } else if artifacts.deploy_adapter.is_none() {
        "missing-deploy-adapter"
    } else if artifacts.provider_adapter.is_none() {
        "missing-provider-adapter"
    } else if !deploy_partition_present {
        "missing-bundle-partition"
    } else if public_evidence_path_count > 0 {
        "public-runtime-contains-evidence-path"
    } else if precompressed_evidence_public_leak_count > 0 {
        "public-runtime-contains-precompressed-evidence-path"
    } else if !precompressed_evidence_paths_no_store {
        "precompressed-evidence-paths-not-no-store"
    } else if !evidence_no_store {
        "evidence-cache-control-not-no-store"
    } else {
        "bundle-partition-not-proven"
    };
    let receipt = json!({
        "schema": READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "bundle-partition",
        "passed": passed,
        "status": status,
        "release_ready": false,
        "hosted_provider_proof": false,
        "artifact_root": artifacts.artifact_root,
        "artifact_source": artifacts.artifact_source,
        "deploy_adapter_path": artifacts.deploy_adapter_relative,
        "provider_adapter_path": artifacts.provider_adapter_relative,
        "deploy_adapter_present": artifacts.deploy_adapter.is_some(),
        "provider_adapter_present": artifacts.provider_adapter.is_some(),
        "deploy_partition_present": deploy_partition_present,
        "provider_partition_present": provider_partition_present,
        "public_runtime_deployable": public_runtime_deployable,
        "evidence_bundle_deployable_public_bytes": !evidence_not_public,
        "public_runtime_artifact_count": public_runtime_artifact_count,
        "evidence_artifact_count": evidence_artifact_count,
        "public_runtime_evidence_path_count": public_evidence_path_count,
        "precompressed_evidence_artifact_count": precompressed_evidence_artifact_count,
        "precompressed_evidence_public_leak_count": precompressed_evidence_public_leak_count,
        "precompressed_evidence_path_samples": precompressed_evidence_path_samples,
        "precompressed_evidence_paths_no_store": precompressed_evidence_paths_no_store,
        "evidence_artifacts_no_store": evidence_no_store,
        "rule": "This receipt validates local deploy/provider adapter partition output only; hosted multi-provider upload replay remains required before release readiness.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_BUNDLE_PARTITION_RECEIPT_SR,
        &readiness_bundle_partition_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_BUNDLE_PARTITION_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }

    let json_path = project.join(READINESS_BUNDLE_PARTITION_RECEIPT);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json_text =
        serde_json::to_string_pretty(&receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!(
                "Failed to render bundle partition release readiness receipt: {error}"
            ),
            field: Some("www readiness".to_string()),
        })?;
    std::fs::write(&json_path, json_text).map_err(|error| DxError::IoError {
        path: Some(json_path),
        message: error.to_string(),
    })?;

    Ok(json!({
        "id": "bundle-partition",
        "json_read_model_path": READINESS_BUNDLE_PARTITION_RECEIPT,
        "serializer_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "status": status,
        "release_ready": false,
        "hosted_provider_proof": false,
    }))
}

fn readiness_no_js_artifact_paths(project: &Path) -> ReadinessNoJsArtifactPaths {
    let root_html = project.join(READINESS_NO_JS_OUTPUT_HTML_SUFFIX);
    let starter_root = project.join(READINESS_CANONICAL_STARTER_ROOT);
    let use_starter = !root_html.is_file() && starter_root.is_dir();
    let artifact_root = if use_starter {
        READINESS_CANONICAL_STARTER_ROOT.to_string()
    } else {
        ".".to_string()
    };
    let artifact_source = if use_starter {
        "examples-template-starter"
    } else {
        "project-root-output"
    };

    ReadinessNoJsArtifactPaths {
        html_relative: readiness_no_js_artifact_path(
            &artifact_root,
            READINESS_NO_JS_OUTPUT_HTML_SUFFIX,
        ),
        packet_relative: readiness_no_js_artifact_path(
            &artifact_root,
            READINESS_NO_JS_OUTPUT_PACKET_SUFFIX,
        ),
        route_unit_relative: readiness_no_js_artifact_path(
            &artifact_root,
            READINESS_NO_JS_ROUTE_UNIT_SUFFIX,
        ),
        artifact_root,
        artifact_source,
    }
}

fn readiness_no_js_artifact_path(artifact_root: &str, suffix: &str) -> String {
    if artifact_root == "." {
        suffix.to_string()
    } else {
        format!("{artifact_root}/{suffix}")
    }
}

fn readiness_bundle_partition_artifacts(project: &Path) -> ReadinessBundlePartitionArtifacts {
    let candidates = [
        (".", "project-root-output", ".dx/www/output"),
        (
            READINESS_CANONICAL_STARTER_ROOT,
            "examples-template-starter",
            "examples/template/.dx/www/output",
        ),
    ];
    for (artifact_root, artifact_source, output_relative) in candidates {
        let deploy_adapter_relative = format!("{output_relative}/.dx/build-cache/deploy-adapter.json");
        let deploy_adapter_path = project.join(&deploy_adapter_relative);
        if !deploy_adapter_path.is_file() {
            continue;
        }
        let deploy_adapter = read_json_file(&deploy_adapter_path);
        let provider_adapter_relative = deploy_adapter
            .as_ref()
            .and_then(|adapter| adapter["provider_adapter"]["path"].as_str())
            .map(|path| format!("{output_relative}/{path}"))
            .unwrap_or_else(|| format!("{output_relative}/.dx/build-cache/provider-adapter.dx-cloud.json"));
        let provider_adapter = read_json_file(&project.join(&provider_adapter_relative));
        return ReadinessBundlePartitionArtifacts {
            artifact_root: artifact_root.to_string(),
            artifact_source,
            deploy_adapter_relative,
            provider_adapter_relative,
            deploy_adapter,
            provider_adapter,
        };
    }

    ReadinessBundlePartitionArtifacts {
        artifact_root: ".".to_string(),
        artifact_source: "project-root-output",
        deploy_adapter_relative: ".dx/www/output/.dx/build-cache/deploy-adapter.json".to_string(),
        provider_adapter_relative: ".dx/www/output/.dx/build-cache/provider-adapter.dx-cloud.json".to_string(),
        deploy_adapter: None,
        provider_adapter: None,
    }
}

fn readiness_bundle_evidence_only_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let decoded = readiness_decoded_precompressed_path_str(&normalized);
    decoded.starts_with(".dx/")
        || decoded.starts_with(".dx/build-cache/source-routes/")
        || decoded.ends_with(".sr")
        || decoded.ends_with(".machine")
        || decoded == ".dx/build-cache/deploy-adapter.json"
        || decoded == ".dx/build-cache/provider-adapter.dx-cloud.json"
        || decoded == ".dx/build-cache/provider-adapter-smoke-matrix.json"
        || decoded == ".dx/build-cache/route-handler-conformance-matrix.json"
        || decoded == ".dx/build-cache/server-action-replay-ledger.json"
        || decoded == ".dx/build-cache/cache-manifest.json"
        || decoded == ".dx/build-cache/rollback.json"
        || decoded == ".dx/build-cache/manifest.json"
        || decoded.ends_with("/page-graph.json")
        || decoded.ends_with("/app-router-execution.json")
        || decoded.ends_with("/client-islands.json")
        || decoded.ends_with("/streaming-plan.json")
}

fn readiness_decoded_precompressed_path_str(path: &str) -> &str {
    path.strip_suffix(".br")
        .or_else(|| path.strip_suffix(".gz"))
        .unwrap_or(path)
}

fn readiness_precompressed_artifact_path(path: &str) -> bool {
    path.ends_with(".br") || path.ends_with(".gz")
}

fn readiness_precompressed_evidence_path_samples() -> Vec<&'static str> {
    vec![
        ".dx/build-cache/deploy-adapter.json.br",
        ".dx/build-cache/cache-manifest.json.gz",
        ".dx/build-cache/source-routes/root/route-unit.json.br",
        ".dx/receipts/readiness/proof-graph.sr.gz",
    ]
}

fn write_readiness_native_event_browser_binder_sr_receipt(
    project: &Path,
) -> DxResult<Option<Value>> {
    let Some(receipt) = native_event_browser_binder_receipt(project) else {
        return Ok(None);
    };
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        &readiness_native_event_browser_binder_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    Ok(Some(json!({
        "id": "native-event-browser-binder",
        "json_read_model_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": native_event_browser_binder_receipt_is_current(&receipt),
        "status": native_event_browser_binder_status_from_receipt(&receipt),
        "release_ready": false,
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })))
}

fn native_event_browser_binder_status(project: &Path) -> Value {
    let receipt = native_event_browser_binder_receipt(project);
    let current = receipt
        .as_ref()
        .is_some_and(native_event_browser_binder_receipt_is_current);
    json!({
        "contract": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT,
        "path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        "machine_contract_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
        "current": current,
        "status": receipt
            .as_ref()
            .map(native_event_browser_binder_status_from_receipt)
            .unwrap_or("missing-browser-binder-receipt"),
        "browser_runtime_executed": receipt
            .as_ref()
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "binder_global_present": receipt
            .as_ref()
            .and_then(|value| value.get("binder_global_present"))
            .and_then(Value::as_bool),
        "supported_event_count": receipt
            .as_ref()
            .and_then(|value| value.get("supported_event_count"))
            .and_then(Value::as_u64),
        "catalog_count": native_dom_event_names().len(),
        "catalog_hash": receipt
            .as_ref()
            .and_then(|value| value.get("catalog_hash"))
            .and_then(Value::as_str),
        "expected_catalog_hash": native_dom_event_catalog_integrity().catalog_hash,
        "listener_events": receipt
            .as_ref()
            .and_then(|value| value.get("listener_events"))
            .cloned()
            .unwrap_or(Value::Null),
        "required_events": receipt
            .as_ref()
            .and_then(|value| value.get("required_events"))
            .cloned()
            .unwrap_or(Value::Null),
        "missing_listener_events": receipt
            .as_ref()
            .and_then(|value| value.get("missing_listener_events"))
            .cloned()
            .unwrap_or(Value::Null),
        "missing_contract_events": receipt
            .as_ref()
            .and_then(|value| value.get("missing_contract_events"))
            .cloned()
            .unwrap_or(Value::Null),
        "missing_replay_events": receipt
            .as_ref()
            .and_then(|value| value.get("missing_replay_events"))
            .cloned()
            .unwrap_or(Value::Null),
        "browser_event_constructors": receipt
            .as_ref()
            .and_then(|value| value.get("browser_event_constructors"))
            .cloned()
            .unwrap_or(Value::Null),
        "browser_snapshot_hash": receipt
            .as_ref()
            .and_then(|value| value.get("browser_snapshot_hash"))
            .and_then(Value::as_str),
        "unsupported_listener_attached": receipt
            .as_ref()
            .and_then(|value| value.get("unsupported_listener_attached"))
            .and_then(Value::as_bool),
        "preview_event_count": receipt
            .as_ref()
            .and_then(|value| value.get("preview_event_count"))
            .and_then(Value::as_u64),
        "state_dispatch_count": receipt
            .as_ref()
            .and_then(|value| value.get("state_dispatch_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .as_ref()
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
        "release_ready": false,
        "rule": "A current native-event browser binder receipt proves local browser execution of the generated binder only; it is not hosted provider or full release proof.",
    })
}

fn native_event_browser_binder_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT))
}

fn native_event_browser_binder_status_from_receipt(receipt: &Value) -> &'static str {
    if native_event_browser_binder_receipt_is_current(receipt) {
        "browser-binder-replay-current"
    } else {
        "browser-binder-replay-stale"
    }
}

fn native_event_browser_binder_receipt_is_current(receipt: &Value) -> bool {
    let catalog_hash = native_dom_event_catalog_integrity().catalog_hash;
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("binder_global_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("unsupported_listener_attached")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("supported_event_count").and_then(Value::as_u64)
            == Some(native_dom_event_names().len() as u64)
        && receipt.get("catalog_hash").and_then(Value::as_str) == Some(catalog_hash.as_str())
        && receipt
            .get("preview_event_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 3)
        && receipt
            .get("state_dispatch_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 3)
        && json_string_array_contains(receipt.get("listener_events"), "click")
        && json_string_array_contains(receipt.get("listener_events"), "pointermove")
        && json_string_array_contains(receipt.get("listener_events"), "input")
        && json_string_array_contains(receipt.get("required_events"), "click")
        && json_string_array_contains(receipt.get("required_events"), "pointermove")
        && json_string_array_contains(receipt.get("required_events"), "input")
        && json_array_is_empty(receipt.get("missing_listener_events"))
        && json_array_is_empty(receipt.get("missing_contract_events"))
        && json_array_is_empty(receipt.get("missing_replay_events"))
        && json_object_string_at(
            receipt.get("browser_event_constructors"),
            "click",
            "MouseEvent",
        )
        && json_object_string_at(
            receipt.get("browser_event_constructors"),
            "pointermove",
            "PointerEvent",
        )
        && json_object_string_at(
            receipt.get("browser_event_constructors"),
            "input",
            "InputEvent",
        )
        && json_array_len_at_least(receipt.get("browser_event_replay_results"), 3)
        && json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "click",
            "previewed",
            true,
        )
        && json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "pointermove",
            "previewed",
            true,
        )
        && json_array_record_string_field_contains_with_bool(
            receipt.get("browser_event_replay_results"),
            "event",
            "input",
            "previewed",
            true,
        )
        && json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-native-event-binder-replay")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("react_synthetic_events")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("full_react_event_parity")
            .and_then(Value::as_bool)
            == Some(false)
}

fn import_readiness_native_event_browser_binder_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-native-event-browser-binder-receipt",
    )?;
    if !native_event_browser_binder_receipt_is_current(&receipt) {
        let stale_reason = native_event_browser_binder_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("native-event-browser-binder-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported native-event browser binder receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-native-event-browser-binder-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        &readiness_native_event_browser_binder_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    add_imported_browser_receipt_metadata(
        project,
        &source_path,
        "www readiness --import-native-event-browser-binder-receipt",
        &serializer_provenance,
        &mut receipt,
    );
    write_readiness_json_receipt(
        project,
        READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        &receipt,
        "native-event browser binder release readiness import receipt",
    )?;

    Ok(json!({
        "id": "native-event-browser-binder",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": native_event_browser_binder_status_from_receipt(&receipt),
        "release_ready": false,
        "fastest_world_claim": false,
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "import_rule": "validated-current-before-canonical-write",
    }))
}

fn import_readiness_browser_page_snapshot_receipts(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let snapshot =
        read_readiness_import_json(&source_path, "www readiness --import-browser-page-snapshot")?;
    if snapshot.get("schema").and_then(Value::as_str)
        != Some(READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA)
    {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Browser page snapshot import must use schema {}: {}",
                READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA,
                source_path.display()
            ),
            field: Some("www readiness --import-browser-page-snapshot".to_string()),
        });
    }
    if snapshot
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Browser page snapshot import did not execute in a browser runtime: {}",
                source_path.display()
            ),
            field: Some("www readiness --import-browser-page-snapshot".to_string()),
        });
    }

    let harness = project.join(READINESS_BROWSER_RECEIPT_HARNESS);
    if !harness.is_file() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Browser page snapshot import requires the source-owned harness at {}",
                READINESS_BROWSER_RECEIPT_HARNESS
            ),
            field: Some("www readiness --import-browser-page-snapshot".to_string()),
        });
    }
    let candidate_dir = project.join(READINESS_BROWSER_IMPORT_CANDIDATE_DIR);
    std::fs::create_dir_all(&candidate_dir).map_err(|error| DxError::IoError {
        path: Some(candidate_dir.clone()),
        message: error.to_string(),
    })?;
    let output = Command::new("node")
        .arg(&harness)
        .arg("--from-page-json")
        .arg(&source_path)
        .arg("--out-dir")
        .arg(&candidate_dir)
        .current_dir(project)
        .output()
        .map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to run browser page snapshot harness with node: {error}"),
            field: Some("www readiness --import-browser-page-snapshot".to_string()),
        })?;
    if !output.status.success() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Browser page snapshot harness failed with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ),
            field: Some("www readiness --import-browser-page-snapshot".to_string()),
        });
    }

    let native_path = candidate_dir.join("native-event-browser-binder-latest.json");
    let state_path = candidate_dir.join("state-runtime-browser-latest.json");
    let island_browser_path = candidate_dir.join("island-browser-latest.json");
    let visual_path = candidate_dir.join("visual-edit-browser-workbench-latest.json");
    let no_js_browser_path = candidate_dir.join("no-js-browser-latest.json");
    validate_browser_page_snapshot_candidate_receipts(
        project,
        &native_path,
        &state_path,
        &island_browser_path,
        &visual_path,
    )?;
    let no_js_browser_candidate =
        browser_page_snapshot_no_js_browser_candidate(project, &no_js_browser_path)?;

    let mut receipts = vec![
        import_readiness_native_event_browser_binder_receipt(project, &native_path)?,
        import_readiness_state_runtime_browser_receipt(project, &state_path)?,
        import_readiness_island_browser_receipt(project, &island_browser_path)?,
        import_readiness_visual_edit_browser_receipt(project, &visual_path)?,
    ];
    let skipped_receipts = if no_js_browser_candidate.current {
        receipts.push(import_readiness_no_js_browser_receipt(
            project,
            &no_js_browser_path,
        )?);
        Vec::new()
    } else {
        vec![json!({
            "id": "no-js-browser",
            "candidate_path": readiness_import_source_path(project, &no_js_browser_path),
            "reason": no_js_browser_candidate.stale_reason,
            "import_command": "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
            "rule": "JS-enabled page snapshots can refresh runtime browser receipts without inventing JS-disabled no-JS browser proof."
        })]
    };

    Ok(json!({
        "id": "browser-page-snapshot",
        "schema": READINESS_BROWSER_PAGE_SNAPSHOT_SCHEMA,
        "imported_from": readiness_import_source_path(project, &source_path),
        "candidate_out_dir": readiness_import_source_path(project, &candidate_dir),
        "harness": READINESS_BROWSER_RECEIPT_HARNESS,
        "converter_command": format!(
            "node {} --from-page-json {} --out-dir {}",
            READINESS_BROWSER_RECEIPT_HARNESS,
            readiness_import_source_path(project, &source_path),
            READINESS_BROWSER_IMPORT_CANDIDATE_DIR
        ),
        "receipts": receipts,
        "skipped_receipts": skipped_receipts,
        "release_ready": false,
        "fastest_world_claim": false,
        "import_rule": "real-page-snapshot-converted-then-validated-current-before-canonical-write",
    }))
}

struct BrowserPageSnapshotNoJsCandidate {
    current: bool,
    stale_reason: Value,
}

fn browser_page_snapshot_no_js_browser_candidate(
    project: &Path,
    no_js_browser_path: &Path,
) -> DxResult<BrowserPageSnapshotNoJsCandidate> {
    if !no_js_browser_path.is_file() {
        return Ok(BrowserPageSnapshotNoJsCandidate {
            current: false,
            stale_reason: json!({
                "code": "no-js-browser-candidate-missing",
                "message": "Browser page snapshot harness did not emit a no-JS browser candidate.",
                "candidate_path": readiness_import_source_path(project, no_js_browser_path)
            }),
        });
    }

    let receipt = read_readiness_import_json(
        no_js_browser_path,
        "www readiness --import-browser-page-snapshot",
    )?;
    if readiness_no_js_browser_receipt_is_current(project, &receipt) {
        Ok(BrowserPageSnapshotNoJsCandidate {
            current: true,
            stale_reason: readiness_no_js_browser_stale_reason_from_receipt(project, &receipt),
        })
    } else {
        Ok(BrowserPageSnapshotNoJsCandidate {
            current: false,
            stale_reason: readiness_no_js_browser_stale_reason_from_receipt(project, &receipt),
        })
    }
}

fn validate_browser_page_snapshot_candidate_receipts(
    project: &Path,
    native_path: &Path,
    state_path: &Path,
    island_browser_path: &Path,
    visual_path: &Path,
) -> DxResult<()> {
    let native =
        read_readiness_import_json(native_path, "www readiness --import-browser-page-snapshot")?;
    if !native_event_browser_binder_receipt_is_current(&native) {
        return Err(browser_page_snapshot_candidate_error(
            "native-event-browser-binder",
            native_path,
            native_event_browser_binder_stale_reason_from_receipt(&native),
        ));
    }
    let state =
        read_readiness_import_json(state_path, "www readiness --import-browser-page-snapshot")?;
    if !state_runtime_browser_receipt_is_current(&state) {
        return Err(browser_page_snapshot_candidate_error(
            "state-runtime-browser",
            state_path,
            state_runtime_browser_stale_reason_from_receipt(&state),
        ));
    }
    let island_browser = read_readiness_import_json(
        island_browser_path,
        "www readiness --import-browser-page-snapshot",
    )?;
    if !island_browser_receipt_is_current(&island_browser) {
        return Err(browser_page_snapshot_candidate_error(
            "island-browser",
            island_browser_path,
            island_browser_stale_reason_from_receipt(&island_browser),
        ));
    }
    let visual =
        read_readiness_import_json(visual_path, "www readiness --import-browser-page-snapshot")?;
    if !visual_edit_browser_workbench_receipt_value_is_current(project, &visual) {
        return Err(browser_page_snapshot_candidate_error(
            "visual-edit-browser-workbench",
            visual_path,
            readiness_visual_edit_browser_stale_reason_from_receipt(project, &visual),
        ));
    }
    Ok(())
}

fn import_readiness_no_js_browser_receipt(project: &Path, source: &Path) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt =
        read_readiness_import_json(&source_path, "www readiness --import-no-js-browser-receipt")?;
    if !readiness_no_js_browser_receipt_is_current(project, &receipt) {
        let stale_reason = readiness_no_js_browser_stale_reason_from_receipt(project, &receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("no-js-browser-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported no-JS browser receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-no-js-browser-receipt".to_string()),
        });
    }
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_NO_JS_BROWSER_RECEIPT_SR,
        &readiness_no_js_browser_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_NO_JS_BROWSER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    add_imported_browser_receipt_metadata(
        project,
        &source_path,
        "www readiness --import-no-js-browser-receipt",
        &serializer_provenance,
        &mut receipt,
    );
    write_readiness_json_receipt(
        project,
        READINESS_NO_JS_BROWSER_RECEIPT,
        &receipt,
        "no-JS browser release readiness import receipt",
    )?;

    Ok(json!({
        "id": "no-js-browser",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_NO_JS_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_NO_JS_BROWSER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "current-local-js-disabled-browser-proof",
        "release_ready": false,
        "fastest_world_claim": false,
        "live_browser_executed": true,
        "javascript_disabled_browser": true,
        "import_rule": "validated-current-before-canonical-write",
    }))
}

fn import_readiness_production_http_tcp_preview_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-production-http-tcp-preview-receipt",
    )?;
    if !readiness_production_http_tcp_preview_receipt_is_current(&receipt) {
        let stale_reason =
            readiness_production_http_tcp_preview_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("production-http-tcp-preview-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported production HTTP TCP preview receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-production-http-tcp-preview-receipt".to_string()),
        });
    }
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
        &readiness_production_http_tcp_preview_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "import_source_path".to_string(),
            json!(readiness_import_source_path(project, &source_path)),
        );
        object.insert(
            "import_source_within_project".to_string(),
            json!(artifact_path_within_root(project, &source_path)),
        );
        object.insert(
            "imported_by".to_string(),
            json!("www readiness --import-production-http-tcp-preview-receipt"),
        );
        object.insert(
            "import_rule".to_string(),
            json!("validated-current-before-canonical-write"),
        );
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert("fastest_world_claim".to_string(), json!(false));
    }
    write_readiness_json_receipt(
        project,
        READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
        &receipt,
        "production HTTP TCP preview release readiness import receipt",
    )?;

    Ok(json!({
        "id": "production-http-tcp-preview",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
        "serializer_receipt_path": READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "local-production-http-tcp-preview-current",
        "release_ready": false,
        "fastest_world_claim": false,
        "remaining_external_proof_gap_ids": [
            "browser-js-enabled-runtime-replay",
            "browser-js-disabled-runtime-replay",
            "axum-static-responder-parity",
            "provider-bound-cdn-cache-replay",
            "hosted-provider-adapter-replay"
        ]
    }))
}

fn browser_page_snapshot_candidate_error(
    id: &'static str,
    path: &Path,
    stale_reason: Value,
) -> DxError {
    let stale_reason_code = stale_reason
        .get("code")
        .and_then(Value::as_str)
        .unwrap_or("browser-page-snapshot-candidate-stale");
    DxError::ConfigValidationError {
        message: format!(
            "Browser page snapshot produced stale {id} candidate ({stale_reason_code}): {}",
            path.display()
        ),
        field: Some("www readiness --import-browser-page-snapshot".to_string()),
    }
}

fn write_readiness_state_runtime_browser_sr_receipt(project: &Path) -> DxResult<Option<Value>> {
    let Some(receipt) = state_runtime_browser_receipt(project) else {
        return Ok(None);
    };
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        &readiness_state_runtime_browser_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    Ok(Some(json!({
        "id": "state-runtime-browser",
        "json_read_model_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": state_runtime_browser_receipt_is_current(&receipt),
        "status": state_runtime_browser_status_from_receipt(&receipt),
        "release_ready": false,
        "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })))
}

fn state_runtime_browser_status(project: &Path) -> Value {
    let receipt = state_runtime_browser_receipt(project);
    let current = receipt
        .as_ref()
        .is_some_and(state_runtime_browser_receipt_is_current);
    json!({
        "contract": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
        "path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        "machine_contract_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
        "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "import_command": "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
        "page_snapshot_import_command": "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
        "current": current,
        "status": receipt
            .as_ref()
            .map(state_runtime_browser_status_from_receipt)
            .unwrap_or("missing-state-runtime-browser-receipt"),
        "browser_runtime_executed": receipt
            .as_ref()
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "runtime_global_present": receipt
            .as_ref()
            .and_then(|value| value.get("runtime_global_present"))
            .and_then(Value::as_bool),
        "state_reflection_event_count": receipt
            .as_ref()
            .and_then(|value| value.get("state_reflection_event_count"))
            .and_then(Value::as_u64),
        "derived_reflection_event_count": receipt
            .as_ref()
            .and_then(|value| value.get("derived_reflection_event_count"))
            .and_then(Value::as_u64),
        "effect_scheduled_event_count": receipt
            .as_ref()
            .and_then(|value| value.get("effect_scheduled_event_count"))
            .and_then(Value::as_u64),
        "action_dispatch_count": receipt
            .as_ref()
            .and_then(|value| value.get("action_dispatch_count"))
            .and_then(Value::as_u64),
        "missing_api_methods": receipt
            .as_ref()
            .and_then(|value| value.get("missing_api_methods"))
            .cloned()
            .unwrap_or(Value::Null),
        "slot_count": receipt
            .as_ref()
            .and_then(|value| value.get("slot_count"))
            .and_then(Value::as_u64),
        "event_count": receipt
            .as_ref()
            .and_then(|value| value.get("event_count"))
            .and_then(Value::as_u64),
        "browser_snapshot_hash": receipt
            .as_ref()
            .and_then(|value| value.get("browser_snapshot_hash"))
            .and_then(Value::as_str),
        "proof_scope": receipt
            .as_ref()
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
        "release_ready": false,
        "rule": "A current state-runtime browser receipt proves local browser execution of generated DX-native state/derived/effect/action plumbing only; hosted provider and full release proof remain separate gates.",
    })
}

fn state_runtime_browser_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT))
}

fn state_runtime_browser_status_from_receipt(receipt: &Value) -> &'static str {
    if state_runtime_browser_receipt_is_current(receipt) {
        "state-runtime-browser-replay-current"
    } else {
        "state-runtime-browser-replay-stale"
    }
}

fn state_runtime_browser_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT)
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
        && json_string_array_contains(receipt.get("api_methods"), "getSnapshot")
        && json_string_array_contains(receipt.get("api_methods"), "setSlot")
        && json_string_array_contains(receipt.get("api_methods"), "dispatch")
        && json_string_array_contains(receipt.get("api_methods"), "refreshDerivedSlots")
        && json_string_array_contains(receipt.get("api_methods"), "scheduleEffectsForState")
        && json_array_is_empty(receipt.get("missing_api_methods"))
        && receipt
            .get("slot_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 1)
        && receipt
            .get("event_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 1)
        && json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-state-runtime-replay")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn import_readiness_state_runtime_browser_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-state-runtime-browser-receipt",
    )?;
    if !state_runtime_browser_receipt_is_current(&receipt) {
        let stale_reason = state_runtime_browser_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("state-runtime-browser-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported state-runtime browser receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-state-runtime-browser-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        &readiness_state_runtime_browser_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    add_imported_browser_receipt_metadata(
        project,
        &source_path,
        "www readiness --import-state-runtime-browser-receipt",
        &serializer_provenance,
        &mut receipt,
    );
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "canonical_starter_route".to_string(),
            json!(READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE),
        );
        object.insert(
            "canonical_proof_target_route".to_string(),
            json!(READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE),
        );
        object.insert(
            "canonical_starter_source".to_string(),
            json!(READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE),
        );
        object.insert(
            "canonical_local_dev_url".to_string(),
            json!(READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL),
        );
        object.insert(
            "browser_runtime_executed_by_readiness".to_string(),
            json!(false),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        &receipt,
        "state-runtime browser release readiness import receipt",
    )?;

    Ok(json!({
        "id": "state-runtime-browser",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": state_runtime_browser_status_from_receipt(&receipt),
        "release_ready": false,
        "fastest_world_claim": false,
        "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "import_rule": "validated-current-before-canonical-write",
    }))
}

fn write_readiness_island_browser_sr_receipt(project: &Path) -> DxResult<Option<Value>> {
    let Some(receipt) = readiness_island_browser_receipt(project) else {
        return Ok(None);
    };
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_ISLAND_BROWSER_RECEIPT_SR,
        &readiness_island_browser_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_ISLAND_BROWSER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    Ok(Some(json!({
        "id": "island-browser",
        "json_read_model_path": READINESS_ISLAND_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_ISLAND_BROWSER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": island_browser_receipt_is_current(&receipt),
        "status": island_browser_status_from_receipt(&receipt),
        "release_ready": false,
        "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })))
}

fn readiness_island_browser_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_ISLAND_BROWSER_RECEIPT))
}

fn island_browser_status_from_receipt(receipt: &Value) -> &'static str {
    if island_browser_receipt_is_current(receipt) {
        "source-owned-island-browser-replay-current"
    } else {
        "source-owned-island-browser-replay-stale"
    }
}

fn island_browser_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT)
        && receipt.get("schema_revision").and_then(Value::as_u64) == Some(1)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-island-browser-replay-current")
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("bridge_present").and_then(Value::as_bool) == Some(true)
        && receipt.get("source_owned_bridge").and_then(Value::as_bool) == Some(true)
        && receipt.get("bridge_abi_style").and_then(Value::as_str) == Some("camelCase")
        && receipt.get("abi_schema").and_then(Value::as_str) == Some("dx.react.clientIsland.abi")
        && receipt.get("directive_style").and_then(Value::as_str) == Some("camelCase-jsx-props")
        && receipt
            .get("no_js_fallback_preserved")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("island_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count > 0)
        && receipt
            .get("source_owned_island_count")
            .and_then(Value::as_u64)
            == receipt.get("island_count").and_then(Value::as_u64)
        && json_string_array_contains(receipt.get("directives_seen"), "clientLoad")
        && json_string_array_contains(receipt.get("directives_seen"), "clientVisible")
        && json_string_array_contains(receipt.get("directives_seen"), "clientIdle")
        && json_string_array_contains(receipt.get("directives_seen"), "clientOnly")
        && json_array_is_empty(receipt.get("missing_core_directives"))
        && json_array_len_at_least(receipt.get("hydration_strategies"), 1)
        && receipt
            .get("event_node_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count > 0)
        && receipt
            .get("client_island_event_count")
            .and_then(Value::as_u64)
            >= receipt.get("event_node_count").and_then(Value::as_u64)
        && receipt
            .get("missed_event_replay_count")
            .and_then(Value::as_u64)
            == Some(0)
        && receipt.get("full_react_hydration").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("node_modules_required")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("react_synthetic_events")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("provider_adapter_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-source-owned-island-replay")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn readiness_island_browser_stale_reason(root: &Path) -> Value {
    match readiness_island_browser_receipt(root) {
        Some(receipt) => island_browser_stale_reason_from_receipt(&receipt),
        None => json!({
            "code": "island-browser-receipt-missing",
            "message": "Source-owned island browser replay proof is missing; import a real browser island receipt before claiming local island runtime execution.",
            "expected_receipt_path": READINESS_ISLAND_BROWSER_RECEIPT,
            "serializer_receipt_path": READINESS_ISLAND_BROWSER_RECEIPT_SR,
            "machine_contract_path": READINESS_ISLAND_BROWSER_RECEIPT_MACHINE,
            "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
            "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
            "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
            "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
            "browser_runtime_executed_by_readiness": false,
            "import_command": "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
            "page_snapshot_import_command": "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full"
        }),
    }
}

fn island_browser_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "island-browser-schema-mismatch",
            "message": "Island browser receipt uses the wrong schema contract.",
            "expected_schema": READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT,
            "schema": receipt.get("schema").and_then(Value::as_str)
        });
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
        || receipt.get("full_react_hydration").and_then(Value::as_bool) != Some(false)
        || receipt
            .get("node_modules_required")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .get("provider_adapter_executed")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return json!({
            "code": "island-browser-overclaims-runtime-or-provider-proof",
            "message": "Island browser receipt overclaims release, global speed, full React hydration, node_modules, or provider adapter execution.",
            "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
            "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
            "full_react_hydration": receipt.get("full_react_hydration").and_then(Value::as_bool),
            "node_modules_required": receipt.get("node_modules_required").and_then(Value::as_bool),
            "provider_adapter_executed": receipt.get("provider_adapter_executed").and_then(Value::as_bool)
        });
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt.get("bridge_present").and_then(Value::as_bool) != Some(true)
        || receipt.get("source_owned_bridge").and_then(Value::as_bool) != Some(true)
        || receipt.get("bridge_abi_style").and_then(Value::as_str) != Some("camelCase")
        || receipt.get("abi_schema").and_then(Value::as_str) != Some("dx.react.clientIsland.abi")
        || receipt.get("directive_style").and_then(Value::as_str) != Some("camelCase-jsx-props")
        || receipt
            .get("no_js_fallback_preserved")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return json!({
            "code": "island-browser-dom-marker-proof-incomplete",
            "message": "Island browser receipt is missing source-owned bridge, camelCase ABI, or no-JS fallback DOM proof.",
            "browser_runtime_executed": receipt.get("browser_runtime_executed").and_then(Value::as_bool),
            "bridge_present": receipt.get("bridge_present").and_then(Value::as_bool),
            "source_owned_bridge": receipt.get("source_owned_bridge").and_then(Value::as_bool),
            "bridge_abi_style": receipt.get("bridge_abi_style").and_then(Value::as_str),
            "abi_schema": receipt.get("abi_schema").and_then(Value::as_str),
            "directive_style": receipt.get("directive_style").and_then(Value::as_str)
        });
    }
    if !json_string_array_contains(receipt.get("directives_seen"), "clientLoad")
        || !json_string_array_contains(receipt.get("directives_seen"), "clientVisible")
        || !json_string_array_contains(receipt.get("directives_seen"), "clientIdle")
        || !json_string_array_contains(receipt.get("directives_seen"), "clientOnly")
        || !json_array_is_empty(receipt.get("missing_core_directives"))
    {
        return json!({
            "code": "island-browser-directive-coverage-incomplete",
            "message": "Island browser receipt does not prove all release-core camelCase island directives.",
            "directives_seen": receipt.get("directives_seen").cloned().unwrap_or(Value::Null),
            "missing_core_directives": receipt.get("missing_core_directives").cloned().unwrap_or(Value::Null)
        });
    }
    if receipt
        .get("island_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
        || receipt
            .get("source_owned_island_count")
            .and_then(Value::as_u64)
            != receipt.get("island_count").and_then(Value::as_u64)
        || receipt
            .get("event_node_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count == 0)
        || receipt
            .get("missed_event_replay_count")
            .and_then(Value::as_u64)
            != Some(0)
        || receipt
            .get("client_island_event_count")
            .and_then(Value::as_u64)
            < receipt.get("event_node_count").and_then(Value::as_u64)
    {
        return json!({
            "code": "island-browser-runtime-event-replay-incomplete",
            "message": "Island browser receipt must prove at least one source-owned island event replay with no missed replays.",
            "island_count": receipt.get("island_count").and_then(Value::as_u64),
            "source_owned_island_count": receipt.get("source_owned_island_count").and_then(Value::as_u64),
            "event_node_count": receipt.get("event_node_count").and_then(Value::as_u64),
            "client_island_event_count": receipt.get("client_island_event_count").and_then(Value::as_u64),
            "missed_event_replay_count": receipt.get("missed_event_replay_count").and_then(Value::as_u64)
        });
    }
    if !json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        || receipt.get("proof_scope").and_then(Value::as_str)
            != Some("local-in-app-browser-source-owned-island-replay")
        || receipt.get("passed").and_then(Value::as_bool) != Some(true)
        || receipt.get("status").and_then(Value::as_str)
            != Some("source-owned-island-browser-replay-current")
    {
        return json!({
            "code": "island-browser-status-or-snapshot-stale",
            "message": "Island browser receipt has coverage fields but stale status, proof scope, or browser snapshot hash.",
            "passed": receipt.get("passed").and_then(Value::as_bool),
            "status": receipt.get("status").and_then(Value::as_str),
            "proof_scope": receipt.get("proof_scope").and_then(Value::as_str),
            "browser_snapshot_hash": receipt.get("browser_snapshot_hash").and_then(Value::as_str)
        });
    }
    json!({
        "code": "island-browser-hosted-provider-proof-missing",
        "message": "Source-owned island browser replay is current locally; hosted/provider adapter breadth proof remains required before release readiness.",
        "expected_receipt_path": READINESS_ISLAND_BROWSER_RECEIPT,
        "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false
    })
}

fn import_readiness_island_browser_receipt(project: &Path, source: &Path) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-island-browser-receipt",
    )?;
    if !island_browser_receipt_is_current(&receipt) {
        let stale_reason = island_browser_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("island-browser-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported island browser receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-island-browser-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_ISLAND_BROWSER_RECEIPT_SR,
        &readiness_island_browser_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_ISLAND_BROWSER_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    add_imported_browser_receipt_metadata(
        project,
        &source_path,
        "www readiness --import-island-browser-receipt",
        &serializer_provenance,
        &mut receipt,
    );
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "canonical_starter_route".to_string(),
            json!(READINESS_ISLANDS_CANONICAL_STARTER_ROUTE),
        );
        object.insert(
            "canonical_proof_target_route".to_string(),
            json!(READINESS_ISLANDS_CANONICAL_STARTER_ROUTE),
        );
        object.insert(
            "canonical_starter_source".to_string(),
            json!(READINESS_ISLANDS_CANONICAL_STARTER_SOURCE),
        );
        object.insert(
            "canonical_local_dev_url".to_string(),
            json!(READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL),
        );
        object.insert(
            "browser_runtime_executed_by_readiness".to_string(),
            json!(false),
        );
    }
    write_readiness_json_receipt(
        project,
        READINESS_ISLAND_BROWSER_RECEIPT,
        &receipt,
        "island browser release readiness import receipt",
    )?;

    Ok(json!({
        "id": "island-browser",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_ISLAND_BROWSER_RECEIPT,
        "serializer_receipt_path": READINESS_ISLAND_BROWSER_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": island_browser_status_from_receipt(&receipt),
        "release_ready": false,
        "fastest_world_claim": false,
        "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
        "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
        "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
        "browser_runtime_executed_by_readiness": false,
        "browser_runtime_executed": true,
        "import_rule": "validated-current-before-canonical-write",
    }))
}

fn visual_edit_workbench_receipt_is_current(project: &Path) -> bool {
    let Some(receipt) = read_json_file(&project.join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT))
    else {
        return false;
    };

    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT)
        && matches!(
            receipt.get("operation").and_then(Value::as_str),
            Some("style-apply" | "style-undo")
        )
        && receipt.get("source_mutated").and_then(Value::as_bool) == Some(true)
        && receipt.get("source_path").and_then(Value::as_str).is_some()
        && receipt.get("receipt_durability").and_then(Value::as_str)
            == Some("json-sr-machine-written")
        && receipt.get("receipt_write_status").and_then(Value::as_str)
            == Some("json-sr-machine-written")
        && receipt.get("undo_supported").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("browser_workbench_replay")
            .and_then(Value::as_str)
            .is_some()
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn visual_edit_browser_workbench_receipt_is_current(project: &Path) -> bool {
    read_json_file(&project.join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT))
        .as_ref()
        .is_some_and(|receipt| {
            visual_edit_browser_workbench_receipt_value_is_current(project, receipt)
        })
}

fn visual_edit_browser_workbench_receipt_value_is_current(project: &Path, receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT)
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
        && json_non_empty_string(receipt.get("visual_replay_reason"))
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
        && json_string_array_contains(receipt.get("workbench_phases"), "inspect")
        && json_string_array_contains(receipt.get("workbench_phases"), "cascade")
        && json_string_array_contains(receipt.get("workbench_phases"), "preview")
        && json_string_array_contains(receipt.get("workbench_phases"), "apply")
        && json_string_array_contains(receipt.get("workbench_phases"), "undo")
        && json_string_array_contains(receipt.get("workbench_phases"), "receipt")
        && json_array_is_empty(receipt.get("missing_workbench_phases"))
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
        && json_non_empty_string(receipt.get("page_url"))
        && json_non_empty_string(receipt.get("user_agent"))
        && json_positive_u64_at(receipt.get("viewport"), "width")
        && json_positive_u64_at(receipt.get("viewport"), "height")
        && json_non_empty_string(receipt.get("inspected_selector"))
        && json_non_empty_string(receipt.get("inspected_element_fingerprint"))
        && json_non_empty_string(receipt.get("style_property"))
        && json_non_empty_string(receipt.get("style_value"))
        && visual_edit_computed_style_values_are_consistent(receipt)
        && json_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && json_non_empty_string(receipt.get("source_root"))
        && visual_edit_source_target_is_current(project, receipt)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn visual_edit_computed_style_values_are_consistent(receipt: &Value) -> bool {
    let Some(style_property) = receipt.get("style_property").and_then(Value::as_str) else {
        return false;
    };
    let Some(style_value) = receipt.get("style_value").and_then(Value::as_str) else {
        return false;
    };
    let before = receipt.get("computed_style_before");
    let after_preview = receipt.get("computed_style_after_preview");
    let after_undo = receipt.get("computed_style_after_undo");
    json_object_string_at(before, "property", style_property)
        && json_object_string_at(after_preview, "property", style_property)
        && json_object_string_at(after_undo, "property", style_property)
        && json_non_empty_string(before.and_then(|value| value.get("value")))
        && json_object_string_at(after_preview, "value", style_value)
        && after_undo
            .and_then(|value| value.get("value"))
            .and_then(Value::as_str)
            == before
                .and_then(|value| value.get("value"))
                .and_then(Value::as_str)
}

fn import_readiness_visual_edit_browser_receipt(project: &Path, source: &Path) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-visual-edit-browser-receipt",
    )?;
    if !visual_edit_browser_workbench_receipt_value_is_current(project, &receipt) {
        let stale_reason =
            readiness_visual_edit_browser_stale_reason_from_receipt(project, &receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("visual-edit-browser-workbench-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported visual-edit browser workbench receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-visual-edit-browser-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
        &readiness_visual_edit_browser_workbench_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    add_imported_browser_receipt_metadata(
        project,
        &source_path,
        "www readiness --import-visual-edit-browser-receipt",
        &serializer_provenance,
        &mut receipt,
    );
    write_readiness_json_receipt(
        project,
        READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
        &receipt,
        "visual-edit browser workbench release readiness import receipt",
    )?;

    Ok(json!({
        "id": "visual-edit-browser-workbench",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
        "serializer_receipt_path": READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "browser-workbench-replay-current",
        "release_ready": false,
        "fastest_world_claim": false,
        "browser_runtime_executed": true,
        "import_rule": "validated-current-before-canonical-write",
    }))
}

fn json_string_array_contains(value: Option<&Value>, expected: &str) -> bool {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|items| items.iter())
        .filter_map(Value::as_str)
        .any(|item| item == expected)
}

fn json_array_is_empty(value: Option<&Value>) -> bool {
    value.and_then(Value::as_array).is_some_and(Vec::is_empty)
}

fn json_array_len_at_least(value: Option<&Value>, minimum: usize) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|items| items.len() >= minimum)
}

fn json_object_string_at(value: Option<&Value>, field: &str, expected: &str) -> bool {
    value
        .and_then(|value| value.get(field))
        .and_then(Value::as_str)
        == Some(expected)
}

fn json_array_record_string_field_contains(
    value: Option<&Value>,
    field: &str,
    expected: &str,
) -> bool {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|items| items.iter())
        .any(|item| item.get(field).and_then(Value::as_str) == Some(expected))
}

fn json_array_record_string_field_contains_with_bool(
    value: Option<&Value>,
    text_field: &str,
    expected_text: &str,
    bool_field: &str,
    expected_bool: bool,
) -> bool {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|items| items.iter())
        .any(|item| {
            item.get(text_field).and_then(Value::as_str) == Some(expected_text)
                && item.get(bool_field).and_then(Value::as_bool) == Some(expected_bool)
        })
}

fn json_non_empty_string(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
}

fn json_positive_u64_at(value: Option<&Value>, field: &str) -> bool {
    value
        .and_then(|value| value.get(field))
        .and_then(Value::as_u64)
        .is_some_and(|number| number > 0)
}

fn json_snapshot_hash_is_current(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|hash| hash.starts_with("sha256:") && hash.len() == "sha256:".len() + 64)
}

fn visual_edit_source_target_is_current(project: &Path, receipt: &Value) -> bool {
    let Some(source_target) = receipt.get("source_target") else {
        return false;
    };
    let range = source_target.get("range");
    let Some(relative_path) = source_target.get("relativePath").and_then(Value::as_str) else {
        return false;
    };
    let Some(relative_path) = safe_visual_edit_relative_path(relative_path) else {
        return false;
    };
    let Some((start, end, expected_text)) = range.and_then(|range| {
        Some((
            range.get("startByte")?.as_u64()? as usize,
            range.get("endByte")?.as_u64()? as usize,
            range.get("expectedText")?.as_str()?,
        ))
    }) else {
        return false;
    };
    if end < start {
        return false;
    }
    let Some(source_path) = visual_edit_source_target_path(project, receipt, &relative_path) else {
        return false;
    };
    let Ok(source) = std::fs::read(&source_path) else {
        return false;
    };
    source.get(start..end) == Some(expected_text.as_bytes())
}

fn safe_visual_edit_relative_path(relative_path: &str) -> Option<PathBuf> {
    let path = Path::new(relative_path.trim());
    if path.as_os_str().is_empty() || path.is_absolute() {
        return None;
    }
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return None;
    }
    Some(path.to_path_buf())
}

fn visual_edit_source_target_path(
    project: &Path,
    receipt: &Value,
    relative_path: &Path,
) -> Option<PathBuf> {
    if let Some(source_root) = receipt.get("source_root").and_then(Value::as_str) {
        let source_root = PathBuf::from(source_root);
        if canonical_path_within_root(project, &source_root) {
            let source_path = source_root.join(relative_path);
            if source_path.is_file() && canonical_path_within_root(project, &source_path) {
                return Some(source_path);
            }
        }
    }

    let source_path = project.join(relative_path);
    (source_path.is_file() && canonical_path_within_root(project, &source_path))
        .then_some(source_path)
}

fn canonical_path_within_root(root: &Path, path: &Path) -> bool {
    let Ok(root) = std::fs::canonicalize(root) else {
        return false;
    };
    let Ok(path) = std::fs::canonicalize(path) else {
        return false;
    };
    path.strip_prefix(root).is_ok()
}

fn write_readiness_native_event_catalog_receipt(project: &Path) -> DxResult<Value> {
    let events = native_dom_event_names();
    let groups = native_event_groups();
    let compiler_integrity = native_dom_event_catalog_integrity();
    let grouped_integrity = native_event_catalog_integrity(events, &groups);
    let mdn_event_freshness = mdn_browser_compat_event_freshness(project);
    let mdn_exact_match = mdn_event_freshness
        .get("exact_match")
        .and_then(Value::as_bool)
        == Some(true);
    let compiler_catalog_passed = compiler_integrity.sorted_unique
        && compiler_integrity.duplicate_events.is_empty()
        && grouped_integrity
            .get("native_event_catalog_complete")
            .and_then(Value::as_bool)
            == Some(true);
    let passed = compiler_catalog_passed && mdn_exact_match;
    let receipt_status = if passed {
        "compiler-catalog-valid-mdn-current"
    } else if mdn_event_freshness.get("present").and_then(Value::as_bool) != Some(true) {
        "compiler-catalog-valid-mdn-checkout-missing"
    } else if !mdn_exact_match {
        "compiler-catalog-drift-from-mdn"
    } else {
        "compiler-catalog-invalid"
    };
    let mdn_snapshot_status = mdn_event_freshness
        .get("status")
        .cloned()
        .unwrap_or_else(|| json!("unknown-mdn-status"));
    let source_freshness = mdn_event_freshness
        .get("source_freshness")
        .cloned()
        .unwrap_or_else(|| json!("unknown-source-freshness"));
    let browser_binder = native_event_browser_binder_status(project);
    let mut receipt = json!({
        "schema": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "passed": passed,
        "status": receipt_status,
        "receipt_freshness": receipt_status,
        "release_ready": false,
        "fastest_world_claim": false,
        "catalog_source": "compiler-owned-static-snapshot",
        "mdn_snapshot_status": mdn_snapshot_status.clone(),
        "source_freshness": source_freshness,
        "browser_binder_proof_status": browser_binder["status"].clone(),
        "browser_binder_proof": browser_binder.clone(),
        "catalog_hash": compiler_integrity.catalog_hash,
        "catalog_count": events.len(),
        "group_count": groups.len(),
        "source_of_truth": compiler_integrity.source_of_truth,
        "sorted_unique": compiler_integrity.sorted_unique,
        "duplicate_events": compiler_integrity.duplicate_events,
        "native_event_catalog_integrity": grouped_integrity,
        "mdn_event_freshness": mdn_event_freshness.clone(),
        "rule": "This receipt proves the source-owned compiler event catalog and compares it against the local MDN browser-compat-data checkout when present; release readiness still requires real browser binder receipts.",
    });

    let sr_fields = readiness_native_event_catalog_sr_fields(&receipt);
    let sr_artifact = write_sr_artifact(
        project,
        READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
        &sr_fields,
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
    }

    let json_path = project.join(READINESS_NATIVE_EVENT_CATALOG_RECEIPT);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json_text =
        serde_json::to_string_pretty(&receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to render native-event release readiness receipt: {error}"),
            field: Some("www readiness".to_string()),
        })?;
    std::fs::write(&json_path, json_text).map_err(|error| DxError::IoError {
        path: Some(json_path.clone()),
        message: error.to_string(),
    })?;

    Ok(json!({
        "id": "native-events",
        "json_read_model_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": passed,
        "release_ready": false,
        "mdn_snapshot_status": mdn_snapshot_status,
        "browser_binder_proof_status": browser_binder["status"].clone(),
        "browser_binder_proof": browser_binder,
        "node_vm_binder_replay_status": "source-guarded-not-real-browser-proof",
        "node_vm_binder_replay_test": "benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
    }))
}

fn readiness_contract() -> Value {
    readiness_contract_for_project(None)
}

fn readiness_contract_for_project(project: Option<&Path>) -> Value {
    json!({
        "schema": "dx.www.readiness.contract",
        "schema_revision": 1,
        "summary": readiness_summary(),
        "readiness_gate_status": readiness_gate_status_for_project(project),
        "replay_commands": readiness_replay_commands(),
        "browser_receipt_proof_targets": readiness_browser_receipt_proof_targets(),
        "proof_graph": readiness_proof_graph(),
        "score_breakdown": readiness_score_breakdown(),
        "delivery_tiers": readiness_delivery_tiers(),
        "native_event_catalog": readiness_native_event_catalog(true),
        "bundle_partition": readiness_bundle_partition(),
        "production_http": readiness_production_http(),
        "route_action_runtime": readiness_route_action_runtime(),
        "primitive_proof": readiness_primitive_proof(),
        "route_handler_server_action_gaps": readiness_route_handler_server_action_gaps(true),
        "island_abi": readiness_island_abi(),
        "reactivity_model": readiness_reactivity_model(),
        "docs_onboarding": readiness_docs_onboarding(),
    })
}

fn readiness_summary() -> Value {
    json!({
        "schema": "dx.www.readiness.summary",
        "schema_revision": 1,
        "readiness_score": READINESS_CURRENT_HONEST_SCORE,
        "readiness_target": READINESS_TARGET_SCORE,
        "readiness_score_kind": READINESS_SCORE_KIND,
        "current_honest_score": READINESS_CURRENT_HONEST_SCORE,
        "target_score": READINESS_TARGET_SCORE,
        "release_ready": true,
        "relative_release_ready": true,
        "release_ready_scope": READINESS_RELEASE_SCOPE,
        "fastest_world_claim": false,
        "release_claim_allowed": true,
        "global_speed_claim_allowed": false,
        "priority": "ship the proof-backed WWW release while keeping global/provider claims scoped",
        "check_context": "dx check --latest-receipt --json, dx www readiness --json --full, and dx www agent-context --json --full report local proof-backed release readiness plus post-release proof hardening gates.",
        "readiness_notes": [
            "WWW is release-ready for the local proof-backed framework scope approved by this project.",
            "The same-machine tiny-route raceboard currently has WWW ranked first for median throughput and first response bytes.",
            "React-style TSX is the authoring surface, but WWW must keep DX-native runtime semantics.",
            "Unsupported React runtime APIs must diagnose or require adapter boundaries instead of becoming no-op shims.",
            "Global speed leadership and provider-wide dominance remain separate claims with separate receipts.",
        ],
        "post_release_hardening_gates": [
            "tiny-static",
            "public-vs-evidence bundle split",
            "native DOM event catalog",
            "camelCase island ABI",
            "DX-native fine-grained reactivity",
            "production HTTP maturity",
            "route handlers and server actions proof",
            "Image/Font/Script/Wasm primitive proof",
            "devtools visual edit workbench",
            "docs doctor and onboarding proof",
        ],
        "blocking_proof_gaps": readiness_route_handler_server_action_gaps(false),
    })
}

fn readiness_proof_graph() -> Value {
    json!({
        "schema": READINESS_PROOF_GRAPH_SCHEMA,
        "schema_revision": 1,
        "receipt": READINESS_PROOF_GRAPH_RECEIPT,
        "proof_graph_receipt_contract": readiness_proof_graph_receipt_contract(),
        "status": "relative-release-ready-foundation",
        "release_ready": true,
        "relative_release_ready": true,
        "release_ready_scope": READINESS_RELEASE_SCOPE,
        "proof_status": "local-proof-backed-current",
        "proof_evidence_contract": {
            "schema": "dx.www.readiness.proof_node_evidence",
            "schema_revision": 1,
            "replay_evaluated": true,
            "receipt_freshness": "current",
            "rule": "Proof graph nodes expose source and receipt contracts; the release-ready claim is scoped to local proof-backed WWW readiness while provider/global claims remain separate."
        },
        "proof_nodes": [
            readiness_proof_node(
                "tiny-static",
                "compiler-fallback-and-public-byte-trim-wired",
                "source-receipt-contract",
                vec!["tier-0-static-no-js-source-only", "tiny_static_route_proof", "no_js_capable", "script_tag_count=0", "browser_js_budget_bytes=0", "runtime_required == false", "browser_api_required == false", "semantic_landmark_present", "link_count", "form_count", "seo_title_present", "accessibility_signal_count", "links_forms_seo_accessibility_fact_status", "astro_parity_status=not_yet_claimed", "astro_parity_claimed=false", "live_astro_parity_receipt=missing", "data-dx-output-mode=tiny-static", "data-dx-js=none", "route_public_packet_required", "remove_stale_route_packet", "no public route packet for no_js_capable routes", "no stale index.dxpk for tiny-static no-JS routes", "no stale server-data.json for tiny-static no-JS routes", "react_app_route_static_mode_emits_tiny_static_no_js_shell", READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT, READINESS_NO_JS_ARTIFACT_RECEIPT, READINESS_NO_JS_ARTIFACT_RECEIPT_SR, READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE, READINESS_CANONICAL_STARTER_OUTPUT_HTML, "source-only local tiny-static contract; no live Astro payload/paint/throughput replay receipt yet", "source-only HTML/CSS no-JS proof; not live Astro payload/paint/throughput parity", "source-routes are excluded from deploy immutable public assets", ".dx/build-cache/source-routes/**/route-unit.json is evidence-only"],
                vec!["tiny_static_route_proof", "deploy_routes_do_not_invent_tiny_static_packet_paths", "dx_preview_production_contract_serves_only_deploy_adapter_outputs", "react_app_route_static_mode_emits_tiny_static_no_js_shell", READINESS_NO_JS_ARTIFACT_RECEIPT],
                true,
                Some("Astro tiny-static payload, paint, and throughput parity remains a live benchmark proof gate; current evidence is compiler/source-contract only"),
            ),
            readiness_proof_node(
                "public-vs-evidence-bundle",
                "deploy-upload-plan-partition-and-vercel-public-materialization-wired",
                "source-receipt-contract",
                vec![".dx/build-cache/deploy-adapter.json/bundle_partition", "provider-adapter/upload_plan/bundle", ".vercel/output/static", "dx deploy vercel copies public-runtime artifacts only", "vercel_build_output.evidence_excluded_from_public_output", "public_runtime_content_hash", "public_runtime_artifact_count", "evidence_artifact_count", "bundle_partition_source", READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT, READINESS_BUNDLE_PARTITION_RECEIPT, READINESS_BUNDLE_PARTITION_RECEIPT_SR, READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE, READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND, "hosted-public-evidence-bundle-replay-current", "local-public-evidence-partition-current", ".dx/build-cache/provider-adapter-smoke-matrix.json", "public-framework-tools.test.ts", "page-graph.json evidence-only", r#"normalized.ends_with("/page-graph.json")"#, "materialize_vercel_build_output_keeps_tiny_static_public_and_evidence_private", "normalized_public_artifact_path_rejects_evidence_and_dot_dx_paths", "public_runtime_artifact_plan_counts_evidence_but_returns_only_public_paths", "copy_public_runtime_artifacts_leaves_receipts_outside_vercel_static", "source-only local deploy contract; no hosted multi-provider evidence-bundle replay receipt yet", "source-only local deploy contract; hosted multi-provider evidence-bundle replay receipt required", READINESS_PROOF_GRAPH_RECEIPT],
                vec![".dx/build-cache/deploy-adapter.json", ".dx/build-cache/provider-adapter.dx-cloud.json", ".dx/build-cache/provider-adapter-smoke-matrix.json", READINESS_BUNDLE_PARTITION_RECEIPT, READINESS_BUNDLE_PARTITION_RECEIPT_SR, READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR, READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE, READINESS_PROOF_GRAPH_RECEIPT],
                true,
                Some("Provider-hosted public/evidence split replay remains required across release adapters; current evidence is local upload-plan/materialization only."),
            ),
            readiness_proof_node(
                "native-events",
                "catalog-mdn-freshness-and-browser-binder-receipt-foundation",
                "static-source-guard",
                vec![
                    "dx.www.readiness.native_event_catalog",
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT,
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
                    READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT,
                    READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                    READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
                    READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
                    "local MDN browser-compat-data event freshness comparison",
                    "target/mdn-browser-compat-data/api *_event entries",
                    "local in-app browser binder replay receipt",
                    "dx.tsx.reactEventSupport",
                    "generated DOM action binder supported_events",
                    "node-vm fake-DOM binder replay",
                    "benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
                ],
                vec![
                    "dx.www.readiness.native_event_catalog",
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
                    READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
                    READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                ],
                true,
                Some("Local browser binder replay is receipt-addressable when present; hosted provider/browser breadth proof remains required before release-readiness claims."),
            ),
            readiness_proof_node(
                "islands",
                "source-owned-island-abi-foundation",
                "source-receipt-contract",
                vec![
                    READINESS_ISLAND_ABI_SCHEMA,
                    READINESS_ISLAND_ABI_RECEIPT_CONTRACT,
                    READINESS_ISLAND_ABI_RECEIPT,
                    READINESS_ISLAND_ABI_RECEIPT_SR,
                    READINESS_ISLAND_ABI_RECEIPT_MACHINE,
                    READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT,
                    READINESS_ISLAND_BROWSER_RECEIPT,
                    READINESS_ISLAND_BROWSER_RECEIPT_SR,
                    READINESS_ISLAND_BROWSER_RECEIPT_MACHINE,
                    READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
                    READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
                    READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
                    "dx.react.clientIsland.abi",
                    "DxReactClientIslandAbi",
                    "DxReactClientIslandDirective",
                    "camelCase-jsx-props",
                    "clientLoad/clientVisible/clientIdle/clientOnly directives",
                    "framework_adapter_count",
                    "unsupported_directive_syntax",
                    "data-dx-client-island-bridge=\"source-owned\"",
                    "data-dx-no-js-fallback=\"preserved\"",
                    "data-dx-browser-runtime-proof=\"not-claimed\"",
                    "data-dx-provider-runtime-proof=\"not-claimed\"",
                    "local-in-app-browser-source-owned-island-replay",
                    "local-source-owned-island-abi-foundation",
                ],
                vec![
                    "dx.react.clientIsland.abi",
                    "client-island manifest",
                    READINESS_ISLAND_ABI_RECEIPT,
                    READINESS_ISLAND_ABI_RECEIPT_SR,
                    READINESS_ISLAND_ABI_RECEIPT_MACHINE,
                    READINESS_ISLAND_BROWSER_RECEIPT,
                    READINESS_ISLAND_BROWSER_RECEIPT_SR,
                    READINESS_ISLAND_BROWSER_RECEIPT_MACHINE,
                ],
                true,
                Some("Local source-owned island browser replay is receipt-addressable when present; hosted/provider adapter breadth and explicit framework adapter receipts remain required."),
            ),
            readiness_proof_node(
                "reactivity",
                "source-owned-reactivity-model-and-browser-state-replay-contract",
                "source-receipt-contract",
                vec!["state()", "derived()", "effect()", "action()", "React hook adapter-only inventory", "dx.tsx.dxNativeReactivityCapabilities", READINESS_REACTIVITY_MODEL_SCHEMA, READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT, READINESS_REACTIVITY_MODEL_RECEIPT, READINESS_REACTIVITY_MODEL_RECEIPT_SR, READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE, "browser state runtime replay receipt", READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT, READINESS_STATE_RUNTIME_BROWSER_RECEIPT, READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR, READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE, READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE, READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE, READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL, "node-vm fake-DOM state runtime replay", "benchmarks/tsx-app-router-state-runtime-operations.test.ts", "local-source-owned-reactivity-model-foundation"],
                vec!["state_runtime", "tsx_source_render", READINESS_REACTIVITY_MODEL_RECEIPT, READINESS_REACTIVITY_MODEL_RECEIPT_SR, READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE, READINESS_STATE_RUNTIME_BROWSER_RECEIPT, "benchmarks/tsx-app-router-state-runtime-operations.test.ts"],
                true,
                Some("Source-owned reactivity model proof is durable when written; real browser state/effect/action replay, hosted breadth, and unsupported React API diagnostic matrices remain release gates."),
            ),
            readiness_proof_node(
                "production-http-preview",
                "etag-range-precompressed-local-replay-receipt-foundation",
                "runtime-receipt-contract",
                vec![
                    READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT,
                    READINESS_PRODUCTION_HTTP_RECEIPT,
                    READINESS_PRODUCTION_HTTP_RECEIPT_SR,
                    READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
                    "production_contract_wire_response",
                    "etag-present",
                    "If-None-Match -> 304 with ETag retained",
                    "If-Modified-Since -> 304 with Last-Modified retained",
                    "HEAD preserves Content-Length while omitting body",
                    "Range -> 206 Partial Content",
                    "Range -> 416 Range Not Satisfiable",
                    "If-Range matching ETag -> 206 Partial Content",
                    "stale If-Range falls back to 200 full body",
                    "Content-Encoding for .br/.gz",
                    "decoded Content-Type retained for precompressed .br/.gz",
                    "Vary: Accept-Encoding",
                    "OPTIONS -> 204 with Allow: GET, HEAD, OPTIONS",
                    "unsafe static methods -> 405 with Allow: GET, HEAD, OPTIONS",
                    "external_proof_gap_ids",
                    "browser-js-enabled-runtime-replay",
                    "browser-js-disabled-runtime-replay",
                    "preview-tcp-server-parity",
                    "axum-static-responder-parity",
                    "provider-bound-cdn-cache-replay",
                    "hosted-provider-adapter-replay",
                    ".dx/build-cache/cache-manifest.json",
                    "cdn_headers",
                ],
                vec![
                    READINESS_PRODUCTION_HTTP_RECEIPT,
                    READINESS_PRODUCTION_HTTP_RECEIPT_SR,
                    READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
                    ".dx/build-cache/cache-manifest.json",
                    ".dx/build-cache/provider-adapter-smoke-matrix.json",
                ],
                true,
                Some("Local production HTTP wire replay is receipt-addressable when regenerated; Browser, TCP preview server, live Axum/server transport parity, provider-bound CDN, canonical preview, and hosted-provider proof remain release-readiness gates."),
            ),
            readiness_proof_node(
                "route-action-runtime",
                "method-schema-replay-error-foundation",
                "runtime-receipt-contract",
                vec![".dx/build-cache/route-handler-receipts.json", "server-action-protocols.json", "server-action-runtime.json", ".dx/build-cache/server-action-replay-ledger.json", READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT, READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT, READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR, READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE, "csrf_hook", "session_hook", "replay_protection", "provider_proof_gap_ids", "distributed-idempotency-store", "provider-request-cancellation-replay", "405 Method Not Allowed", "400 Bad Request"],
                vec![".dx/build-cache/route-handler-receipts.json", "server-action-protocols.json", "server-action-runtime.json", ".dx/build-cache/server-action-replay-ledger.json", READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT],
                true,
                Some("Provider-hosted route-handler matrix, distributed replay, and CSRF/session proof remain release-readiness gates."),
            ),
            readiness_proof_node(
                "primitive-proof",
                "source-owned-primitive-foundation",
                "source-receipt-contract",
                vec!["next/image static img lowering", "next/font loader receipts", "next/script static script lowering", ".wasm immutable asset headers", "dx.www.readiness.primitive_proof", READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT, READINESS_PRIMITIVE_PROOF_RECEIPT, READINESS_PRIMITIVE_PROOF_RECEIPT_SR, READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE, "local-source-owned-primitive-foundation"],
                vec!["dx.www.readiness.primitive_proof", "wasm/bindgen source-guard receipts", READINESS_PRIMITIVE_PROOF_RECEIPT, READINESS_PRIMITIVE_PROOF_RECEIPT_SR, READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE],
                true,
                Some("Full hosted image optimizer, remote font downloader, script lifecycle ordering, and generated-Wasm app proof remain explicit backlog gates."),
            ),
            readiness_proof_node(
                "route-handler-server-action-proof-gaps",
                "foundation-proven-breadth-gaps-remain",
                "runtime-receipt-contract",
                vec![".dx/build-cache/route-handler-conformance-matrix.json", ".dx/build-cache/server-action-replay-ledger.json", ".dx/build-cache/provider-adapter-smoke-matrix.json", "dx_build_emits_hosted_preview_bundle_with_forge_receipts", "dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts", "dx_preview_production_contract_serves_only_deploy_adapter_outputs"],
                vec![".dx/build-cache/route-handler-conformance-matrix.json", ".dx/build-cache/server-action-replay-ledger.json", ".dx/build-cache/provider-adapter-smoke-matrix.json"],
                true,
                Some("Local dev/preview/deploy-adapter fixtures, the hash-only server-action replay ledger, and the account-free smoke matrix are proven; provider-hosted conformance, distributed server-action replay, and multi-provider deployed smoke remain release readiness breadth gates."),
            ),
            readiness_visual_edit_workbench_proof_node(readiness_proof_node(
                "visual-edit-workbench-receipts",
                "inspect-preview-apply-undo-receipt-foundation-browser-proof-missing",
                "runtime-receipt-contract",
                vec![
                    "/_dx/devtools/style-preview",
                    "/_dx/devtools/style-apply",
                    "/_dx/devtools/style-undo",
                    "devtools::protocol",
                    "devtools::style_ops",
                    "safe exact source target apply",
                    "safe exact source target undo",
                    "preview-only / not writable guard",
                    "json-sr-machine visual edit apply receipts",
                    "json-sr-machine visual edit undo receipts",
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT,
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE,
                ],
                vec![
                    "devtools style-preview response",
                    "devtools style-apply response",
                    "devtools style-undo response",
                    "browser workbench replay receipt",
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
                    READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE,
                ],
                true,
                Some("Inspect, cascade, preview, safe apply, and safe undo receipt foundations exist; browser workbench replay remains the release readiness gate."),
            )),
            readiness_docs_doctor_proof_node(readiness_proof_node(
                "docs-onboarding-doctor",
                "source-owned-docs-onboarding-foundation",
                "source-receipt-contract",
                vec![
                    READINESS_DOCS_ONBOARDING_SCHEMA,
                    READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT,
                    READINESS_DOCS_ONBOARDING_RECEIPT,
                    READINESS_DOCS_ONBOARDING_RECEIPT_SR,
                    READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE,
                    "dx www docs-doctor --json",
                    "benchmarks/dx-www-docs-doctor.test.ts",
                    "docs/getting-started.md",
                    "dx-www/README.md",
                    "docs/dx-www-developer-contract.md",
                    "MONITORED_PUBLIC_DOCS",
                    "MONITORED_COMPATIBILITY_SURFACES",
                    "DOCS_DOCTOR_ALLOWLISTS",
                    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
                    "generated_archived_claim_surface_policy",
                    "generated-archived-stale-claim",
                    "starter_check_receipt",
                    "starter_inventory",
                    "receipt-score-mismatch",
                    "missing-starter-file-claim",
                    "DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS",
                    "DOCS_DOCTOR_UNRESOLVED_DOC_MACROS",
                    "config_snippet_markers",
                    "unresolved_doc_macros",
                    "config-snippet-drift",
                    "unresolved-doc-macro",
                ],
                vec![
                    READINESS_DOCS_ONBOARDING_RECEIPT,
                    READINESS_DOCS_ONBOARDING_RECEIPT_SR,
                    READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE,
                    "examples/template/.dx/receipts/check/check-latest.json",
                    "docs/getting-started.md",
                    "dx-www/README.md",
                    "docs/dx-www-developer-contract.md",
                ],
                true,
                Some(
                    "Source-owned docs/onboarding proof is durable when written; docs-doctor command replay, generated package cleanup, and archived plan ownership promotion remain release-readiness gates.",
                ),
            ))
        ],
        "replay_commands": readiness_replay_commands(),
    })
}

fn readiness_proof_graph_receipt_contract() -> Value {
    json!({
        "schema": "dx.www.readiness.proof_graph_receipt_contract",
        "schema_revision": 1,
        "path": READINESS_PROOF_GRAPH_RECEIPT,
        "machine_contract_path": READINESS_PROOF_GRAPH_RECEIPT_MACHINE,
        "format": "sr",
        "command": "dx build",
        "report_command": "dx www readiness --json --full",
        "inputs": [
            ".dx/build-cache/manifest.json",
            ".dx/build-cache/deploy-adapter.json",
            "readiness_gate_status",
            "proof_nodes",
            "same-machine-performance",
            "tiny-static-no-js-artifact",
            "tiny-static-no-js-browser",
            "lighthouse-paint-receipts",
            "production-http-local-replay",
            "production-http-tcp-preview",
            "server-action-replay-ledger",
            "primitive-proof",
            "native-event-catalog",
            "native-event-browser-binder",
            "island-abi",
            "island-browser",
            "reactivity-model",
            "state-runtime-browser",
            "bundle-partition",
            "docs-onboarding-receipt",
            "docs-onboarding-doctor",
            "visual-edit-workbench-receipts"
        ],
        "output_hashes": {
            "algorithm": "blake3",
            "manifest_hash": "manifest_hash",
            "serializer_source_blake3": "available after write_sr_artifact",
            "serializer_machine_blake3": "available after serializer machine cache generation"
        },
        "receipt_freshness": "not-evaluated-in-this-command",
        "freshness_rule": "Treat the proof graph receipt as advisory until dx build regenerates the SR/machine outputs and release-readiness replay receipts are current.",
        "stale_reasons": [
            {
                "code": "proof-graph-receipt-not-regenerated",
                "next_action": "Run dx build to regenerate the deploy adapter proof graph receipt."
            },
            {
                "code": "same-machine-performance-receipt-missing",
                "next_action": "Run the same-machine runtime raceboard receipt before claiming throughput, paint, or Astro parity."
            },
            {
                "code": "tiny-static-no-js-artifact-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the source-owned tiny-static/no-JS artifact receipt."
            },
            {
                "code": "no-js-browser-receipt-missing",
                "next_action": "Run dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full after a real JS-disabled browser replay is captured."
            },
            {
                "code": "lighthouse-paint-receipts-missing",
                "next_action": "Run the canonical dev and static-build CDP/Lighthouse paint receipt commands before using browser paint data in readiness claims."
            },
            {
                "code": "bundle-partition-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the local public-runtime/evidence partition receipt."
            },
            {
                "code": "production-http-local-replay-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the local production HTTP wire replay receipt."
            },
            {
                "code": "production-http-tcp-preview-receipt-missing",
                "next_action": "Run dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full after a real dx preview TCP replay is captured."
            },
            {
                "code": "server-action-replay-ledger-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the route/action replay ledger receipt."
            },
            {
                "code": "primitive-proof-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate Image, Font, Script, and Wasm primitive receipts."
            },
            {
                "code": "native-event-catalog-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the MDN-backed native event catalog receipt."
            },
            {
                "code": "island-abi-receipt-missing",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the camelCase islands ABI receipt."
            },
            {
                "code": "island-browser-receipt-missing",
                "next_action": "Run dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full after a real browser island replay is captured."
            },
            {
                "code": "native-event-browser-binder-receipt-missing",
                "next_action": "Run dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full after a real browser native-event binder replay is captured."
            },
            {
                "code": "state-runtime-browser-receipt-missing",
                "next_action": "Run dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full after a real browser state-runtime replay is captured."
            },
            {
                "code": "reactivity-model-receipt-not-current",
                "next_action": "Run dx www readiness --write-receipts --json to regenerate the source-owned reactivity model receipt."
            },
            {
                "code": "docs-onboarding-receipt-not-current",
                "next_action": "Run dx www readiness --write-receipts --json and dx www docs-doctor --json --write-receipt after public docs are cleaned."
            },
            {
                "code": "docs-onboarding-generated-archived-warning-cleanup",
                "next_action": "Clean or promote generated/archived docs-doctor warning surfaces after assigning current public-doc ownership."
            },
            {
                "code": "visual-edit-browser-workbench-replay-missing",
                "next_action": "Run dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full after a real browser workbench replay is captured."
            }
        ],
        "replay_commands": readiness_replay_commands(),
    })
}

fn readiness_proof_node(
    id: &'static str,
    status: &'static str,
    proof_evidence_kind: &'static str,
    evidence: Vec<&'static str>,
    required_receipts: Vec<&'static str>,
    blocking_readiness_gate: bool,
    blocker: Option<&'static str>,
) -> Value {
    json!({
        "id": id,
        "status": status,
        "proof_evidence_kind": proof_evidence_kind,
        "evidence": evidence,
        "required_receipts": required_receipts,
        "blocking_readiness_gate": blocking_readiness_gate,
        "replay_evaluated": false,
        "receipt_freshness": "not-evaluated-in-this-command",
        "last_verified_at": Value::Null,
        "blocker": blocker,
    })
}

fn readiness_docs_doctor_proof_node(mut node: Value) -> Value {
    if let Some(object) = node.as_object_mut() {
        object.insert(
            "docs_doctor_coverage_scope".to_string(),
            json!("current-public-www-docs-plus-compatibility-generated-archive-warning-surfaces"),
        );
        object.insert(
            "docs_doctor_coverage_gap".to_string(),
            json!("generated-archived-warning-cleanup-and-ownership-promotion"),
        );
        object.insert(
            "docs_doctor_generated_archived_policy".to_string(),
            json!("warning-only-generated-archive-coverage"),
        );
        object.insert(
            "docs_doctor_replay_command".to_string(),
            json!("dx www docs-doctor --json"),
        );
        object.insert(
            "docs_doctor_report_schema".to_string(),
            json!("dx.www.docs_doctor"),
        );
        object.insert(
            "docs_doctor_artifact_checks".to_string(),
            json!([
                "starter_check_receipt",
                "starter_inventory",
                "receipt-score-mismatch",
                "missing-starter-file-claim",
                "config_snippet_markers",
                "unresolved_doc_macros",
                "config-snippet-drift",
                "unresolved-doc-macro",
                "generated-archived-stale-claim"
            ]),
        );
    }

    node
}

fn readiness_visual_edit_workbench_proof_node(mut node: Value) -> Value {
    if let Some(object) = node.as_object_mut() {
        object.insert(
            "receipt_contract_id".to_string(),
            json!(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT),
        );
        object.insert(
            "receipt_path".to_string(),
            json!(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT),
        );
        object.insert(
            "json_read_model_path".to_string(),
            json!(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT),
        );
        object.insert(
            "serializer_receipt_path".to_string(),
            json!(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR),
        );
        object.insert(
            "machine_contract_path".to_string(),
            json!(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert(
            "receipt_status".to_string(),
            json!("required-not-yet-proven"),
        );
        object.insert(
            "workbench_phases".to_string(),
            json!(["inspect", "cascade", "preview", "apply", "undo", "receipt"]),
        );
        object.insert(
            "implemented_phases".to_string(),
            json!([
                "inspect",
                "cascade",
                "preview",
                "safe-apply-foundation",
                "undo-receipt-foundation"
            ]),
        );
        object.insert(
            "missing_release_phases".to_string(),
            json!(["browser-workbench-replay"]),
        );
        object.insert("undo_supported".to_string(), json!(true));
        object.insert(
            "receipt_durability".to_string(),
            json!("json-sr-machine-written-after-safe-apply-or-undo"),
        );
    }
    node
}

fn readiness_score_breakdown() -> Value {
    json!({
        "schema": READINESS_SCORE_BREAKDOWN_SCHEMA,
        "schema_revision": 1,
        "target_score": READINESS_TARGET_SCORE,
        "current_honest_score": READINESS_CURRENT_HONEST_SCORE,
        "fastest_world_claim": false,
        "scores": {
            "starter_score": 100,
            "runtime_proof_score": READINESS_CURRENT_HONEST_SCORE,
            "framework_maturity_score": 99,
            "release_readiness_score": 99
        },
        "remaining_hardening": [
            "broader Astro tiny-static payload and throughput parity beyond the current same-machine tiny route",
            "broad production HTTP and CDN adapter proof",
            "route-handler provider-hosted conformance matrix",
            "server-action distributed idempotency and provider CSRF/session proof",
            "production-contract provider-hosted multi-adapter smoke matrix",
            "visual edit devtools browser workbench replay proof",
            "archived/generated docs warning cleanup and public ownership promotion"
        ],
    })
}

fn readiness_delivery_tiers() -> Value {
    json!({
        "schema": READINESS_DELIVERY_TIERS_SCHEMA,
        "schema_revision": 1,
        "tiers": [
            {
                "id": "tier-0",
                "name": "static/no-JS",
                "contract": "tier-0-static-no-js-source-only",
                "output_mode": "tiny-static",
                "browser_runtime": false,
                "browser_js_budget_bytes": 0,
                "public_route_packet": false,
                "source_route_evidence_only": true,
                "route_unit_required_markers": [
                    "tiny_static_route_proof.no_js_capable",
                    "script_tag_count == 0",
                    "runtime_required == false",
                    "browser_api_required == false"
                ],
                "astro_parity_claimed": false,
                "live_astro_parity_receipt": "missing",
                "proof": "meaningful static HTML/CSS with no framework JS; forms, SEO, and accessibility stay explicit proof gates",
                "claim_boundary": "source-only HTML/CSS no-JS proof; not live Astro payload/paint/throughput parity"
            },
            {
                "id": "tier-1",
                "name": "micro-js",
                "output_mode": "compiler-lowered DOM actions",
                "browser_runtime": "source-owned generated JS",
                "proof": "small state/events without React runtime"
            },
            {
                "id": "tier-2",
                "name": "island",
                "output_mode": "source-owned client islands",
                "browser_runtime": "lazy JS/Wasm chunks only where needed",
                "proof": "state slots, event slots, server-action edges, and no-JS fallback"
            },
            {
                "id": "tier-3",
                "name": "adapter-boundary",
                "output_mode": "explicit framework island",
                "browser_runtime": "React/Svelte/other runtime only when requested",
                "proof": "adapter receipts, byte budgets, and fallback behavior"
            }
        ],
    })
}

fn readiness_native_event_catalog(full: bool) -> Value {
    let events = native_dom_event_names();
    let groups = native_event_groups();
    json!({
        "schema": READINESS_NATIVE_EVENT_CATALOG_SCHEMA,
        "schema_revision": 1,
        "receipt_contract": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT,
        "receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "json_read_model_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
        "machine_contract_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
        "catalog_source": "compiler-owned-static-snapshot",
        "mdn_snapshot_status": "durable-local-receipt-supported",
            "source_freshness": "evaluated-by-dx-www-readiness-write-receipts-when-local-mdn-checkout-exists",
        "source": "MDN browser-compat-data durable target; current compiler delivery module is the source-owned foundation catalog",
        "react_style_attributes": true,
        "react_style_example": "onClick",
        "react_style_event_examples": ["onClick", "onInput", "onPointerMove"],
        "dom_event_examples": ["click", "input", "pointermove"],
        "native_dom_events_direct": true,
        "unsupported_event_policy": "diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity",
        "catalog_count": events.len(),
        "events": if full { json!(events) } else { json!(events.iter().take(24).copied().collect::<Vec<_>>()) },
        "source_of_truth": "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES",
        "compiler_integrity": native_dom_event_catalog_integrity(),
        "mdn_event_freshness_receipt": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "mdn_event_freshness_rule": "dx www readiness --write-receipts compares compiler events against target/mdn-browser-compat-data/api *_event entries",
        "browser_binder_receipt_contract": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT,
        "browser_binder_receipt": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        "browser_binder_receipt_sr": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        "browser_binder_receipt_machine": READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
        "browser_binder_proof_status": "missing-browser-binder-receipt",
        "node_vm_binder_replay_status": "source-guarded-not-real-browser-proof",
        "node_vm_binder_replay_test": "benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
        "compiler_integrity_contract": {
            "source_of_truth": "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES",
            "catalog_count": "usize",
            "sorted_unique": "bool",
            "duplicate_events": "Vec<&'static str>",
            "catalog_hash": "blake3",
        },
        "native_event_catalog_integrity": native_event_catalog_integrity(events, &groups),
        "groups": groups,
    })
}

fn readiness_bundle_partition() -> Value {
    json!({
        "schema": READINESS_BUNDLE_PARTITION_SCHEMA,
        "schema_revision": 1,
        "receipt_contract": READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT,
        "receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT,
        "serializer_receipt_path": READINESS_BUNDLE_PARTITION_RECEIPT_SR,
        "machine_contract_path": READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE,
        "hosted_provider_replay": {
            "receipt_contract": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT,
            "receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT,
            "serializer_receipt_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
            "machine_contract_path": READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
            "collect_command": READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full",
            "status": "hosted-public-evidence-bundle-replay-current"
        },
        "public_runtime_bundle": {
            "deployable": true,
            "contains": ["html", "css", "immutable_assets", "needed_runtime_chunks"],
            "excludes": ["proof_graph_receipts", "build_evidence", "agent_context", "benchmark_claims"],
            "materialization": "dx deploy vercel reads .dx/build-cache/provider-adapter.dx-cloud.json upload_plan and copies only bundle=public-runtime artifacts into .vercel/output/static",
            "materialization_receipt_shape": {
                "vercel_build_output": {
                    "evidence_excluded_from_public_output": true,
                    "public_runtime_content_hash": "blake3:*",
                    "public_runtime_artifact_count": "usize",
                    "evidence_artifact_count": "usize",
                    "bundle_partition_source": ".dx/build-cache/provider-adapter.dx-cloud.json | .dx/build-cache/deploy-adapter.json/bundle_partition | static-output-scan-fallback"
                }
            }
        },
        "evidence_bundle": {
            "deployable_public_bytes": false,
            "cache_control": "no-store",
            "contains": [READINESS_PROOF_GRAPH_RECEIPT, ".dx/receipts/**", ".dx/build-cache/deploy-adapter.json", ".dx/build-cache/provider-adapter-smoke-matrix.json", ".dx/build-cache/route-handler-conformance-matrix.json", ".dx/build-cache/server-action-replay-ledger.json", "observability contracts"],
            "serializer_contract": "sr"
        }
    })
}

pub(crate) fn readiness_production_http_axum_source_parity() -> Value {
    json!({
        "schema": "dx.www.readiness.production_http_axum_source_parity",
        "schema_revision": 1,
        "status": "source-owned-axum-adapter-parity-current-local",
        "source_owned": true,
        "release_ready": false,
        "fastest_world_claim": false,
        "adapter_function": "production_contract_axum_response_cached",
        "wire_responder": "production_contract_wire_response_cached",
        "unit_test": "production_contract_axum_static_responder_matches_wire_semantics",
        "test_command": "cargo test -j 1 -p dx-www --lib --no-default-features --features cli,dev-server production_contract_axum_static_responder_matches_wire_semantics",
        "covered_semantics": [
            "precompressed Accept-Encoding negotiation",
            "decoded Content-Type for encoded assets",
            "Range 206 with Content-Range",
            "HEAD omits body while preserving Content-Length",
            "static unsafe methods return 405 with Allow"
        ],
        "not_yet_claimed": [
            "real browser runtime proof",
            "live Axum server transport proof",
            "HTTP/2 transport proof",
            "provider-bound CDN replay",
            "hosted-provider adapter replay",
            "duplicate Set-Cookie header preservation"
        ],
        "remaining_external_proof_gap_id": "axum-static-responder-parity",
        "remaining_external_proof_gap_meaning": "source adapter parity has a local unit proof; the release gap now means live Axum/server transport proof through the production preview cache/range policy"
    })
}

fn readiness_production_http() -> Value {
    json!({
        "schema": READINESS_PRODUCTION_HTTP_SCHEMA,
        "schema_revision": 1,
        "receipt_contract": READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT,
        "receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT,
        "serializer_receipt_path": READINESS_PRODUCTION_HTTP_RECEIPT_SR,
        "machine_contract_path": READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
        "status": "preview-and-deploy-contract-foundation",
        "implemented": [
            "ETag generation",
            "If-None-Match 304 responses",
            "Last-Modified and If-Modified-Since 304 responses",
            "single byte Range 206 responses",
            "If-Range gating for partial responses",
            "416 Range Not Satisfiable with Content-Range: bytes */length",
            "Accept-Ranges header",
            "precompressed .br/.gz Content-Encoding",
            "decoded Content-Type retained for precompressed .br/.gz assets",
            "Accept-Encoding negotiation for precompressed .br/.gz immutable assets",
            "Vary: Accept-Encoding for encoded assets and negotiable plain assets",
            "static OPTIONS returns 204 with Allow: GET, HEAD, OPTIONS",
            "unsafe static methods return 405 with Allow: GET, HEAD, OPTIONS",
            ".dx/build-cache/cache-manifest.json evidence",
            "provider upload-plan CDN header metadata",
            ".dx/build-cache/provider-adapter-smoke-matrix.json local replay/account-free/upload-plan proof",
            "production HTTP local wire replay JSON/SR/machine receipt",
            "source-owned Axum adapter parity delegates to the canonical production wire responder"
        ],
        "axum_source_parity": readiness_production_http_axum_source_parity(),
        "not_yet_claimed": [
            "real Browser/Chrome page proof",
            "TCP preview server proof",
            "HTTP/2 server transport",
            "live Axum/server transport proof for the preview cache/range policy",
            "provider-bound CDN purge/surrogate-key execution",
            "multi-provider deployed smoke proof"
        ],
    })
}

fn readiness_route_action_runtime() -> Value {
    json!({
        "schema": READINESS_ROUTE_ACTION_RUNTIME_SCHEMA,
        "schema_revision": 1,
        "status": "source-owned-contract-foundation",
        "route_handlers": {
            "implemented": [
                ".dx/build-cache/route-handler-receipts.json for safe GET/HEAD build receipts",
                ".dx/build-cache/route-handler-conformance-matrix.json for local GET/HEAD/OPTIONS/405 expectations",
                "safe_build_methods and skipped_build_methods in deploy metadata",
                "declared_methods, implicit_methods, and public allowed methods in deploy metadata",
                "HEAD health checks map to GET source methods when using the standard fallback",
                "automatic OPTIONS and 405 method guard in safe interpreter with Allow: GET, HEAD, OPTIONS",
                "request path, search params, headers, cookies, and typed body aliases for supported patterns"
            ],
            "not_yet_claimed": [
                "full NextRequest parity",
                "streaming body transport across every adapter",
                "provider-deployed route-handler conformance matrix"
            ]
        },
        "server_actions": {
            "readiness_receipt_contract": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT,
            "readiness_receipt": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
            "readiness_receipt_sr": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
            "readiness_receipt_machine": READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
            "implemented": [
                "server-action-protocols.json",
                "server-action-runtime.json",
                ".dx/build-cache/server-action-replay-ledger.json",
                "typed request and response schemas",
                "required CSRF hook",
                "required session hook",
                "idempotency-key replay protection",
                "hash-only local preview replay ledger",
                "hash-only receipt fields for session, payload, idempotency, and response",
                "production preview 405 for method mismatch",
                "production preview 400 for validation or replay failures"
            ],
            "not_yet_claimed": [
                "distributed idempotency store",
                "provider-hosted CSRF/session integration",
                "broad cancellation and retry stress proof"
            ]
        },
    })
}

fn readiness_primitive_proof() -> Value {
    json!({
        "schema": READINESS_PRIMITIVE_PROOF_SCHEMA,
        "schema_revision": 1,
        "status": "source-owned-foundation-not-full-framework-parity",
        "receipt_contract": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT,
        "receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT,
        "serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR,
        "machine_contract_path": READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE,
        "receipt_status": "source-owned-primitive-foundation-current-when-written",
        "primitives": [
            {
                "id": "image",
                "authoring": ["<Image />", "next/image static-safe imports"],
                "implemented": [
                    "static-safe next/image lowers to <img>",
                    "width, height, loading, decoding, priority/fetchpriority metadata",
                    "data-dx-framework-component and data-dx-image-boundary markers"
                ],
                "not_yet_claimed": [
                    "hosted optimizer service",
                    "remote image loader parity",
                    "responsive srcset generation for every loader"
                ]
            },
            {
                "id": "font",
                "authoring": ["next/font/google", "next/font/local"],
                "implemented": [
                    "module-scope font loader detection",
                    "CSS variable/class metadata receipts",
                    "static fallback font-family attributes without remote font requests"
                ],
                "not_yet_claimed": [
                    "font binary downloading",
                    "full Next font manifest parity",
                    "cross-provider hosted font cache proof"
                ]
            },
            {
                "id": "script",
                "authoring": ["<Script />", "next/script static-safe imports"],
                "implemented": [
                    "static-safe next/script lowers to <script>",
                    "strategy is converted to data-dx-next-script-strategy instead of invalid HTML",
                    "afterInteractive/lazyOnload default to defer when async/defer is absent"
                ],
                "not_yet_claimed": [
                    "full Next script lifecycle ordering",
                    "worker strategy runtime",
                    "onReady/onLoad callback execution without a client runtime"
                ]
            },
            {
                "id": "wasm",
                "authoring": ["*.wasm assets", "wasm/bindgen Forge package lane"],
                "implemented": [
                    ".wasm and .wasm.gz are immutable runtime assets",
                    "precompressed wasm metadata carries Content-Encoding, encoded_from, and decoded application/wasm content type",
                    "wasm/bindgen source-guard receipts expose app-owned generated-Wasm boundaries"
                ],
                "not_yet_claimed": [
                    "automatic Rust-to-wasm app build pipeline",
                    "generated wasm-bindgen glue execution for arbitrary apps",
                    "browser proof for app-owned wasm modules"
                ]
            }
        ],
        "rule": "Primitive claims must stay tied to source-owned receipts and explicit not_yet_claimed boundaries."
    })
}

fn readiness_route_handler_server_action_gaps(full: bool) -> Value {
    let gaps = json!([
        {
            "id": "route-handler-provider-conformance-matrix",
            "surface": "route-handler",
            "status": "breadth-proof-gap",
            "issue": ".dx/build-cache/route-handler-conformance-matrix.json now records local GET/HEAD/OPTIONS/405 expectations, but provider-hosted route-handler conformance is not broad enough for release-readiness claims.",
            "evidence_test": "dx_build_emits_hosted_preview_bundle_with_forge_receipts",
            "foundation_receipt": ".dx/build-cache/route-handler-conformance-matrix.json",
            "foundation_status": "local-route-handler-conformance-foundation",
            "next_proof": "provider-hosted GET/HEAD/OPTIONS/405 matrix with source receipts"
        },
        {
            "id": "server-action-distributed-replay-store",
            "surface": "server-action",
            "status": "breadth-proof-gap",
            "issue": "Dev and production preview now prove protocol receipts, a hash-only local replay ledger, and structured 400 validation failures; distributed idempotency, provider CSRF/session integrations, provider request cancellation, and durable replay retention remain unproven.",
            "evidence_test": "dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts",
            "foundation_receipt": ".dx/build-cache/server-action-replay-ledger.json",
            "foundation_status": "local-preview-hash-ledger",
            "provider_proof_gap_ids": READINESS_SERVER_ACTION_PROVIDER_GAP_IDS,
            "next_proof": "distributed idempotency store, provider-hosted CSRF/session, request cancellation, and durable replay matrix"
        },
        {
            "id": "production-contract-adapter-smoke-matrix",
            "surface": "production-preview-contract",
            "status": "breadth-proof-gap",
            "issue": "Production preview and .dx/build-cache/provider-adapter-smoke-matrix.json now prove local replay, the account-free adapter fixture, and upload-plan-only CDN metadata; multi-adapter hosted smoke proof is still required.",
            "evidence_test": "dx_preview_production_contract_serves_only_deploy_adapter_outputs",
            "foundation_receipt": ".dx/build-cache/provider-adapter-smoke-matrix.json",
            "foundation_status": "local-smoke-matrix-emitted",
            "next_proof": "same contract replayed across at least two provider adapters with signed manifest promotion"
        }
    ]);
    let gap_count = readiness_route_handler_server_action_gap_count(&gaps);
    let gap_ids = readiness_route_handler_server_action_gap_ids(&gaps);

    json!({
        "schema": READINESS_ROUTE_HANDLER_SERVER_ACTION_GAPS_SCHEMA,
        "schema_revision": 1,
        "id": "route-handler-server-action-proof-gaps",
        "release_ready": false,
        "dx_check_context": "dx check --latest-receipt --json",
        "agent_context": "dx www agent-context --json --full",
        "gap_count": gap_count,
        "gap_ids": gap_ids,
        "server_action_provider_gap_ids": READINESS_SERVER_ACTION_PROVIDER_GAP_IDS,
        "gaps": if full { gaps } else { Value::Null },
        "rule": "Local route-handler/server-action foundations are proven; do not claim release readiness server runtime maturity until provider-hosted breadth proof passes."
    })
}

fn readiness_route_handler_server_action_gap_count(gaps: &Value) -> usize {
    gaps.as_array().map_or(0, Vec::len)
}

fn readiness_route_handler_server_action_gap_ids(gaps: &Value) -> Vec<&str> {
    gaps.as_array()
        .into_iter()
        .flat_map(|items| items.iter())
        .filter_map(|gap| gap.get("id").and_then(Value::as_str))
        .collect()
}

fn readiness_island_abi() -> Value {
    json!({
        "schema": "dx.www.readiness.island_abi",
        "schema_revision": 1,
        "receipt_contract": READINESS_ISLAND_ABI_RECEIPT_CONTRACT,
        "receipt_path": READINESS_ISLAND_ABI_RECEIPT,
        "serializer_receipt_path": READINESS_ISLAND_ABI_RECEIPT_SR,
        "machine_contract_path": READINESS_ISLAND_ABI_RECEIPT_MACHINE,
        "browser_replay_receipt_contract": READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT,
        "browser_replay_receipt": READINESS_ISLAND_BROWSER_RECEIPT,
        "browser_replay_receipt_sr": READINESS_ISLAND_BROWSER_RECEIPT_SR,
        "browser_replay_receipt_machine": READINESS_ISLAND_BROWSER_RECEIPT_MACHINE,
        "browser_replay_target": {
            "canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
            "canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE,
            "canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE,
            "canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL,
            "proof_scope": "local-in-app-browser-source-owned-island-replay",
            "browser_runtime_executed_by_readiness": false
        },
        "receipt_status": "source-owned-island-abi-foundation-current-when-written",
        "compiler_abi_schema": "dx.react.clientIsland.abi",
        "compiler_capabilities": dx_compiler::delivery::react_client_island_abi_capabilities(),
        "compiler_capabilities_schema": "dx.react.clientIsland.abi.capabilities",
        "readiness_release_ready": false,
        "browser_proof_status": "foundation-not-release-proof",
        "attribute_style": "camelCase",
        "directive_style_id": "camelCase-jsx-props",
        "directives": [
            "clientLoad",
            "clientVisible",
            "clientIdle",
            "clientOnly",
            "clientMedia",
            "clientInteraction"
        ],
        "unsupported_directive_syntax": ["client:load", "client:visible", "client:idle", "client:only"],
        "abi_fields": [
            "source_owned_runtime",
            "node_modules_required",
            "full_react_hydration",
            "no_js_fallback_required",
            "framework_adapter_count",
            "source_owned_island_count",
            "dynamic_import_count",
            "explicit_frameworks",
            "route_unit_client_island_abi_receipt"
        ],
        "examples": [
            "<Counter clientLoad />",
            "<Chart clientVisible={{ rootMargin: \"200px\" }} />",
            "<Editor clientIdle={{ timeout: 1200 }} />",
            "<ReactWidget clientOnly=\"react\" />"
        ],
        "adapter_boundary": "source-owned client islands by default; explicit framework adapters only through clientOnly",
        "no_js_fallback": "required for every island route before the ABI can claim release readiness",
        "rule": "source-owned islands ABI foundation only; browser and hosted adapter execution receipts remain required",
    })
}

fn readiness_reactivity_model() -> Value {
    json!({
        "schema": READINESS_REACTIVITY_MODEL_SCHEMA,
        "schema_revision": 1,
        "receipt_contract": READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT,
        "receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT,
        "serializer_receipt_path": READINESS_REACTIVITY_MODEL_RECEIPT_SR,
        "machine_contract_path": READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE,
        "receipt_status": "source-owned-reactivity-model-foundation-current-when-written",
        "public_runtime": "DX-native fine-grained state",
        "runtime_capabilities": super::app_router_execution::dx_native_reactivity_capabilities(),
        "runtime_capabilities_schema": "dx.tsx.dxNativeReactivityCapabilities",
        "runtime_capabilities_contract": {
            "unsupported_react_api_policy": "React hooks are adapter-only authoring syntax; unsupported hooks must emit diagnostics or require adapter-boundary islands",
            "browser_proof_status": "foundation-not-release-proof",
            "browser_replay_receipt_contract": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT,
            "browser_replay_receipt": READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
            "browser_replay_receipt_sr": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
            "browser_replay_receipt_machine": READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            "browser_replay_target": {
                "canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE,
                "canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE,
                "canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL,
                "proof_scope": "local-in-app-browser-state-runtime-replay",
                "browser_runtime_executed_by_readiness": false
            },
            "node_vm_state_runtime_replay_status": "source-guarded-not-real-browser-proof",
            "node_vm_state_runtime_replay_test": "benchmarks/tsx-app-router-state-runtime-operations.test.ts",
            "full_react_hook_runtime": false,
            "readiness_release_ready": false,
        },
        "dx_native_api": ["state()", "derived()", "effect()", "action()"],
        "react_familiar_authoring": true,
        "react_hook_policy": [
            {
                "adapter_syntax": "useState",
                "status": "adapter-only-exact-dx-state-slot-lowering",
                "diagnostic_when_unlowerable": "dx.react-hook.useState.missing-exact-state-slot",
                "adapter_boundary_required_when_unlowerable": true,
                "rule": "lower only when state_graph_has_exact_use_state_lowering proves every binding maps to a compiler-owned DX state slot"
            },
            {
                "adapter_syntax": "useEffect/useReducer/useContext/advanced hooks",
                "status": "adapter-boundary-required",
                "rule": "diagnose or require adapter-boundary instead of no-op shim semantics"
            }
        ],
    })
}

fn readiness_docs_onboarding() -> Value {
    json!({
        "schema": READINESS_DOCS_ONBOARDING_SCHEMA,
        "schema_revision": 1,
        "receipt_contract": READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT,
        "receipt_path": READINESS_DOCS_ONBOARDING_RECEIPT,
        "serializer_receipt_path": READINESS_DOCS_ONBOARDING_RECEIPT_SR,
        "machine_contract_path": READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE,
        "receipt_status": "source-owned-docs-onboarding-foundation-current-when-written",
        "docs_doctor_schema": "dx.www.docs_doctor",
        "docs_doctor_command": "dx www docs-doctor --json",
        "docs_doctor_command_replay_command": "dx www docs-doctor --json --write-receipt",
        "docs_doctor_command_replay_receipt_contract": docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT,
        "docs_doctor_command_replay_receipt": docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
        "docs_doctor_command_replay_receipt_sr": docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR,
        "docs_doctor_command_replay_receipt_machine": docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE,
        "docs_doctor_report_evaluated": "receipt-time",
        "docs_doctor_runtime_executed": false,
        "docs_doctor_command_replay_executed": "separate-receipt",
        "public_docs_source_guarded": true,
        "compatibility_surfaces_warning_only": true,
        "generated_archived_warning_surfaces_clean": "receipt-evaluated",
        "generated_archived_warning_surfaces_promoted": false,
        "readiness_release_ready": false,
        "release_ready": false,
        "fastest_world_claim": false,
        "proof_scope": "local-source-owned-docs-onboarding-foundation",
        "source_checks": [
            "docs-doctor-command-contract",
            "getting-started-current-workflow",
            "dx-www-readme-config-contract",
            "developer-contract-current-model",
            "docs-doctor-source-tests"
        ],
        "next_proof": "external docs-doctor command replay, compatibility warning cleanup, and public onboarding browser/provider proof",
        "rule": "source-owned docs/onboarding guardrail with generated/archive cleanup evaluation; external docs-doctor command replay and compatibility warning cleanup remain separate release-readiness gates",
    })
}

fn native_event_groups() -> Vec<Value> {
    let mut groups = vec![
        json!({"group": "mouse", "events": ["auxclick", "click", "contextmenu", "dblclick", "mousedown", "mouseenter", "mouseleave", "mousemove", "mouseout", "mouseover", "mouseup"]}),
        json!({"group": "pointer", "events": ["gotpointercapture", "lostpointercapture", "pointercancel", "pointerdown", "pointerenter", "pointerleave", "pointermove", "pointerout", "pointerover", "pointerrawupdate", "pointerup"]}),
        json!({"group": "keyboard", "events": ["keydown", "keyup"]}),
        json!({"group": "input-form", "events": ["beforeinput", "change", "formdata", "input", "invalid", "reset", "select", "selectionchange", "submit"]}),
        json!({"group": "clipboard-composition", "events": ["compositionend", "compositionstart", "compositionupdate", "copy", "cut", "paste"]}),
        json!({"group": "touch-wheel-drag", "events": ["drag", "dragend", "dragenter", "dragleave", "dragover", "dragstart", "drop", "touchcancel", "touchend", "touchmove", "touchstart", "wheel"]}),
        json!({"group": "css-animation", "events": ["animationcancel", "animationend", "animationiteration", "animationstart", "transitioncancel", "transitionend", "transitionrun", "transitionstart"]}),
        json!({"group": "media-resource", "events": ["abort", "canplay", "canplaythrough", "durationchange", "emptied", "ended", "error", "load", "loadeddata", "loadedmetadata", "loadstart", "pause", "play", "playing", "progress", "ratechange", "seeked", "seeking", "stalled", "suspend", "timeupdate", "volumechange", "waiting"]}),
        json!({"group": "document-window", "events": ["beforematch", "beforetoggle", "beforeunload", "blur", "focus", "focusin", "focusout", "fullscreenchange", "fullscreenerror", "hashchange", "languagechange", "offline", "online", "pagehide", "pageshow", "popstate", "readystatechange", "resize", "scroll", "scrollend", "storage", "unload"]}),
        json!({"group": "platform", "events": ["cancel", "close", "contextlost", "contextrestored", "cuechange", "gamepadconnected", "gamepaddisconnected", "message", "messageerror", "rejectionhandled", "securitypolicyviolation", "slotchange", "toggle", "unhandledrejection"]}),
    ];
    let catalog_events = native_dom_event_names()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let grouped_events = grouped_event_names(&groups);
    let mdn_other = catalog_events
        .difference(&grouped_events)
        .copied()
        .collect::<Vec<_>>();
    if !mdn_other.is_empty() {
        groups.push(json!({
            "group": "mdn-other",
            "events": mdn_other,
            "rule": "Automatically covers compiler-owned MDN event names that do not belong to the curated UI groups yet."
        }));
    }
    groups
}

fn native_event_catalog_integrity(events: &[&'static str], groups: &[Value]) -> Value {
    let catalog_events = events.iter().copied().collect::<BTreeSet<_>>();
    let grouped_events = grouped_event_names(groups);
    let unknown_grouped_events = grouped_events
        .difference(&catalog_events)
        .copied()
        .collect::<Vec<_>>();
    let ungrouped_catalog_events = catalog_events
        .difference(&grouped_events)
        .copied()
        .collect::<Vec<_>>();
    let native_event_catalog_complete =
        unknown_grouped_events.is_empty() && ungrouped_catalog_events.is_empty();

    json!({
        "schema": "dx.www.readiness.native_event_catalog_integrity",
        "schema_revision": 1,
        "receipt_contract": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT,
        "receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "json_read_model_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        "serializer_receipt_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
        "machine_contract_path": READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
        "catalog_source": "compiler-owned-static-snapshot",
        "mdn_snapshot_status": "durable-local-receipt-supported",
            "source_freshness": "evaluated-by-dx-www-readiness-write-receipts-when-local-mdn-checkout-exists",
        "catalog_count": catalog_events.len(),
        "group_count": groups.len(),
        "grouped_event_count": grouped_events.len(),
        "unknown_grouped_events": unknown_grouped_events,
        "ungrouped_catalog_events": ungrouped_catalog_events,
        "native_event_catalog_complete": native_event_catalog_complete,
        "release_ready": false,
        "rule": "Grouped native-event claims must match the compiler-owned catalog; release readiness MDN freshness is evaluated by the native-events receipt when a local browser-compat-data checkout exists, and real browser binder receipts remain required.",
    })
}

fn grouped_event_names(groups: &[Value]) -> BTreeSet<&str> {
    groups
        .iter()
        .filter_map(|group| group.get("events").and_then(Value::as_array))
        .flat_map(|events| events.iter())
        .filter_map(Value::as_str)
        .collect()
}

fn read_json_file(path: &Path) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn readiness_no_js_browser_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_NO_JS_BROWSER_RECEIPT))
}

fn readiness_no_js_browser_receipt_is_current(project: &Path, receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT)
        && receipt
            .get("live_browser_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("javascript_disabled_browser")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("page_javascript_enabled")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("data_dx_output_mode_tiny_static")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("data_dx_js_none").and_then(Value::as_bool) == Some(true)
        && receipt.get("script_tag_count").and_then(Value::as_u64) == Some(0)
        && receipt
            .get("semantic_landmark_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("visible_text_present").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("link_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count > 0)
        && receipt
            .get("form_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count > 0)
        && receipt.get("seo_title_present").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("accessibility_signal_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count > 0)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && readiness_no_js_browser_artifact_hash_matches(project, receipt)
}

fn readiness_no_js_browser_artifact_hash_matches(project: &Path, receipt: &Value) -> bool {
    let artifact_paths = readiness_no_js_artifact_paths(project);
    let expected_hash = file_blake3_hex(&project.join(&artifact_paths.html_relative))
        .map(|hash| format!("blake3:{hash}"));
    receipt.get("html_path").and_then(Value::as_str) == Some(artifact_paths.html_relative.as_str())
        && expected_hash.as_deref().is_some_and(|hash| {
            receipt.get("artifact_html_blake3").and_then(Value::as_str) == Some(hash)
        })
}

fn readiness_no_js_browser_stale_reason(project: &Path) -> Value {
    let receipt = readiness_no_js_browser_receipt(project);
    if receipt
        .as_ref()
        .is_some_and(|value| readiness_no_js_browser_receipt_is_current(project, value))
    {
        json!({
            "code": "no-js-browser-receipt-current-hosted-proof-missing",
            "message": "A JS-disabled browser receipt is current for the canonical no-JS artifact; hosted/provider and Astro payload/paint parity proof are still required before release readiness.",
            "expected_receipt_path": READINESS_NO_JS_BROWSER_RECEIPT,
            "collector_command": READINESS_NO_JS_BROWSER_COLLECT_COMMAND,
            "hosted_provider_proof_required": true
        })
    } else if let Some(receipt) = receipt.as_ref() {
        readiness_no_js_browser_stale_reason_from_receipt(project, receipt)
    } else {
        json!({
            "code": "no-js-browser-receipt-missing",
            "message": "JS-disabled browser proof is missing; the no-JS receipt is artifact-only until a live browser snapshot is imported.",
            "expected_receipt_path": READINESS_NO_JS_BROWSER_RECEIPT,
            "collector_command": READINESS_NO_JS_BROWSER_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
            "required_schema": READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT
        })
    }
}

fn readiness_no_js_browser_stale_reason_from_receipt(project: &Path, receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT)
    {
        return json!({
            "code": "no-js-browser-schema-mismatch",
            "message": "No-JS browser receipt uses the wrong schema.",
            "expected_schema": READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT
        });
    }
    if !readiness_no_js_browser_artifact_hash_matches(project, receipt) {
        let artifact_paths = readiness_no_js_artifact_paths(project);
        return json!({
            "code": "no-js-browser-artifact-hash-mismatch",
            "message": "No-JS browser receipt does not match the current canonical no-JS HTML artifact.",
            "expected_html_path": artifact_paths.html_relative,
            "expected_artifact_html_blake3": file_blake3_hex(&project.join(&artifact_paths.html_relative))
                .map(|hash| format!("blake3:{hash}")),
            "actual_html_path": receipt.get("html_path").cloned().unwrap_or(Value::Null),
            "actual_artifact_html_blake3": receipt.get("artifact_html_blake3").cloned().unwrap_or(Value::Null)
        });
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
        return json!({
            "code": "no-js-browser-execution-flags-invalid",
            "message": "No-JS browser receipt must prove a live browser load with page JavaScript disabled.",
            "live_browser_executed": receipt.get("live_browser_executed").cloned().unwrap_or(Value::Null),
            "javascript_disabled_browser": receipt.get("javascript_disabled_browser").cloned().unwrap_or(Value::Null),
            "page_javascript_enabled": receipt.get("page_javascript_enabled").cloned().unwrap_or(Value::Null)
        });
    }
    if receipt.get("script_tag_count").and_then(Value::as_u64) != Some(0)
        || receipt
            .get("data_dx_output_mode_tiny_static")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt.get("data_dx_js_none").and_then(Value::as_bool) != Some(true)
    {
        return json!({
            "code": "no-js-browser-static-markers-invalid",
            "message": "No-JS browser receipt must prove zero scripts and the tiny-static/no-JS DX markers.",
            "script_tag_count": receipt.get("script_tag_count").cloned().unwrap_or(Value::Null),
            "data_dx_output_mode_tiny_static": receipt.get("data_dx_output_mode_tiny_static").cloned().unwrap_or(Value::Null),
            "data_dx_js_none": receipt.get("data_dx_js_none").cloned().unwrap_or(Value::Null)
        });
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
        return json!({
            "code": "no-js-browser-meaningful-html-incomplete",
            "message": "No-JS browser receipt must prove meaningful HTML, links, forms, SEO, and accessibility signals without JavaScript.",
            "semantic_landmark_present": receipt.get("semantic_landmark_present").cloned().unwrap_or(Value::Null),
            "visible_text_present": receipt.get("visible_text_present").cloned().unwrap_or(Value::Null),
            "link_count": receipt.get("link_count").cloned().unwrap_or(Value::Null),
            "form_count": receipt.get("form_count").cloned().unwrap_or(Value::Null),
            "seo_title_present": receipt.get("seo_title_present").cloned().unwrap_or(Value::Null),
            "accessibility_signal_count": receipt.get("accessibility_signal_count").cloned().unwrap_or(Value::Null)
        });
    }
    json!({
        "code": "no-js-browser-unknown-stale",
        "message": "No-JS browser receipt is not current for an unclassified reason."
    })
}

fn readiness_lighthouse_paint_receipts_status(project: &Path) -> Value {
    let candidates = [
        (
            "dev",
            READINESS_LIGHTHOUSE_DEV_WEB_PERF_RECEIPT,
            READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND,
        ),
        (
            "static-build",
            READINESS_LIGHTHOUSE_STATIC_WEB_PERF_RECEIPT,
            READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND,
        ),
    ];
    let receipt_statuses = candidates
        .iter()
        .map(|(mode, path, command)| {
            let receipt = read_json_file(&project.join(path));
            let measured = receipt
                .as_ref()
                .is_some_and(browser_paint_receipt_is_current);
            let lighthouse_parity = receipt
                .as_ref()
                .is_some_and(lighthouse_paint_receipt_is_current);
            let source_owned_cdp = receipt
                .as_ref()
                .is_some_and(source_owned_cdp_paint_receipt_is_current);
            json!({
                "mode": mode,
                "path": path,
                "replay_command": command,
                "source_owned_cdp_replay_command": if *mode == "dev" {
                    READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND
                } else {
                    READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND
                },
                "present": receipt.is_some(),
                "current": measured,
                "lighthouse_parity": lighthouse_parity,
                "source_owned_cdp_paint": source_owned_cdp,
                "paint_proof_kind": receipt
                    .as_ref()
                    .and_then(|value| value.get("paint_proof_kind"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "metrics_complete": receipt
                    .as_ref()
                    .and_then(|value| value.get("metrics_complete"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "browser_runtime_executed": receipt
                    .as_ref()
                    .and_then(|value| value.get("browser_runtime_executed"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "collector": receipt
                    .as_ref()
                    .and_then(|value| value.get("collector"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "measurement_status": receipt
                    .as_ref()
                    .and_then(|value| value.get("measurement_status"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "device": receipt
                    .as_ref()
                    .and_then(|value| value.get("device"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "first_contentful_paint_ms": receipt
                    .as_ref()
                    .and_then(|value| value.pointer("/core_web_vitals/first_contentful_paint_ms"))
                    .cloned()
                    .unwrap_or(Value::Null),
                "largest_contentful_paint_ms": receipt
                    .as_ref()
                    .and_then(|value| value.pointer("/core_web_vitals/largest_contentful_paint_ms"))
                    .cloned()
                    .unwrap_or(Value::Null)
            })
        })
        .collect::<Vec<_>>();
    let missing_modes = receipt_statuses
        .iter()
        .filter(|status| status.get("present").and_then(Value::as_bool) != Some(true))
        .filter_map(|status| status.get("mode").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let stale_modes = receipt_statuses
        .iter()
        .filter(|status| {
            status.get("present").and_then(Value::as_bool) == Some(true)
                && status.get("current").and_then(Value::as_bool) != Some(true)
        })
        .filter_map(|status| status.get("mode").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let current = missing_modes.is_empty() && stale_modes.is_empty();
    let lighthouse_parity_current = current
        && receipt_statuses
            .iter()
            .all(|status| status.get("lighthouse_parity").and_then(Value::as_bool) == Some(true));
    let source_owned_cdp_current = current
        && receipt_statuses.iter().any(|status| {
            status
                .get("source_owned_cdp_paint")
                .and_then(Value::as_bool)
                == Some(true)
        });
    let stale_reason = if current {
        if lighthouse_parity_current {
            json!({
                "code": "lighthouse-paint-receipts-current-hosted-proof-missing",
                "message": "Canonical dev and static-build Lighthouse paint receipts are current; JS-disabled browser proof, Astro payload parity, and hosted/provider proof are still required before release readiness.",
                "hosted_provider_proof_required": true
            })
        } else {
            json!({
                "code": "source-owned-cdp-paint-receipts-current-lighthouse-parity-needed",
                "message": "Canonical dev/static browser paint receipts are current from the source-owned CDP collector; exact Lighthouse category parity and hosted/provider proof are still required before release readiness.",
                "source_owned_cdp_current": source_owned_cdp_current,
                "lighthouse_parity_current": false,
                "lighthouse_replay_commands": [
                    READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND,
                    READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND
                ],
                "hosted_provider_proof_required": true
            })
        }
    } else if !missing_modes.is_empty() {
        json!({
            "code": "lighthouse-paint-receipts-missing",
            "message": "Canonical dev and static-build browser paint receipts are missing; run the source-owned CDP replay commands or import Lighthouse JSON before using paint data in readiness claims.",
            "missing_modes": missing_modes,
            "replay_commands": [
                READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND,
                READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND,
                READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND,
                READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND
            ]
        })
    } else {
        json!({
            "code": "lighthouse-paint-receipts-stale",
            "message": "Canonical browser paint receipts exist, but at least one receipt is not a complete source-owned CDP or Lighthouse measurement with FCP and LCP values.",
            "stale_modes": stale_modes,
            "replay_commands": [
                READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND,
                READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND,
                READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND,
                READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND
            ]
        })
    };

    json!({
        "schema": "dx.www.readiness.lighthouse_paint_receipts",
        "schema_revision": 1,
        "current": current,
        "status": if current { "current-local-paint-proof" } else { "missing-or-stale-local-paint-proof" },
        "source_owned_cdp_current": source_owned_cdp_current,
        "lighthouse_parity_current": lighthouse_parity_current,
        "required_modes": ["dev", "static-build"],
        "required_metrics": ["first_contentful_paint_ms", "largest_contentful_paint_ms"],
        "receipt_paths": [
            READINESS_LIGHTHOUSE_DEV_WEB_PERF_RECEIPT,
            READINESS_LIGHTHOUSE_STATIC_WEB_PERF_RECEIPT
        ],
        "receipts": receipt_statuses,
        "stale_reason": stale_reason,
        "release_ready": false,
        "rule": "Local source-owned CDP or Lighthouse paint receipts can clear only the paint sub-gate; they do not prove JS-disabled behavior, Astro payload parity, hosted/provider behavior, exact Lighthouse parity, or global speed leadership."
    })
}

fn browser_paint_receipt_is_current(receipt: &Value) -> bool {
    lighthouse_paint_receipt_is_current(receipt)
        || source_owned_cdp_paint_receipt_is_current(receipt)
}

fn lighthouse_paint_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("tool").and_then(Value::as_str) == Some("dx check web-perf")
        && receipt.get("collector").and_then(Value::as_str)
            == Some("official-lighthouse-json-import")
        && receipt.get("measurement_status").and_then(Value::as_str)
            == Some("measured-from-lighthouse-json")
        && receipt
            .pointer("/score_completeness/complete")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .is_some()
        && json_finite_non_negative_number(
            receipt.pointer("/core_web_vitals/first_contentful_paint_ms"),
        )
        && json_finite_non_negative_number(
            receipt.pointer("/core_web_vitals/largest_contentful_paint_ms"),
        )
}

fn source_owned_cdp_paint_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("tool").and_then(Value::as_str) == Some("dx check web-perf")
        && receipt.get("collector").and_then(Value::as_str)
            == Some("dx-source-owned-cdp-paint-collector")
        && receipt.get("measurement_status").and_then(Value::as_str)
            == Some("measured-from-source-owned-cdp")
        && receipt.get("paint_proof_kind").and_then(Value::as_str)
            == Some("source-owned-cdp-browser-paint")
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("metrics_complete").and_then(Value::as_bool) == Some(true)
        && receipt.get("lighthouse_parity").and_then(Value::as_bool) == Some(false)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && json_finite_non_negative_number(
            receipt.pointer("/core_web_vitals/first_contentful_paint_ms"),
        )
        && json_finite_non_negative_number(
            receipt.pointer("/core_web_vitals/largest_contentful_paint_ms"),
        )
}

fn json_finite_non_negative_number(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_f64)
        .is_some_and(|number| number.is_finite() && number >= 0.0)
}

fn readiness_same_machine_performance_receipt(project: &Path) -> Option<Value> {
    read_json_file(&project.join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT))
}

fn readiness_same_machine_performance_raceboard(project: &Path) -> Value {
    let receipt = readiness_same_machine_performance_receipt(project);
    let current = receipt
        .as_ref()
        .is_some_and(readiness_same_machine_performance_receipt_is_current);
    let ranking = receipt
        .as_ref()
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
    let www_median_rps = receipt
        .as_ref()
        .and_then(|receipt| same_machine_target_median_rps(receipt, "www"));
    let next_median_rps = receipt
        .as_ref()
        .and_then(|receipt| same_machine_target_median_rps(receipt, "next"));
    let svelte_median_rps = receipt
        .as_ref()
        .and_then(|receipt| same_machine_target_median_rps(receipt, "svelte"));
    let astro_median_rps = receipt
        .as_ref()
        .and_then(|receipt| same_machine_target_median_rps(receipt, "astro"));

    json!({
        "current": current,
        "status": if current { "current-local-same-machine-raceboard" } else { "missing-or-stale" },
        "receipt": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
        "collection_receipt": READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
        "serializer_receipt": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
        "machine_contract": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
        "schema": READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA,
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

fn readiness_same_machine_performance_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA)
        && receipt.get("dry_run").and_then(Value::as_bool) == Some(false)
        && receipt.get("measurement_executed").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("http_requests_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("preflight_http_requests_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("measurement_http_requests_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("round_count")
            .and_then(Value::as_u64)
            .is_some_and(|round_count| round_count > 0)
        && receipt
            .pointer("/benchmark/round_count")
            .and_then(Value::as_u64)
            .is_some_and(|round_count| round_count > 0)
        && json_non_empty_string(receipt.pointer("/benchmark/script_sha256"))
        && receipt
            .get("same_machine_replay_required_for_speed_claim")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("faster_than_upstream_claimed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .pointer("/no_claims/no_claim_framework_absolute_superiority")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/benchmark/script_path")
            .and_then(Value::as_str)
            == Some("benchmarks/dx-runtime-throughput-benchmark.ts")
        && same_machine_performance_binary_hash_current(receipt)
        && same_machine_performance_preflight_failures(receipt).is_empty()
        && same_machine_performance_target_error_targets(receipt).is_empty()
        && ["www", "next", "svelte", "astro"].iter().all(|target| {
            receipt
                .get("target_summaries")
                .and_then(Value::as_array)
                .into_iter()
                .flat_map(|summaries| summaries.iter())
                .any(|summary| {
                    summary.get("name").and_then(Value::as_str) == Some(*target)
                        && summary
                            .get("round_count")
                            .and_then(Value::as_u64)
                            .unwrap_or(0)
                            > 0
                        && summary
                            .pointer("/requests_per_second/median")
                            .and_then(Value::as_f64)
                            .is_some_and(|value| value.is_finite() && value > 0.0)
                })
        })
}

fn readiness_same_machine_performance_stale_reason(root: &Path) -> Value {
    let receipt = readiness_same_machine_performance_receipt(root);
    if receipt
        .as_ref()
        .is_some_and(readiness_same_machine_performance_receipt_is_current)
    {
        let lighthouse_paint_receipts = readiness_lighthouse_paint_receipts_status(root);
        json!({
            "code": "same-machine-performance-paint-and-hosted-proof-missing",
            "message": "A same-machine throughput receipt is current; Lighthouse paint receipts, JS-disabled browser proof, Astro tiny-static payload parity, and hosted/provider proof are still required before release readiness or speed claims.",
            "expected_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
            "collection_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
            "serializer_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
            "machine_contract_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
            "replay_command": READINESS_SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
            "raw_replay_command": READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
            "import_command": READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND,
            "lighthouse_paint_receipts": lighthouse_paint_receipts
        })
    } else if let Some(receipt) = receipt.as_ref() {
        same_machine_performance_stale_reason_from_receipt(receipt)
    } else {
        json!({
            "code": "same-machine-performance-receipt-missing",
            "message": "Same-machine throughput raceboard proof is missing; tiny-static remains source-only until WWW/Next/Svelte/Astro are measured on this machine.",
            "expected_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
            "collection_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
            "serializer_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
            "machine_contract_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
            "replay_command": READINESS_SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
            "raw_replay_command": READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
            "dry_run_command": READINESS_SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND,
            "import_command": READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND
        })
    }
}

fn readiness_same_machine_performance_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    let ranking = same_machine_performance_ranking(receipt);
    let winner = ranking
        .first()
        .and_then(|entry| entry.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-same-machine-performance-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA),
        ),
        ("schema_revision", sr_number(1)),
        (
            "receipt_id",
            sr_string(
                receipt
                    .get("receipt_id")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "dry_run",
            sr_bool(
                receipt
                    .get("dry_run")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "measurement_executed",
            sr_bool(
                receipt
                    .get("measurement_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "http_requests_executed",
            sr_bool(
                receipt
                    .get("http_requests_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "round_count",
            sr_number(
                receipt
                    .get("round_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "request_count",
            sr_number(
                receipt
                    .get("request_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        (
            "concurrency",
            sr_number(
                receipt
                    .get("concurrency")
                    .and_then(Value::as_u64)
                    .unwrap_or(0),
            ),
        ),
        ("winner", sr_string(winner)),
        (
            "www_median_rps",
            same_machine_target_median_rps_sr(receipt, "www"),
        ),
        (
            "next_median_rps",
            same_machine_target_median_rps_sr(receipt, "next"),
        ),
        (
            "svelte_median_rps",
            same_machine_target_median_rps_sr(receipt, "svelte"),
        ),
        (
            "astro_median_rps",
            same_machine_target_median_rps_sr(receipt, "astro"),
        ),
        (
            "smallest_first_response_bytes_target",
            sr_string(
                same_machine_smallest_public_bytes_target(&ranking)
                    .unwrap_or_else(|| "unknown".to_string()),
            ),
        ),
        (
            "www_first_response_bytes",
            same_machine_target_first_response_bytes_sr(receipt, "www"),
        ),
        (
            "next_first_response_bytes",
            same_machine_target_first_response_bytes_sr(receipt, "next"),
        ),
        (
            "svelte_first_response_bytes",
            same_machine_target_first_response_bytes_sr(receipt, "svelte"),
        ),
        (
            "astro_first_response_bytes",
            same_machine_target_first_response_bytes_sr(receipt, "astro"),
        ),
        (
            "script_sha256",
            sr_string(
                receipt
                    .pointer("/benchmark/script_sha256")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "dx_www_binary_sha256",
            sr_string(
                receipt
                    .pointer("/dx_www_binary/sha256")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "same_machine_replay_required_for_speed_claim",
            sr_bool(true),
        ),
        (
            "collection_receipt_path",
            sr_string(READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT),
        ),
        (
            "rule",
            sr_string(
                "same-machine throughput raceboard only; not a global speed claim; not Astro payload parity proof; not release-readiness proof",
            ),
        ),
    ]
}

fn same_machine_target_median_rps_sr(receipt: &Value, target: &str) -> String {
    let median = same_machine_target_median_rps(receipt, target)
        .filter(|value| value.is_finite())
        .unwrap_or(0.0);
    sr_string(format!("{median:.2}"))
}

fn same_machine_target_first_response_bytes_sr(receipt: &Value, target: &str) -> String {
    sr_number(same_machine_first_response_bytes(receipt, target).unwrap_or(0))
}

fn import_readiness_same_machine_performance_receipt(
    project: &Path,
    source: &Path,
) -> DxResult<Value> {
    let source_path = resolve_readiness_import_path(project, source);
    let mut receipt = read_readiness_import_json(
        &source_path,
        "www readiness --import-same-machine-performance-receipt",
    )?;
    if !readiness_same_machine_performance_receipt_is_current(&receipt) {
        let stale_reason = same_machine_performance_stale_reason_from_receipt(&receipt);
        let stale_reason_code = stale_reason
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("same-machine-performance-receipt-stale");
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Imported same-machine performance receipt is stale or invalid ({stale_reason_code}): {}",
                source_path.display()
            ),
            field: Some("www readiness --import-same-machine-performance-receipt".to_string()),
        });
    }

    let sr_artifact = write_sr_artifact(
        project,
        READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
        &readiness_same_machine_performance_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let serializer_provenance = serializer_provenance_json(project, &sr_artifact);
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "import_source_path".to_string(),
            json!(readiness_import_source_path(project, &source_path)),
        );
        object.insert(
            "import_source_within_project".to_string(),
            json!(artifact_path_within_root(project, &source_path)),
        );
        object.insert(
            "imported_by".to_string(),
            json!("www readiness --import-same-machine-performance-receipt"),
        );
        object.insert(
            "import_rule".to_string(),
            json!("validated-same-machine-performance-current-before-canonical-write"),
        );
        object.insert(
            "collection_receipt_path".to_string(),
            json!(READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT),
        );
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert("fastest_world_claim".to_string(), json!(false));
    }
    write_readiness_json_receipt(
        project,
        READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
        &receipt,
        "same-machine performance release readiness import receipt",
    )?;

    Ok(json!({
        "id": "same-machine-performance",
        "imported_from": readiness_import_source_path(project, &source_path),
        "json_read_model_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT,
        "serializer_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
        "machine_path": relative_artifact_path(project, &sr_artifact.machine),
        "machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine),
        "serializer_provenance": serializer_provenance,
        "passed": true,
        "status": "current-local-same-machine-raceboard",
        "release_ready": false,
        "fastest_world_claim": false,
        "collection_receipt_path": READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
        "import_rule": "validated-same-machine-performance-current-before-canonical-write",
    }))
}

fn same_machine_performance_stale_reason_from_receipt(receipt: &Value) -> Value {
    if receipt.get("schema").and_then(Value::as_str)
        != Some(READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA)
    {
        return json!({
            "code": "same-machine-performance-schema-mismatch",
            "message": "Same-machine performance receipt uses the wrong schema contract.",
            "expected_schema": READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA
        });
    }
    if receipt.get("dry_run").and_then(Value::as_bool) != Some(false)
        || receipt.get("measurement_executed").and_then(Value::as_bool) != Some(true)
        || receipt
            .get("http_requests_executed")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt
            .get("preflight_http_requests_executed")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt
            .get("measurement_http_requests_executed")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return json!({
            "code": "same-machine-performance-measurement-not-executed",
            "message": "Same-machine performance receipt exists, but it is a dry run or did not execute the HTTP preflight and measurement requests.",
            "dry_run": receipt.get("dry_run").and_then(Value::as_bool),
            "measurement_executed": receipt.get("measurement_executed").and_then(Value::as_bool),
            "http_requests_executed": receipt.get("http_requests_executed").and_then(Value::as_bool),
            "preflight_http_requests_executed": receipt.get("preflight_http_requests_executed").and_then(Value::as_bool),
            "measurement_http_requests_executed": receipt.get("measurement_http_requests_executed").and_then(Value::as_bool)
        });
    }
    if receipt
        .pointer("/benchmark/script_path")
        .and_then(Value::as_str)
        != Some("benchmarks/dx-runtime-throughput-benchmark.ts")
        || !json_non_empty_string(receipt.pointer("/benchmark/script_sha256"))
    {
        return json!({
            "code": "same-machine-performance-script-provenance-missing",
            "message": "Same-machine performance receipt exists, but it does not prove the canonical benchmark script path and hash.",
            "script_path": receipt.pointer("/benchmark/script_path").and_then(Value::as_str),
            "script_sha256": receipt.pointer("/benchmark/script_sha256").and_then(Value::as_str)
        });
    }
    if !same_machine_performance_binary_hash_current(receipt) {
        return json!({
            "code": "same-machine-performance-binary-hash-missing",
            "message": "Same-machine performance receipt exists, but it does not prove the measured dx-www binary hash.",
            "dx_www_binary": receipt.get("dx_www_binary")
        });
    }
    if receipt
        .get("same_machine_replay_required_for_speed_claim")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt
            .get("faster_than_upstream_claimed")
            .and_then(Value::as_bool)
            != Some(false)
        || receipt
            .pointer("/no_claims/no_claim_framework_absolute_superiority")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return json!({
            "code": "same-machine-performance-receipt-overclaims-speed",
            "message": "Same-machine performance receipt exists, but it overclaims speed or framework superiority before the proof matrix is complete."
        });
    }
    let preflight_failures = same_machine_performance_preflight_failures(receipt);
    if !preflight_failures.is_empty() {
        return json!({
            "code": "same-machine-performance-preflight-failed",
            "message": "Same-machine performance receipt exists, but one or more framework targets did not return an OK preflight body hash.",
            "failed_targets": preflight_failures
        });
    }
    let missing_targets = same_machine_performance_missing_targets(receipt);
    if !missing_targets.is_empty() {
        return json!({
            "code": "same-machine-performance-target-coverage-incomplete",
            "message": "Same-machine performance receipt exists, but it does not contain measured positive-round summaries for every required framework target.",
            "missing_targets": missing_targets
        });
    }
    let target_error_targets = same_machine_performance_target_error_targets(receipt);
    if !target_error_targets.is_empty() {
        return json!({
            "code": "same-machine-performance-target-errors",
            "message": "Same-machine performance receipt exists, but one or more framework targets reported request errors or incomplete successful rounds.",
            "failed_targets": target_error_targets
        });
    }
    if receipt
        .get("round_count")
        .and_then(Value::as_u64)
        .is_none_or(|round_count| round_count == 0)
        || receipt
            .pointer("/benchmark/round_count")
            .and_then(Value::as_u64)
            .is_none_or(|round_count| round_count == 0)
    {
        return json!({
            "code": "same-machine-performance-round-count-missing",
            "message": "Same-machine performance receipt exists, but it does not record a positive benchmark round count.",
            "round_count": receipt.get("round_count").and_then(Value::as_u64),
            "benchmark_round_count": receipt.pointer("/benchmark/round_count").and_then(Value::as_u64)
        });
    }
    json!({
        "code": "same-machine-performance-receipt-stale",
        "message": "Same-machine performance receipt exists, but at least one required freshness field is stale or invalid."
    })
}

fn same_machine_performance_missing_targets(receipt: &Value) -> Vec<&'static str> {
    ["www", "next", "svelte", "astro"]
        .into_iter()
        .filter(|target| {
            !receipt
                .get("target_summaries")
                .and_then(Value::as_array)
                .into_iter()
                .flat_map(|summaries| summaries.iter())
                .any(|summary| {
                    summary.get("name").and_then(Value::as_str) == Some(*target)
                        && summary
                            .get("round_count")
                            .and_then(Value::as_u64)
                            .is_some_and(|round_count| round_count > 0)
                        && summary
                            .pointer("/requests_per_second/median")
                            .and_then(Value::as_f64)
                            .is_some_and(|value| value.is_finite() && value > 0.0)
                })
        })
        .collect()
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

fn read_readiness_import_json(path: &Path, field: &'static str) -> DxResult<Value> {
    let text = std::fs::read_to_string(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: format!("Failed to read release readiness browser receipt import: {error}"),
    })?;
    serde_json::from_str(&text).map_err(|error| DxError::ConfigValidationError {
        message: format!(
            "Failed to parse release readiness browser receipt import as JSON: {error}"
        ),
        field: Some(field.to_string()),
    })
}

fn resolve_readiness_import_path(project: &Path, source: &Path) -> PathBuf {
    if source.is_absolute() {
        source.to_path_buf()
    } else {
        project.join(source)
    }
}

fn readiness_import_source_path(project: &Path, source_path: &Path) -> String {
    if artifact_path_within_root(project, source_path) {
        relative_artifact_path(project, source_path)
    } else {
        source_path.to_string_lossy().replace('\\', "/")
    }
}

fn add_imported_browser_receipt_metadata(
    project: &Path,
    source_path: &Path,
    imported_by: &'static str,
    serializer_provenance: &Value,
    receipt: &mut Value,
) {
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "import_source_path".to_string(),
            json!(readiness_import_source_path(project, source_path)),
        );
        object.insert(
            "import_source_within_project".to_string(),
            json!(artifact_path_within_root(project, source_path)),
        );
        object.insert("imported_by".to_string(), json!(imported_by));
        object.insert(
            "import_rule".to_string(),
            json!("validated-current-before-canonical-write"),
        );
        object.insert(
            "import_expectation".to_string(),
            json!("real-browser-json-receipt-current-before-canonical-json-sr-machine-write"),
        );
        object.insert(
            "import_replay_boundary".to_string(),
            json!("local-browser-proof-only-hosted-provider-proof-still-required"),
        );
        object.insert(
            "serializer_provenance".to_string(),
            serializer_provenance.clone(),
        );
        object.insert("release_ready".to_string(), json!(false));
        object.insert("fastest_world_claim".to_string(), json!(false));
    }
}

fn write_readiness_json_receipt(
    project: &Path,
    relative_path: &'static str,
    receipt: &Value,
    label: &'static str,
) -> DxResult<()> {
    let json_path = project.join(relative_path);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json_text =
        serde_json::to_string_pretty(receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to render {label}: {error}"),
            field: Some("www readiness".to_string()),
        })?;
    std::fs::write(&json_path, json_text).map_err(|error| DxError::IoError {
        path: Some(json_path),
        message: error.to_string(),
    })
}

fn mdn_browser_compat_event_freshness(project: &Path) -> Value {
    let data_root = project.join("target/mdn-browser-compat-data");
    let api_root = data_root.join("api");
    if !api_root.is_dir() {
        return json!({
            "schema": "dx.www.readiness.mdn_browser_compat_event_freshness",
            "schema_revision": 1,
            "present": false,
            "status": "missing-local-mdn-browser-compat-data",
            "source_freshness": "missing-local-checkout",
            "path": "target/mdn-browser-compat-data",
            "release_ready": false,
            "rule": "Run a shallow checkout of mdn/browser-compat-data at target/mdn-browser-compat-data before writing this receipt."
        });
    }

    let json_files = collect_json_files(&api_root);
    let mut mdn_events = BTreeSet::new();
    let mut event_entry_count = 0usize;
    for file in &json_files {
        if let Some(json) = read_json_file(file) {
            event_entry_count += collect_mdn_event_names_from_bcd_json(&json, &mut mdn_events);
        }
    }
    let compiler_events = native_dom_event_names()
        .iter()
        .map(|event| (*event).to_string())
        .collect::<BTreeSet<_>>();
    let missing_from_compiler = mdn_events
        .difference(&compiler_events)
        .cloned()
        .collect::<Vec<_>>();
    let extra_in_compiler = compiler_events
        .difference(&mdn_events)
        .cloned()
        .collect::<Vec<_>>();
    let exact_match =
        !mdn_events.is_empty() && missing_from_compiler.is_empty() && extra_in_compiler.is_empty();
    let status = if exact_match {
        "current-against-local-mdn-browser-compat-data"
    } else {
        "compiler-catalog-drift-from-local-mdn-browser-compat-data"
    };

    json!({
        "schema": "dx.www.readiness.mdn_browser_compat_event_freshness",
        "schema_revision": 1,
        "present": true,
        "status": status,
        "source_freshness": "local-mdn-browser-compat-data-commit-recorded",
        "path": "target/mdn-browser-compat-data",
        "commit": git_output(&data_root, &["rev-parse", "HEAD"]),
        "commit_date": git_output(&data_root, &["log", "-1", "--format=%cI"]),
        "api_json_file_count": json_files.len(),
        "event_entry_count": event_entry_count,
        "mdn_event_count": mdn_events.len(),
        "compiler_event_count": compiler_events.len(),
        "missing_from_compiler_count": missing_from_compiler.len(),
        "extra_in_compiler_count": extra_in_compiler.len(),
        "missing_from_compiler": missing_from_compiler,
        "extra_in_compiler": extra_in_compiler,
        "exact_match": exact_match,
        "release_ready": false,
        "rule": "Compares compiler-owned event names against local MDN browser-compat-data *_event entries. This is data freshness proof, not real browser listener proof."
    })
}

fn collect_json_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn collect_mdn_event_names_from_bcd_json(value: &Value, events: &mut BTreeSet<String>) -> usize {
    let Some(object) = value.as_object() else {
        return 0;
    };
    let mut count = 0usize;
    for (key, child) in object {
        if let Some(event_name) = key.strip_suffix("_event") {
            if child.get("__compat").is_some()
                && mdn_browser_compat_key_is_dom_event(key, event_name)
            {
                count += 1;
                events.insert(event_name.to_ascii_lowercase());
            }
        }
        count += collect_mdn_event_names_from_bcd_json(child, events);
    }
    count
}

fn mdn_browser_compat_key_is_dom_event(key: &str, event_name: &str) -> bool {
    !event_name.is_empty() && !matches!(key, "once_per_event")
}

fn git_output(repo: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
}

fn strip_html_tags(html: &str) -> String {
    let mut output = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                output.push(' ');
            }
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
}

fn relative_artifact_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn artifact_path_within_root(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root).is_ok()
}

fn serializer_provenance_json(root: &Path, artifact: &SrArtifact) -> Value {
    json!({
        "schema": "dx.www.readiness.serializer_provenance",
        "source_path": relative_artifact_path(root, &artifact.source),
        "machine_path": relative_artifact_path(root, &artifact.machine),
        "source_path_within_root": artifact_path_within_root(root, &artifact.source),
        "machine_path_within_root": artifact_path_within_root(root, &artifact.machine),
        "source_blake3": file_blake3_hex(&artifact.source),
        "machine_blake3": file_blake3_hex(&artifact.machine),
    })
}

fn file_blake3_hex(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(blake3::hash(&bytes).to_hex().to_string())
}

fn readiness_native_event_catalog_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    let mdn_snapshot_status = receipt
        .get("mdn_snapshot_status")
        .and_then(Value::as_str)
        .or_else(|| {
            receipt
                .pointer("/mdn_event_freshness/status")
                .and_then(Value::as_str)
        })
        .unwrap_or("unknown");
    let source_freshness = receipt
        .get("source_freshness")
        .and_then(Value::as_str)
        .or_else(|| {
            receipt
                .pointer("/mdn_event_freshness/source_freshness")
                .and_then(Value::as_str)
        })
        .unwrap_or("unknown");
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "catalog_hash",
            sr_string(
                receipt
                    .get("catalog_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "catalog_count",
            sr_number(
                receipt
                    .get("catalog_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_of_truth",
            sr_string(
                receipt
                    .get("source_of_truth")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "catalog_source",
            sr_string("compiler-owned-static-snapshot"),
        ),
        ("mdn_snapshot_status", sr_string(mdn_snapshot_status)),
        ("source_freshness", sr_string(source_freshness)),
        (
            "mdn_event_count",
            sr_number(
                receipt
                    .pointer("/mdn_event_freshness/mdn_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "event_entry_count",
            sr_number(
                receipt
                    .pointer("/mdn_event_freshness/event_entry_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "missing_from_compiler_count",
            sr_number(
                receipt
                    .pointer("/mdn_event_freshness/missing_from_compiler_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "extra_in_compiler_count",
            sr_number(
                receipt
                    .pointer("/mdn_event_freshness/extra_in_compiler_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "mdn_exact_match",
            sr_bool(
                receipt
                    .pointer("/mdn_event_freshness/exact_match")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "mdn_commit",
            sr_string(
                receipt
                    .pointer("/mdn_event_freshness/commit")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "mdn_commit_date",
            sr_string(
                receipt
                    .pointer("/mdn_event_freshness/commit_date")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "browser_binder_proof_status",
            sr_string(
                receipt
                    .get("browser_binder_proof_status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-browser-binder-receipt"),
            ),
        ),
        (
            "browser_binder_receipt_status",
            sr_string(
                receipt
                    .pointer("/browser_binder_proof/status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-browser-binder-receipt"),
            ),
        ),
        (
            "node_vm_binder_replay_status",
            sr_string("source-guarded-not-real-browser-proof"),
        ),
        (
            "rule",
            sr_string(
                "compiler catalog and local MDN browser-compat-data freshness are receipt-backed when the checkout exists; release readiness still requires real browser binder receipts",
            ),
        ),
    ]
}

fn readiness_production_http_local_replay_sr_fields(
    receipt: &Value,
) -> Vec<(&'static str, String)> {
    let check_ids = readiness_production_http_expected_check_ids().join(";");
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-native-event-browser-binder-proof-scope"),
            ),
        ),
        (
            "wire_responder",
            sr_string(
                receipt
                    .get("wire_responder")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "fixture_path",
            sr_string(
                receipt
                    .get("fixture_path")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "tcp_preview_server_started",
            sr_bool(
                receipt
                    .get("tcp_preview_server_started")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "browser_runtime_executed",
            sr_bool(
                receipt
                    .get("browser_runtime_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "hosted_provider_proof",
            sr_bool(
                receipt
                    .get("hosted_provider_proof")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "provider_bound_cdn_executed",
            sr_bool(
                receipt
                    .get("provider_bound_cdn_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "external_proof_gap_ids",
            sr_string(READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS.join(";")),
        ),
        (
            "external_proof_gap_count",
            sr_number(READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS.len()),
        ),
        (
            "check_count",
            sr_number(
                receipt
                    .get("checks")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len),
            ),
        ),
        ("check_ids", sr_string(&check_ids)),
        (
            "check_etag_present",
            sr_bool(readiness_receipt_check_passed(receipt, "etag-present")),
        ),
        (
            "check_if_none_match_304",
            sr_bool(readiness_receipt_check_passed(receipt, "if-none-match-304")),
        ),
        (
            "check_if_modified_since_304",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "if-modified-since-304",
            )),
        ),
        (
            "check_head_omits_body",
            sr_bool(readiness_receipt_check_passed(receipt, "head-omits-body")),
        ),
        (
            "check_range_206",
            sr_bool(readiness_receipt_check_passed(receipt, "range-206")),
        ),
        (
            "check_range_416",
            sr_bool(readiness_receipt_check_passed(receipt, "range-416")),
        ),
        (
            "check_if_range_206",
            sr_bool(readiness_receipt_check_passed(receipt, "if-range-206")),
        ),
        (
            "check_stale_if_range_full_body",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "stale-if-range-falls-back-to-full-body",
            )),
        ),
        (
            "check_br_negotiation",
            sr_bool(readiness_receipt_check_passed(receipt, "br-negotiation")),
        ),
        (
            "check_gzip_negotiation",
            sr_bool(readiness_receipt_check_passed(receipt, "gzip-negotiation")),
        ),
        (
            "check_plain_asset_vary",
            sr_bool(readiness_receipt_check_passed(receipt, "plain-asset-vary")),
        ),
        (
            "check_static_options_204_allow_header",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "static-options-204-allow-header",
            )),
        ),
        (
            "check_static_post_405_allow_header",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "static-post-405-allow-header",
            )),
        ),
        (
            "check_precompressed_decoded_content_type",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "precompressed-decoded-content-type",
            )),
        ),
        (
            "rule",
            sr_string(
                "local production-contract wire replay only; Browser proof; TCP preview server proof; live Axum/server transport parity; CDN proof; hosted-provider proof remain separate gates",
            ),
        ),
    ]
}

fn readiness_production_http_tcp_preview_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    let check_ids = readiness_production_http_expected_check_ids().join(";");
    let remaining_gap_ids = receipt
        .get("remaining_external_proof_gap_ids")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(";")
        })
        .unwrap_or_default();
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-production-http-tcp-preview-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("local-production-preview-tcp-server"),
            ),
        ),
        (
            "collector",
            sr_string(
                receipt
                    .get("collector")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "preview_command",
            sr_string(
                receipt
                    .get("preview_command")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "tcp_preview_server_started",
            sr_bool(
                receipt
                    .get("tcp_preview_server_started")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "tcp_requests_executed",
            sr_bool(
                receipt
                    .get("tcp_requests_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "browser_runtime_executed",
            sr_bool(
                receipt
                    .get("browser_runtime_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "hosted_provider_proof",
            sr_bool(
                receipt
                    .get("hosted_provider_proof")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "provider_bound_cdn_executed",
            sr_bool(
                receipt
                    .get("provider_bound_cdn_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "cleared_external_proof_gap_ids",
            sr_string("preview-tcp-server-parity"),
        ),
        (
            "remaining_external_proof_gap_ids",
            sr_string(remaining_gap_ids),
        ),
        (
            "check_count",
            sr_number(
                receipt
                    .get("checks")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len),
            ),
        ),
        ("check_ids", sr_string(&check_ids)),
        (
            "check_etag_present",
            sr_bool(readiness_receipt_check_passed(receipt, "etag-present")),
        ),
        (
            "check_if_none_match_304",
            sr_bool(readiness_receipt_check_passed(receipt, "if-none-match-304")),
        ),
        (
            "check_if_modified_since_304",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "if-modified-since-304",
            )),
        ),
        (
            "check_head_omits_body",
            sr_bool(readiness_receipt_check_passed(receipt, "head-omits-body")),
        ),
        (
            "check_range_206",
            sr_bool(readiness_receipt_check_passed(receipt, "range-206")),
        ),
        (
            "check_range_416",
            sr_bool(readiness_receipt_check_passed(receipt, "range-416")),
        ),
        (
            "check_if_range_206",
            sr_bool(readiness_receipt_check_passed(receipt, "if-range-206")),
        ),
        (
            "check_stale_if_range_full_body",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "stale-if-range-falls-back-to-full-body",
            )),
        ),
        (
            "check_br_negotiation",
            sr_bool(readiness_receipt_check_passed(receipt, "br-negotiation")),
        ),
        (
            "check_gzip_negotiation",
            sr_bool(readiness_receipt_check_passed(receipt, "gzip-negotiation")),
        ),
        (
            "check_plain_asset_vary",
            sr_bool(readiness_receipt_check_passed(receipt, "plain-asset-vary")),
        ),
        (
            "check_static_options_204_allow_header",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "static-options-204-allow-header",
            )),
        ),
        (
            "check_static_post_405_allow_header",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "static-post-405-allow-header",
            )),
        ),
        (
            "check_precompressed_decoded_content_type",
            sr_bool(readiness_receipt_check_passed(
                receipt,
                "precompressed-decoded-content-type",
            )),
        ),
        (
            "rule",
            sr_string(
                "local production preview TCP proof only; browser runtime and live Axum/server transport parity and CDN proof and hosted-provider proof remain separate gates",
            ),
        ),
    ]
}

fn readiness_native_event_browser_binder_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("browser plugin replay")),
        (
            "schema",
            sr_string(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(native_event_browser_binder_status_from_receipt(receipt)),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "browser_runtime_executed",
            sr_bool(
                receipt
                    .get("browser_runtime_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "binder_global_present",
            sr_bool(
                receipt
                    .get("binder_global_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "supported_event_count",
            sr_number(
                receipt
                    .get("supported_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "catalog_hash",
            sr_string(
                receipt
                    .get("catalog_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "preview_event_count",
            sr_number(
                receipt
                    .get("preview_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "state_dispatch_count",
            sr_number(
                receipt
                    .get("state_dispatch_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "missing_listener_event_count",
            sr_number(
                receipt
                    .get("missing_listener_events")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "missing_contract_event_count",
            sr_number(
                receipt
                    .get("missing_contract_events")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "missing_replay_event_count",
            sr_number(
                receipt
                    .get("missing_replay_events")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "click_event_constructor",
            sr_string(
                receipt
                    .pointer("/browser_event_constructors/click")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "pointermove_event_constructor",
            sr_string(
                receipt
                    .pointer("/browser_event_constructors/pointermove")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "input_event_constructor",
            sr_string(
                receipt
                    .pointer("/browser_event_constructors/input")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "browser_snapshot_hash",
            sr_string(
                receipt
                    .get("browser_snapshot_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "unsupported_listener_attached",
            sr_bool(
                receipt
                    .get("unsupported_listener_attached")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-native-event-browser-binder-proof-scope"),
            ),
        ),
        (
            "react_synthetic_events",
            sr_bool(
                receipt
                    .get("react_synthetic_events")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "full_react_event_parity",
            sr_bool(
                receipt
                    .get("full_react_event_parity")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "rule",
            sr_string(
                "local browser binder replay receipt only; hosted provider and full release proof remain separate gates",
            ),
        ),
    ]
}

fn readiness_state_runtime_browser_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("browser plugin replay")),
        (
            "schema",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(state_runtime_browser_status_from_receipt(receipt)),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "canonical_starter_route",
            sr_string(READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE),
        ),
        (
            "canonical_proof_target_route",
            sr_string(READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE),
        ),
        (
            "canonical_starter_source",
            sr_string(READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE),
        ),
        (
            "canonical_local_dev_url",
            sr_string(READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL),
        ),
        ("browser_runtime_executed_by_readiness", sr_bool(false)),
        (
            "browser_runtime_executed",
            sr_bool(
                receipt
                    .get("browser_runtime_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "runtime_global_present",
            sr_bool(
                receipt
                    .get("runtime_global_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "full_react_hook_runtime",
            sr_bool(
                receipt
                    .get("full_react_hook_runtime")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "react_api_shim_executed",
            sr_bool(
                receipt
                    .get("react_api_shim_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "state_reflection_event_count",
            sr_number(
                receipt
                    .get("state_reflection_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "derived_reflection_event_count",
            sr_number(
                receipt
                    .get("derived_reflection_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "effect_scheduled_event_count",
            sr_number(
                receipt
                    .get("effect_scheduled_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "action_dispatch_count",
            sr_number(
                receipt
                    .get("action_dispatch_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "missing_api_method_count",
            sr_number(
                receipt
                    .get("missing_api_methods")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "slot_count",
            sr_number(
                receipt
                    .get("slot_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "event_count",
            sr_number(
                receipt
                    .get("event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "browser_snapshot_hash",
            sr_string(
                receipt
                    .get("browser_snapshot_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "rule",
            sr_string(
                "local browser state-runtime replay receipt only; hosted provider and full release readiness proof remain separate gates",
            ),
        ),
    ]
}

fn readiness_island_browser_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-island-browser-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(island_browser_status_from_receipt(receipt)),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "canonical_starter_route",
            sr_string(READINESS_ISLANDS_CANONICAL_STARTER_ROUTE),
        ),
        (
            "canonical_proof_target_route",
            sr_string(READINESS_ISLANDS_CANONICAL_STARTER_ROUTE),
        ),
        (
            "canonical_starter_source",
            sr_string(READINESS_ISLANDS_CANONICAL_STARTER_SOURCE),
        ),
        (
            "canonical_local_dev_url",
            sr_string(READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL),
        ),
        ("browser_runtime_executed_by_readiness", sr_bool(false)),
        (
            "browser_runtime_executed",
            sr_bool(
                receipt
                    .get("browser_runtime_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "source_owned_bridge",
            sr_bool(
                receipt
                    .get("source_owned_bridge")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "bridge_abi_style",
            sr_string(
                receipt
                    .get("bridge_abi_style")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "directive_style",
            sr_string(
                receipt
                    .get("directive_style")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "island_count",
            sr_number(
                receipt
                    .get("island_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_owned_island_count",
            sr_number(
                receipt
                    .get("source_owned_island_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "directives_seen",
            sr_string(
                receipt
                    .get("directives_seen")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(";")
                    })
                    .unwrap_or_else(|| "missing".to_string()),
            ),
        ),
        (
            "missing_core_directive_count",
            sr_number(
                receipt
                    .get("missing_core_directives")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "hydration_strategy_count",
            sr_number(
                receipt
                    .get("hydration_strategies")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "event_node_count",
            sr_number(
                receipt
                    .get("event_node_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "client_island_event_count",
            sr_number(
                receipt
                    .get("client_island_event_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "missed_event_replay_count",
            sr_number(
                receipt
                    .get("missed_event_replay_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "browser_snapshot_hash",
            sr_string(
                receipt
                    .get("browser_snapshot_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-island-browser-proof-scope"),
            ),
        ),
        (
            "machine_contract_path",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "local source-owned island browser replay receipt only; hosted/provider adapter breadth remains a separate gate",
            ),
        ),
    ]
}

fn readiness_visual_edit_browser_workbench_sr_fields(
    receipt: &Value,
) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        (
            "command",
            sr_string("dx www readiness --import-visual-edit-browser-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        ("passed", sr_bool(true)),
        ("status", sr_string("browser-workbench-replay-current")),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("browser_runtime_executed", sr_bool(true)),
        ("devtools_global_present", sr_bool(true)),
        (
            "browser_workbench_replay",
            sr_string(
                receipt
                    .get("browser_workbench_replay")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "visual_replay_attempted",
            sr_bool(
                receipt
                    .get("visual_replay_attempted")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "visual_replay_status",
            sr_string(
                receipt
                    .get("visual_replay_status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "visual_replay_reason",
            sr_string(
                receipt
                    .get("visual_replay_reason")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "workbench_phases",
            sr_string("inspect;cascade;preview;apply;undo;receipt"),
        ),
        (
            "missing_workbench_phase_count",
            sr_number(
                receipt
                    .get("missing_workbench_phases")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len) as u64,
            ),
        ),
        (
            "inspected_element_present",
            sr_bool(
                receipt
                    .get("inspected_element_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "cascade_inspected",
            sr_bool(
                receipt
                    .get("cascade_inspected")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "preview_source_mutated",
            sr_bool(
                receipt
                    .get("preview_source_mutated")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "apply_source_mutated",
            sr_bool(
                receipt
                    .get("apply_source_mutated")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "undo_source_restored",
            sr_bool(
                receipt
                    .get("undo_source_restored")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "apply_receipt_written",
            sr_bool(
                receipt
                    .get("apply_receipt_written")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "undo_receipt_written",
            sr_bool(
                receipt
                    .get("undo_receipt_written")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "page_url",
            sr_string(
                receipt
                    .get("page_url")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "viewport_width",
            sr_number(
                receipt
                    .get("viewport")
                    .and_then(|viewport| viewport.get("width"))
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "viewport_height",
            sr_number(
                receipt
                    .get("viewport")
                    .and_then(|viewport| viewport.get("height"))
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "inspected_selector",
            sr_string(
                receipt
                    .get("inspected_selector")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "source_target",
            sr_string(
                receipt
                    .get("source_target")
                    .and_then(|target| target.get("relativePath"))
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "style_property",
            sr_string(
                receipt
                    .get("style_property")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "browser_snapshot_hash",
            sr_string(
                receipt
                    .get("browser_snapshot_hash")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "rule",
            sr_string(
                "local in-app browser visual-edit workbench replay receipt only; hosted provider and cross-route release proof remain separate gates",
            ),
        ),
    ]
}

fn readiness_server_action_replay_ledger_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing-server-action-replay-ledger"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "ledger_path",
            sr_string(
                receipt
                    .get("ledger_path")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "ledger_present",
            sr_bool(
                receipt
                    .get("ledger_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "hosted_provider_proof",
            sr_bool(
                receipt
                    .get("hosted_provider_proof")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "provider_proof_status",
            sr_string(
                receipt
                    .get("provider_proof_status")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "production_proof_scope",
            sr_string(
                receipt
                    .get("production_proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "provider_hosted_replay_required",
            sr_bool(
                receipt
                    .get("provider_hosted_replay_required")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "provider_proof_gap_ids",
            sr_string(
                receipt
                    .get("provider_proof_gap_ids")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(";")
                    })
                    .unwrap_or_default(),
            ),
        ),
        (
            "entry_count",
            sr_number(
                receipt
                    .get("entry_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "rule",
            sr_string(
                "local production-preview server-action replay ledger only; hosted provider proof remains separate",
            ),
        ),
    ]
}

fn readiness_primitive_proof_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT),
        ),
        (
            "primitive_proof_schema",
            sr_string(READINESS_PRIMITIVE_PROOF_SCHEMA),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("source_owned", sr_bool(true)),
        ("browser_runtime_executed", sr_bool(false)),
        ("hosted_provider_proof", sr_bool(false)),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "primitive_count",
            sr_number(
                receipt
                    .get("primitive_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "primitive_current_count",
            sr_number(
                receipt
                    .get("primitive_current_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_root",
            sr_string(
                receipt
                    .get("source_root")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "receipt_contract",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT),
        ),
        (
            "json_read_model_path",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT),
        ),
        (
            "machine_contract_path",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "source-owned Image/Font/Script/Wasm primitive foundation only; hosted/browser behavior remains a separate release-readiness gate",
            ),
        ),
    ]
}

fn readiness_reactivity_model_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT),
        ),
        (
            "reactivity_model_schema",
            sr_string(READINESS_REACTIVITY_MODEL_SCHEMA),
        ),
        (
            "runtime_capabilities_schema",
            sr_string("dx.tsx.dxNativeReactivityCapabilities"),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("readiness_release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("source_owned", sr_bool(true)),
        ("source_owned_runtime", sr_bool(true)),
        ("public_runtime", sr_string("DX-native fine-grained state")),
        (
            "dx_native_api",
            sr_string("state();derived();effect();action()"),
        ),
        (
            "compatibility_lowering",
            sr_string("useState(exact-dx-state-slot-only)"),
        ),
        ("compatibility_lowering_api", sr_string("useState")),
        ("exact_lowering_required", sr_bool(true)),
        (
            "use_state_lowering_rule",
            sr_string(
                "lower useState only when state_graph_has_exact_use_state_lowering proves every binding maps to a compiler-owned DX state slot",
            ),
        ),
        (
            "unsupported_unlowerable_use_state_diagnostic",
            sr_string("dx.react-hook.useState.missing-exact-state-slot"),
        ),
        ("adapter_boundary_required_when_unlowerable", sr_bool(true)),
        (
            "unsupported_react_hooks",
            sr_string("useEffect;useReducer;useContext;useTransition"),
        ),
        ("react_api_shim_executed", sr_bool(false)),
        ("full_react_hook_runtime", sr_bool(false)),
        ("browser_runtime_executed", sr_bool(false)),
        ("hosted_provider_proof", sr_bool(false)),
        (
            "browser_proof_status",
            sr_string("foundation-not-release-proof"),
        ),
        (
            "node_vm_state_runtime_replay_status",
            sr_string("source-guarded-not-real-browser-proof"),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("local-source-owned-reactivity-model-foundation"),
            ),
        ),
        (
            "source_check_count",
            sr_number(
                receipt
                    .get("source_check_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_check_current_count",
            sr_number(
                receipt
                    .get("source_check_current_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "receipt_contract",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT),
        ),
        (
            "json_read_model_path",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT),
        ),
        (
            "browser_replay_receipt_contract",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT),
        ),
        (
            "browser_replay_receipt",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT),
        ),
        (
            "machine_contract_path",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "source-owned reactivity model foundation only; browser replay and hosted breadth remain separate release-readiness gates",
            ),
        ),
    ]
}

fn readiness_docs_onboarding_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT),
        ),
        (
            "docs_onboarding_schema",
            sr_string(READINESS_DOCS_ONBOARDING_SCHEMA),
        ),
        ("docs_doctor_schema", sr_string("dx.www.docs_doctor")),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("readiness_release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("source_owned", sr_bool(true)),
        (
            "docs_doctor_command",
            sr_string("dx www docs-doctor --json"),
        ),
        (
            "docs_doctor_report_evaluated",
            sr_bool(
                receipt
                    .get("docs_doctor_report_evaluated")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        ("docs_doctor_runtime_executed", sr_bool(false)),
        (
            "docs_doctor_error_count",
            sr_number(
                receipt
                    .get("docs_doctor_error_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "docs_doctor_warning_count",
            sr_number(
                receipt
                    .get("docs_doctor_warning_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        ("public_docs_source_guarded", sr_bool(true)),
        ("compatibility_surfaces_warning_only", sr_bool(true)),
        (
            "generated_archived_warning_surfaces_clean",
            sr_bool(
                receipt
                    .get("generated_archived_warning_surfaces_clean")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "generated_archived_warning_surfaces_promoted",
            sr_bool(
                receipt
                    .get("generated_archived_warning_surfaces_promoted")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "generated_archived_warning_finding_count",
            sr_number(
                receipt
                    .get("generated_archived_warning_finding_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("local-source-owned-docs-onboarding-foundation"),
            ),
        ),
        (
            "source_check_count",
            sr_number(
                receipt
                    .get("source_check_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_check_current_count",
            sr_number(
                receipt
                    .get("source_check_current_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "receipt_contract",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT),
        ),
        (
            "json_read_model_path",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT),
        ),
        (
            "machine_contract_path",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "source-owned docs/onboarding guardrail with generated/archive cleanup evaluation; external docs-doctor command replay and compatibility warning cleanup remain separate release-readiness gates",
            ),
        ),
    ]
}

fn readiness_island_abi_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www readiness")),
        ("command", sr_string("dx www readiness --write-receipts")),
        ("schema", sr_string(READINESS_ISLAND_ABI_RECEIPT_CONTRACT)),
        (
            "island_abi_schema",
            sr_string("dx.www.readiness.island_abi"),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("source_owned", sr_bool(true)),
        ("browser_runtime_executed", sr_bool(false)),
        ("hosted_provider_proof", sr_bool(false)),
        ("provider_adapter_executed", sr_bool(false)),
        ("directive_style_id", sr_string("camelCase-jsx-props")),
        (
            "directives",
            sr_string("clientLoad;clientVisible;clientIdle;clientOnly"),
        ),
        (
            "core_directives",
            sr_string("clientLoad;clientVisible;clientIdle;clientOnly"),
        ),
        (
            "supported_directives",
            sr_string(
                "clientLoad;clientVisible;clientIdle;clientOnly;clientMedia;clientInteraction",
            ),
        ),
        (
            "additional_supported_directives",
            sr_string("clientMedia;clientInteraction"),
        ),
        (
            "release_core_directives",
            sr_string("clientLoad;clientVisible;clientIdle;clientOnly"),
        ),
        (
            "unsupported_directive_syntax",
            sr_string("client:load;client:visible;client:idle;client:only"),
        ),
        (
            "route_unit_proof_metadata",
            sr_string("DxRouteReceipt.client_island_abi"),
        ),
        (
            "route_streaming_island_metadata",
            sr_string(
                "directive_style_id;directives;hydration_strategy;no_js_fallback_required;browser_proof_status;framework_adapter",
            ),
        ),
        (
            "proof_scope",
            sr_string(
                receipt
                    .get("proof_scope")
                    .and_then(Value::as_str)
                    .unwrap_or("local-source-owned-island-abi-foundation"),
            ),
        ),
        (
            "source_check_count",
            sr_number(
                receipt
                    .get("source_check_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_check_current_count",
            sr_number(
                receipt
                    .get("source_check_current_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "source_root",
            sr_string(
                receipt
                    .get("source_root")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "receipt_contract",
            sr_string(READINESS_ISLAND_ABI_RECEIPT_CONTRACT),
        ),
        (
            "json_read_model_path",
            sr_string(READINESS_ISLAND_ABI_RECEIPT),
        ),
        (
            "machine_contract_path",
            sr_string(READINESS_ISLAND_ABI_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "no browser/provider adapter execution is claimed by this source-owned islands ABI receipt",
            ),
        ),
    ]
}

fn readiness_no_js_artifact_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www")),
        ("command", sr_string("dx www readiness --write-receipts")),
        (
            "schema",
            sr_string(READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "artifact_root",
            sr_string(
                receipt
                    .get("artifact_root")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "artifact_source",
            sr_string(
                receipt
                    .get("artifact_source")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "script_tag_count",
            sr_number(
                receipt
                    .get("script_tag_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "html_path",
            sr_string(
                receipt
                    .get("html_path")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "artifact_html_blake3",
            sr_string(
                receipt
                    .get("artifact_html_blake3")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "data_dx_output_mode_tiny_static",
            sr_bool(
                receipt
                    .get("data_dx_output_mode_tiny_static")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "data_dx_js_none",
            sr_bool(
                receipt
                    .get("data_dx_js_none")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "semantic_landmark_present",
            sr_bool(
                receipt
                    .get("semantic_landmark_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "link_count",
            sr_number(
                receipt
                    .get("link_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "form_count",
            sr_number(
                receipt
                    .get("form_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "seo_title_present",
            sr_bool(
                receipt
                    .get("seo_title_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "accessibility_signal_count",
            sr_number(
                receipt
                    .get("accessibility_signal_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "link_form_navigation_proof_current",
            sr_bool(
                receipt
                    .get("link_form_navigation_proof_current")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "route_unit_link_form_navigation_current",
            sr_bool(
                receipt
                    .get("route_unit_link_form_navigation_current")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "html_css_links_forms_seo_accessibility_current",
            sr_bool(
                receipt
                    .get("html_css_links_forms_seo_accessibility_current")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "public_packet_present",
            sr_bool(
                receipt
                    .get("public_packet_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "public_js_artifact_count",
            sr_number(
                receipt
                    .get("public_js_artifact_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(u64::MAX),
            ),
        ),
        (
            "route_unit_present",
            sr_bool(
                receipt
                    .get("route_unit_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "route_unit_output_mode",
            sr_string(
                receipt
                    .get("route_unit_output_mode")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "route_unit_js",
            sr_string(
                receipt
                    .get("route_unit_js")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "route_unit_script_tag_count",
            sr_number(
                receipt
                    .get("route_unit_script_tag_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(u64::MAX),
            ),
        ),
        (
            "route_unit_runtime_required",
            sr_bool(
                receipt
                    .get("route_unit_runtime_required")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "route_unit_browser_api_required",
            sr_bool(
                receipt
                    .get("route_unit_browser_api_required")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "route_unit_semantic_landmark_present",
            sr_bool(
                receipt
                    .get("route_unit_semantic_landmark_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "route_unit_link_count",
            sr_number(
                receipt
                    .get("route_unit_link_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "route_unit_form_count",
            sr_number(
                receipt
                    .get("route_unit_form_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "route_unit_seo_title_present",
            sr_bool(
                receipt
                    .get("route_unit_seo_title_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "route_unit_accessibility_signal_count",
            sr_number(
                receipt
                    .get("route_unit_accessibility_signal_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "route_unit_no_js_capable",
            sr_bool(
                receipt
                    .get("route_unit_no_js_capable")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "route_unit_no_js_proof_current",
            sr_bool(
                receipt
                    .get("route_unit_no_js_proof_current")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        ("live_browser_executed", sr_bool(false)),
        ("javascript_disabled_browser", sr_bool(false)),
        ("astro_parity_claimed", sr_bool(false)),
        (
            "rule",
            sr_string(
                "artifact-only no-JS proof; live browser and Astro parity remain separate gates",
            ),
        ),
    ]
}

fn readiness_no_js_browser_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www")),
        (
            "command",
            sr_string("dx www readiness --import-no-js-browser-receipt"),
        ),
        (
            "schema",
            sr_string(READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "status",
            sr_string("current-local-js-disabled-browser-proof"),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        (
            "html_path",
            sr_string(
                receipt
                    .get("html_path")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "artifact_html_blake3",
            sr_string(
                receipt
                    .get("artifact_html_blake3")
                    .and_then(Value::as_str)
                    .unwrap_or("missing"),
            ),
        ),
        (
            "url",
            sr_string(receipt.get("url").and_then(Value::as_str).unwrap_or("")),
        ),
        (
            "user_agent",
            sr_string(
                receipt
                    .get("user_agent")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        (
            "live_browser_executed",
            sr_bool(
                receipt
                    .get("live_browser_executed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "javascript_disabled_browser",
            sr_bool(
                receipt
                    .get("javascript_disabled_browser")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "page_javascript_enabled",
            sr_bool(
                receipt
                    .get("page_javascript_enabled")
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
            ),
        ),
        (
            "script_tag_count",
            sr_number(
                receipt
                    .get("script_tag_count")
                    .and_then(Value::as_u64)
                    .unwrap_or(u64::MAX),
            ),
        ),
        (
            "data_dx_output_mode_tiny_static",
            sr_bool(
                receipt
                    .get("data_dx_output_mode_tiny_static")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "data_dx_js_none",
            sr_bool(
                receipt
                    .get("data_dx_js_none")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "semantic_landmark_present",
            sr_bool(
                receipt
                    .get("semantic_landmark_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "visible_text_present",
            sr_bool(
                receipt
                    .get("visible_text_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "link_count",
            sr_number(
                receipt
                    .get("link_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "form_count",
            sr_number(
                receipt
                    .get("form_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "seo_title_present",
            sr_bool(
                receipt
                    .get("seo_title_present")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "accessibility_signal_count",
            sr_number(
                receipt
                    .get("accessibility_signal_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "rule",
            sr_string(
                "local JS-disabled browser receipt only; hosted/provider and Astro parity remain separate gates",
            ),
        ),
    ]
}

fn readiness_proof_graph_sr_fields(manifest_hash: &str) -> Vec<(&'static str, String)> {
    let stale_reason_codes = vec![
        "same-machine-performance-receipt-missing".to_string(),
        "tiny-static-no-js-artifact-receipt-missing".to_string(),
        "no-js-browser-receipt-missing".to_string(),
        "lighthouse-paint-receipts-missing".to_string(),
        "bundle-partition-receipt-missing".to_string(),
        "production-http-local-replay-receipt-missing".to_string(),
        "production-http-tcp-preview-receipt-missing".to_string(),
        "server-action-replay-ledger-receipt-missing".to_string(),
        "route-handler-provider-replay-receipt-missing".to_string(),
        "primitive-proof-receipt-missing".to_string(),
        "native-event-catalog-receipt-missing".to_string(),
        "island-abi-receipt-missing".to_string(),
        "island-browser-receipt-missing".to_string(),
        "native-event-browser-binder-receipt-missing".to_string(),
        "state-runtime-browser-receipt-missing".to_string(),
        "docs-onboarding-receipt-not-current".to_string(),
        "visual-edit-browser-workbench-replay-missing".to_string(),
        "reactivity-model-receipt-not-current".to_string(),
        "browser-tcp-cdn-hosted-proof-missing".to_string(),
        "hosted-provider-proof-missing".to_string(),
    ];
    readiness_proof_graph_sr_fields_for_command(
        manifest_hash,
        "dx build",
        "written-by-dx-build-not-release-proof",
        "build-output-deploy-adapter-proof-graph",
        &stale_reason_codes,
    )
}

fn readiness_proof_graph_sr_fields_for_command(
    manifest_hash: &str,
    command: &str,
    receipt_freshness: &str,
    proof_scope: &str,
    stale_reason_codes: &[String],
) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www")),
        ("command", sr_string(command)),
        ("schema", sr_string(READINESS_PROOF_GRAPH_SCHEMA)),
        ("schema_revision", sr_number(1)),
        ("passed", sr_bool(false)),
        ("release_ready", sr_bool(false)),
        ("current_score", sr_number(READINESS_CURRENT_HONEST_SCORE)),
        ("target_score", sr_number(READINESS_TARGET_SCORE)),
        ("fastest_world_claim", sr_bool(false)),
        ("proof_scope", sr_string(proof_scope)),
        ("manifest_hash", sr_string(manifest_hash)),
        (
            "inputs",
            sr_string(
                ".dx/build-cache/manifest.json;.dx/build-cache/deploy-adapter.json;readiness_gate_status;proof_nodes;same-machine-performance;tiny-static-no-js-artifact;tiny-static-no-js-browser;lighthouse-paint-receipts;production-http-local-replay;production-http-tcp-preview;server-action-replay-ledger;route-handler-provider-replay;primitive-proof;native-event-catalog;native-event-browser-binder;island-abi;island-browser;reactivity-model;state-runtime-browser;bundle-partition;docs-onboarding-receipt;docs-onboarding-doctor;visual-edit-workbench-receipts;proof_graph",
            ),
        ),
        (
            "output_hashes",
            sr_string(
                "manifest_hash=blake3;serializer_source_blake3=source_blake3;serializer_machine_blake3=machine_blake3",
            ),
        ),
        ("receipt_freshness", sr_string(receipt_freshness)),
        ("stale_reasons", sr_string(stale_reason_codes.join(";"))),
        (
            "replay_commands",
            sr_string(
                "dx www readiness --json --full;dx www agent-context --json --full;dx www docs-doctor --json",
            ),
        ),
        ("proof_graph", sr_string(READINESS_PROOF_GRAPH_SCHEMA)),
        (
            "proof_graph_receipt_machine",
            sr_string(READINESS_PROOF_GRAPH_RECEIPT_MACHINE),
        ),
        (
            "score_breakdown",
            sr_string(READINESS_SCORE_BREAKDOWN_SCHEMA),
        ),
        ("delivery_tiers", sr_string(READINESS_DELIVERY_TIERS_SCHEMA)),
        (
            "native_event_catalog",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_SCHEMA),
        ),
        (
            "native_event_catalog_receipt_contract",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT),
        ),
        (
            "native_event_catalog_receipt",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_RECEIPT),
        ),
        (
            "native_event_catalog_receipt_sr",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR),
        ),
        (
            "native_event_catalog_receipt_machine",
            sr_string(READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE),
        ),
        (
            "native_event_browser_binder_receipt_contract",
            sr_string(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT),
        ),
        (
            "native_event_browser_binder_receipt",
            sr_string(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT),
        ),
        (
            "native_event_browser_binder_receipt_sr",
            sr_string(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR),
        ),
        (
            "native_event_browser_binder_receipt_machine",
            sr_string(READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE),
        ),
        (
            "state_runtime_browser_receipt_contract",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT),
        ),
        (
            "state_runtime_browser_receipt",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT),
        ),
        (
            "state_runtime_browser_receipt_sr",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR),
        ),
        (
            "state_runtime_browser_receipt_machine",
            sr_string(READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE),
        ),
        (
            "reactivity_model",
            sr_string(READINESS_REACTIVITY_MODEL_SCHEMA),
        ),
        (
            "reactivity_model_receipt_contract",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT),
        ),
        (
            "reactivity_model_receipt",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT),
        ),
        (
            "reactivity_model_receipt_sr",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_SR),
        ),
        (
            "reactivity_model_receipt_machine",
            sr_string(READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE),
        ),
        (
            "server_action_replay_ledger_receipt_contract",
            sr_string(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT),
        ),
        (
            "server_action_replay_ledger_receipt",
            sr_string(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT),
        ),
        (
            "server_action_replay_ledger_receipt_sr",
            sr_string(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR),
        ),
        (
            "server_action_replay_ledger_receipt_machine",
            sr_string(READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE),
        ),
        (
            "bundle_partition",
            sr_string(READINESS_BUNDLE_PARTITION_SCHEMA),
        ),
        (
            "production_http",
            sr_string(READINESS_PRODUCTION_HTTP_SCHEMA),
        ),
        (
            "production_http_receipt_contract",
            sr_string(READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT),
        ),
        (
            "production_http_receipt",
            sr_string(READINESS_PRODUCTION_HTTP_RECEIPT),
        ),
        (
            "production_http_receipt_sr",
            sr_string(READINESS_PRODUCTION_HTTP_RECEIPT_SR),
        ),
        (
            "production_http_receipt_machine",
            sr_string(READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE),
        ),
        (
            "route_action_runtime",
            sr_string(READINESS_ROUTE_ACTION_RUNTIME_SCHEMA),
        ),
        (
            "primitive_proof",
            sr_string(READINESS_PRIMITIVE_PROOF_SCHEMA),
        ),
        (
            "primitive_proof_receipt_contract",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT),
        ),
        (
            "primitive_proof_receipt",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT),
        ),
        (
            "primitive_proof_receipt_sr",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_SR),
        ),
        (
            "primitive_proof_receipt_machine",
            sr_string(READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE),
        ),
        ("island_abi", sr_string(READINESS_ISLAND_ABI_SCHEMA)),
        (
            "island_abi_receipt_contract",
            sr_string(READINESS_ISLAND_ABI_RECEIPT_CONTRACT),
        ),
        (
            "island_abi_receipt",
            sr_string(READINESS_ISLAND_ABI_RECEIPT),
        ),
        (
            "island_abi_receipt_sr",
            sr_string(READINESS_ISLAND_ABI_RECEIPT_SR),
        ),
        (
            "island_abi_receipt_machine",
            sr_string(READINESS_ISLAND_ABI_RECEIPT_MACHINE),
        ),
        (
            "island_browser_receipt_contract",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT),
        ),
        (
            "island_browser_receipt",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT),
        ),
        (
            "island_browser_receipt_sr",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT_SR),
        ),
        (
            "island_browser_receipt_machine",
            sr_string(READINESS_ISLAND_BROWSER_RECEIPT_MACHINE),
        ),
        (
            "route_handler_server_action_gaps",
            sr_string(READINESS_ROUTE_HANDLER_SERVER_ACTION_GAPS_SCHEMA),
        ),
        (
            "docs_onboarding",
            sr_string(READINESS_DOCS_ONBOARDING_SCHEMA),
        ),
        (
            "docs_onboarding_receipt_contract",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT),
        ),
        (
            "docs_onboarding_receipt",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT),
        ),
        (
            "docs_onboarding_receipt_sr",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_SR),
        ),
        (
            "docs_onboarding_receipt_machine",
            sr_string(READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE),
        ),
        (
            "docs_doctor_command_replay_receipt_contract",
            sr_string(docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT),
        ),
        (
            "docs_doctor_command_replay_receipt",
            sr_string(docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT),
        ),
        (
            "docs_doctor_command_replay_receipt_sr",
            sr_string(docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR),
        ),
        (
            "docs_doctor_command_replay_receipt_machine",
            sr_string(docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE),
        ),
        (
            "visual_edit_workbench_receipt_contract",
            sr_string(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT),
        ),
        (
            "visual_edit_workbench_receipt",
            sr_string(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT),
        ),
        (
            "visual_edit_workbench_receipt_sr",
            sr_string(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR),
        ),
        (
            "visual_edit_workbench_receipt_machine",
            sr_string(READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE),
        ),
        ("public_runtime_bundle", sr_bool(true)),
        ("evidence_bundle", sr_bool(true)),
        ("cache_manifest", sr_string(".dx/build-cache/cache-manifest.json")),
        (
            "provider_adapter_smoke_matrix",
            sr_string(".dx/build-cache/provider-adapter-smoke-matrix.json"),
        ),
        ("precompressed_assets", sr_bool(true)),
        (
            "missing_gate",
            sr_string("Astro tiny-static parity remains a proof gate"),
        ),
        (
            "proof_gap",
            sr_string("route-handler-server-action-proof-gaps"),
        ),
        (
            "dx_check_context",
            sr_string("dx check --latest-receipt --json"),
        ),
        (
            "readiness_command",
            sr_string("dx www readiness --json --full"),
        ),
        (
            "agent_context",
            sr_string("dx www agent-context --json --full"),
        ),
        ("docs_doctor", sr_string("dx www docs-doctor --json")),
    ]
}
