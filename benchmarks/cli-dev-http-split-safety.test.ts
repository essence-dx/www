import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath) {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

test("dev HTTP parsing and wire helpers stay out of the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const devBridge = readRepoFile("dx-www/src/cli/dev_bridge.rs");
  const devHttp = readRepoFile("dx-www/src/cli/dev_http.rs");
  const devWire = readRepoFile("dx-www/src/cli/dev_wire.rs");

  assert.match(cliMod, /\bmod dev_http;/);
  assert.match(cliMod, /\bmod dev_wire;/);
  assert.match(cliMod, /use self::dev_http::\{/);
  assert.match(devBridge, /use super::dev_wire::\{/);
  assert.match(devHttp, /\bpub\(super\) struct DxCliHttpRequest\b/);
  assert.match(devHttp, /\bpub\(super\) struct DxCliHttpResponse\b/);
  assert.match(devHttp, /\bpub\(super\) fn parse_http_request\b/);
  assert.match(devHttp, /\bpub\(super\) fn apply_dev_cache_headers\b/);
  assert.match(devHttp, /\bpub\(super\) fn dev_lookup_path\b/);
  assert.match(devHttp, /\bpub\(super\) fn dev_project_reload_token\b/);
  assert.match(devHttp, /\bpub\(super\) fn with_dev_hot_reload\b/);
  assert.match(devHttp, /dev_hot_reload_client::dev_hot_reload_client_script/);
  assert.match(devHttp, /\bfn dev_reload_source_roots\b/);
  assert.match(devHttp, /\bfn split_http_headers_and_body\b/);
  assert.match(devHttp, /\bfn parse_http_headers\b/);
  assert.match(devWire, /\bpub\(super\) fn dev_wire_response_bytes\b/);
  assert.match(devWire, /\bpub\(super\) fn dev_response_headers\b/);
  assert.match(devWire, /\bpub\(super\) fn dev_response_cache_key\b/);
  assert.match(devWire, /\bpub\(super\) type DxDevHttpResponder\b/);
  assert.match(devWire, /\bpub\(super\) type DxDevResponseCache\b/);
  assert.match(devWire, /\bpub\(super\) fn new_dev_response_cache\b/);
  assert.match(devWire, /\bpub\(super\) fn handle_http_wire_response_cached\b/);
  assert.match(devWire, /response_cache: &DxDevResponseCache/);
  assert.match(devWire, /respond: DxDevHttpResponder/);
  assert.match(devWire, /let request = parse_http_request\(request\);/);
  assert.match(devWire, /let cache_key = if hot_reload/);
  assert.match(devWire, /let mut response = respond\(cwd, &request, translations\);/);
  assert.match(devWire, /with_dev_html_injections_token\(response,/);
  assert.match(devWire, /\bfn http_header_name\b/);
  assert.doesNotMatch(devHttp, /\bpub\(super\) fn dev_wire_response_bytes\b/);
  assert.doesNotMatch(devHttp, /\bpub\(super\) fn dev_response_headers\b/);
  assert.doesNotMatch(devHttp, /\bfn http_header_name\b/);
  assert.doesNotMatch(cliMod, /use self::dev_wire::\{/);

  assert.doesNotMatch(cliMod, /\bfn is_dev_cache_sensitive_path\b/);
  assert.doesNotMatch(cliMod, /\bfn dev_reload_source_roots\b/);
  assert.doesNotMatch(cliMod, /\bfn split_http_headers_and_body\b/);
  assert.doesNotMatch(cliMod, /\bfn parse_http_headers\b/);
  assert.doesNotMatch(cliMod, /\bfn http_header_name\b/);
  assert.doesNotMatch(cliMod, /\bfn with_dev_hot_reload\b/);
  assert.doesNotMatch(cliMod, /\bfn handle_http_wire_response_cached\b/);
  assert.doesNotMatch(cliMod, /\bDxDevCachedWireResponse\b/);
  assert.doesNotMatch(cliMod, /expires_at: now \+ DX_DEV_RESPONSE_CACHE_TTL/);
  assert.doesNotMatch(cliMod, /dev_hot_reload_client::dev_hot_reload_client_script/);
});

test("dev command serving loop stays out of the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const cliCore = readRepoFile("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const devCommand = readRepoFile("dx-www/src/cli/dev_command.rs");
  const devTinyServer = readRepoFile("dx-www/src/cli/dev_tiny_server.rs");

  assert.match(cliMod, /\bmod dev_command;/);
  assert.match(cliCore, /dev_command::cmd_dev\(/);
  assert.match(devCommand, /\bpub\(super\) fn cmd_dev\b/);
  assert.match(devCommand, /\bparse_dev_options\(args, &config\)\?/);
  assert.match(devCommand, /\bbind_dev_listener\(&options\.host, options\.port, options\.port_explicit\)\?/);
  assert.match(devCommand, /Development server running at http:\/\/\{\}:\{\}/);
  assert.match(devCommand, /crate::dev::axum_server::serve_dev_router\(listener, state\)/);
  assert.match(devTinyServer, /\bfn accept_tiny_dev_connections\b/);
  assert.match(devTinyServer, /\bfn tiny_dev_worker_count\b/);
  assert.match(devTinyServer, /\bhandle_http_wire_response_cached_with_connection\(/);

  assert.doesNotMatch(cliMod, /\bbind_dev_listener\(&options\.host, options\.port, options\.port_explicit\)\?/);
  assert.doesNotMatch(cliMod, /Development server running at http:\/\/\{\}:\{\}/);
  assert.doesNotMatch(cliMod, /crate::dev::axum_server::serve_dev_router\(listener, state\)/);
  assert.doesNotMatch(cliMod, /\baccept_tiny_dev_connections\b/);
  assert.doesNotMatch(cliMod, /\bhandle_http_wire_response_cached\(/);
});

test("dev HTTP response dispatch stays out of the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const devBridge = readRepoFile("dx-www/src/cli/dev_bridge.rs");
  const devResponse = readRepoFile("dx-www/src/cli/dev_response.rs");

  assert.match(cliMod, /\bmod dev_bridge;/);
  assert.match(cliMod, /\bmod dev_response;/);
  assert.match(devBridge, /\bimpl Cli\b/);
  assert.match(devBridge, /\bpub\(super\) fn handle_parsed_http_response\b/);
  assert.match(devBridge, /dev_response::handle_parsed_http_response\(/);
  assert.match(devBridge, /\bfn handle_http_response\b/);
  assert.match(devBridge, /\bfn handle_http_request\b/);
  assert.match(devBridge, /\bfn dev_lookup_path\b/);
  assert.match(devBridge, /\bfn dev_project_reload_token\b/);
  assert.match(devBridge, /\bfn dev_response_cache_key\b/);
  assert.match(devBridge, /\bfn dev_wire_response_bytes\b/);
  assert.match(devBridge, /\bfn dev_response_headers\b/);
  assert.match(devResponse, /use super::dev_http::parse_http_request;/);
  assert.match(devResponse, /\bpub\(super\) type DxDevRouteHandlerResponder\b/);
  assert.match(devResponse, /\bpub\(super\) type DxDevPageResponder\b/);
  assert.match(devResponse, /\bpub\(super\) fn handle_http_response\b/);
  assert.match(devResponse, /\bpub\(super\) fn handle_parsed_http_response\b/);
  assert.match(devResponse, /\bfn is_hot_reload_version_request\b/);
  assert.match(devResponse, /\bfn is_hot_reload_event_stream_request\b/);
  assert.match(devResponse, /\bfn hot_reload_event_stream_response\b/);
  assert.match(devResponse, /DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT/);
  assert.match(devResponse, /dx_hot_reload_version_payload\(/);
  assert.match(devResponse, /dx_hot_reload_resource_from_path\(&request\.path\)/);
  assert.match(devResponse, /apply_dev_cache_headers\(request, &mut response\)/);

  assert.doesNotMatch(cliMod, /\bfn apply_dev_cache_headers\b/);
  assert.doesNotMatch(cliMod, /\bfn handle_http_request\b/);
  assert.doesNotMatch(cliMod, /\bfn handle_http_response\b/);
  assert.doesNotMatch(cliMod, /\bfn handle_parsed_http_response\b/);
  assert.doesNotMatch(cliMod, /\bfn dev_wire_response_bytes\b/);
  assert.doesNotMatch(cliMod, /\bfn dev_response_headers\b/);
  assert.doesNotMatch(cliMod, /dx_hot_reload_version_payload\(/);
  assert.doesNotMatch(cliMod, /dx_hot_reload_resource_from_path\(&request\.path\)/);
  assert.doesNotMatch(cliMod, /apply_dev_cache_headers\(request, &mut response\)/);
});

test("dev fallback TCP reader has bounded request diagnostics outside the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const devTinyServer = readRepoFile("dx-www/src/cli/dev_tiny_server.rs");
  const devHttp = readRepoFile("dx-www/src/cli/dev_http.rs");
  const devWire = readRepoFile("dx-www/src/cli/dev_wire.rs");

  assert.match(devWire, /\bpub\(super\) const DX_DEV_MAX_REQUEST_BYTES\b/);
  assert.match(devWire, /\bpub\(super\) enum DxDevWireRequestError\b/);
  assert.match(devWire, /\bpub\(super\) fn read_http_wire_request\b/);
  assert.match(devWire, /\bpub\(super\) fn dev_wire_request_error_response\b/);
  assert.match(devWire, /\bfn request_bytes_complete\b/);
  assert.match(devWire, /\bfn declared_content_length\b/);
  assert.match(devWire, /"dx-dev-request-too-large"/);
  assert.match(devWire, /"dx-dev-request-malformed"/);
  assert.match(devWire, /"node_modules_required": false/);
  assert.doesNotMatch(devHttp, /\bpub\(super\) enum DxDevWireRequestError\b/);
  assert.doesNotMatch(devHttp, /\bpub\(super\) fn read_http_wire_request\b/);

  assert.match(devTinyServer, /read_http_wire_request\(&mut stream\)/);
  assert.match(devTinyServer, /dev_wire_request_error_response\(&error\)/);
  assert.doesNotMatch(devTinyServer, /let mut buffer = \[0; 8192\];/);
  assert.doesNotMatch(cliMod, /\bfn read_http_wire_request\b/);
  assert.doesNotMatch(cliMod, /dx-dev-request-too-large/);
});
