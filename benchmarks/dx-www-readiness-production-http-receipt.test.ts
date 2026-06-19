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

test("release-readiness production HTTP local replay has durable JSON/SR/machine receipts", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const previewContract = read("dx-www/src/cli/preview_contract.rs");
  const deployAdapter = read("dx-www/src/cli/deploy_adapter_contract.rs");

  expectAll(readiness, [
    "READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT",
    "READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND",
    "READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.json",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.sr",
    ".dx/serializer/receipts-readiness-production-http-tcp-preview-latest.machine",
    "READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS",
    "dx.www.readiness.production_http_local_replay_receipt_contract",
    ".dx/receipts/readiness/production-http-local-replay-latest.json",
    ".dx/receipts/readiness/production-http-local-replay-latest.sr",
    ".dx/serializer/receipts-readiness-production-http-local-replay-latest.machine",
    "write_readiness_production_http_local_replay_receipt",
    "readiness_production_http_local_replay_receipt_is_current",
    "readiness_production_http_expected_check_ids",
    "readiness_production_http_local_replay_stale_reason",
    "readiness_production_http_tcp_preview_receipt_is_current",
    "readiness_production_http_tcp_preview_stale_reason",
    "readiness_production_http_axum_source_parity",
    "source-owned-axum-adapter-parity-current-local",
    "axum_responder_source_parity",
    "remaining_external_proof_gap_meaning",
    "live Axum/server transport proof",
    "readiness_production_http_tcp_preview_sr_fields",
    "import_readiness_production_http_tcp_preview_receipt",
    "readiness_production_http_local_replay_stale_reason_from_receipt",
    "readiness_production_http_missing_check_ids",
    "production_http_stale_reason",
    "production-http-local-replay-receipt-missing",
    "production-http-local-replay-checks-failed",
    "production-http-browser-tcp-cdn-provider-proof-missing",
    "production_http_local_replay_current",
    "\"stale_reason\": production_http_stale_reason",
    "readiness_production_http_local_replay_sr_fields",
    "production_contract_wire_response",
    "local-production-contract-wire-replay",
    "tcp_preview_server_started",
    "browser_runtime_executed",
    "hosted_provider_proof",
    "provider_bound_cdn_executed",
    "external_proof_gap_ids",
    "remaining_external_proof_gap_ids",
    "production_http_local_replay_has_external_gap_ids",
    "browser-js-enabled-runtime-replay",
    "browser-js-disabled-runtime-replay",
    "preview-tcp-server-parity",
    "axum-static-responder-parity",
    "provider-bound-cdn-cache-replay",
    "hosted-provider-adapter-replay",
    "ETag",
    "If-None-Match",
    "If-Modified-Since",
    "Range: bytes=1-3",
    "If-Range",
    "stale-if-range-falls-back-to-full-body",
    "Content-Length: 6",
    "Accept-Encoding: gzip;q=0.5, br",
    "Accept-Encoding: br;q=0, gzip;q=1",
    "Content-Range: bytes 1-3/6",
    "Content-Range: bytes */6",
    "Content-Encoding: br",
    "Content-Encoding: gzip",
    "Content-Type: application/javascript; charset=utf-8",
    "Vary: Accept-Encoding",
    "static-options-204-allow-header",
    "static-post-405-allow-header",
    "precompressed-decoded-content-type",
    "Allow: GET, HEAD, OPTIONS",
    "check_static_options_204_allow_header",
    "check_static_post_405_allow_header",
    "check_precompressed_decoded_content_type",
    "check_if_none_match_304",
    "check_stale_if_range_full_body",
    "check_plain_asset_vary",
    "write_sr_artifact(",
    "write_readiness_json_receipt(",
    "production HTTP local wire replay JSON/SR/machine receipt",
    "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
    "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
    "node benchmarks/dx-www-production-preview-tcp-receipt.ts --dx-www-bin target/release/dx-www.exe --build-dir examples/template/.dx/www/output --out .dx/receipts/readiness/browser-import-candidates/production-http-tcp-preview-latest.json",
    "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
    "local-production-http-tcp-preview-current",
    "production-http-tcp-preview-receipt-missing",
  ]);

  expectAll(previewContract, [
    "production_contract_wire_response",
    "production_contract_wire_response_cached",
    "production_contract_axum_response_cached",
    "production_contract_axum_static_responder_matches_wire_semantics",
    "Accept-Ranges: bytes",
    "ETag:",
    "Last-Modified:",
    "Content-Range:",
    "Content-Encoding:",
    "Vary: Accept-Encoding",
    "accepted_precompressed_asset_suffixes",
    "strong_etag_matches",
    "request_method_allows_conditional_not_modified",
    "W/{etag}",
    "production_contract_static_options_response",
    "production_contract_static_method_guard_response",
    "production_contract_internal_options_response",
    "production_contract_internal_method_not_allowed_response",
    "production-ready-method-not-allowed",
    "production-observability-method-not-allowed",
    "production_contract_health_options_response",
    "production_contract_health_allowed_methods",
    "production_contract_health_method_matches",
    "production-health-method-not-allowed",
    "server_action_options_response",
    "server_action_allowed_methods",
    "production_contract_server_action_post_does_not_return_304_for_if_none_match",
    '"allowed_methods": [method, "OPTIONS"]',
    "format!(\"{method}, OPTIONS\")",
    "Allow: GET, HEAD, OPTIONS",
    "static-route-method-not-allowed",
  ]);

  expectAll(deployAdapter, [
    "static-preview-method-contract",
    "GET/HEAD/OPTIONS/405 static preview method guard",
  ]);

  const agentContext = read("dx-www/src/cli/agent_context.rs");
  expectAll(agentContext, [
    "remaining_external_proof_gap_ids",
    "external_proof_gap_ids",
    "missing-external-proof-gap-",
    "READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS",
  ]);

  assert.match(
    readiness,
    /"rule": "This receipt executes the local production-contract wire responder[\s\S]*not Browser, TCP preview server, Axum, CDN, or hosted-provider proof/,
  );
});
