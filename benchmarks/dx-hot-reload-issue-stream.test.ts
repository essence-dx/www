import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("DX hot reload issue stream is source-owned and payload-visible", () => {
  const protocol = read("dx-www/src/hot_reload_protocol.rs");
  const manifest = read("dx-www/src/cli/studio_manifest/hot_reload_manifest.rs");
  const client = read("dx-www/src/cli/dev_hot_reload_client.rs");

  assert.match(protocol, /DX_HOT_RELOAD_ISSUE_INSTRUCTION: &str = "report-issue"/);
  assert.match(protocol, /DX_HOT_RELOAD_CLEAR_ISSUE_INSTRUCTION: &str = "clear-issue"/);
  assert.match(protocol, /DX_HOT_RELOAD_ISSUE_MODE: &str = "diagnostic-overlay"/);
  assert.match(
    protocol,
    /DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA: &str = "dx\.dev\.hot_reload\.issue_receipt"/,
  );
  assert.match(protocol, /DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT: u64 = 1/);
  assert.match(protocol, /pub\(crate\) struct DxHotReloadIssue/);
  assert.match(protocol, /pub\(crate\) fn dx_hot_reload_issue_payload/);
  assert.match(protocol, /pub\(crate\) fn dx_hot_reload_issue_recovery_payload/);
  assert.match(protocol, /"instruction": \{\s+"type": instruction_type,/);
  assert.match(protocol, /payload\.insert\("issue_receipt"\.to_string\(\), issue_receipt\.clone\(\)\)/);
  assert.match(protocol, /payload\.insert\("issues"\.to_string\(\), issue_receipt\["issues"\]\.clone\(\)\)/);
  assert.match(protocol, /"issue_stream"\.to_string\(\), json!\(hot_reload\)/);
  assert.match(protocol, /event_stream\.insert\("issue_stream"\.to_string\(\), json!\(hot_reload\)\)/);
  assert.match(protocol, /event_stream\.insert\(\s+"issue_receipt_schema"\.to_string\(\),\s+json!\(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA\),/);
  assert.match(protocol, /payload\["event_stream"\]\["issue_stream"\], true/);
  assert.match(protocol, /payload\["event_stream"\]\["issue_receipt_schema"\],\s+DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA/);
  assert.match(protocol, /"partial_module_updates": false/);
  assert.match(protocol, /DX_HOT_RELOAD_NODE_RUNTIME_BOUNDARY/);
  assert.doesNotMatch(protocol, /=\s*"[^"]*\.v1"/);
  assert.doesNotMatch(protocol, /turbopack-hmr|turbopack-subscribe|_next\/hmr/);

  assert.match(manifest, /fn issue_stream_contract\(\) -> Value/);
  assert.match(manifest, /"schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA/);
  assert.match(manifest, /"format": DX_HOT_RELOAD_ISSUE_RECEIPT_FORMAT/);
  assert.match(manifest, /"instruction": DX_HOT_RELOAD_ISSUE_INSTRUCTION/);
  assert.match(manifest, /"mode": DX_HOT_RELOAD_ISSUE_MODE/);
  assert.match(manifest, /"partial_module_updates": false/);
  assert.match(manifest, /"turbopack_protocol": false/);
  assert.doesNotMatch(manifest, /=\s*"[^"]*\.v1"/);

  assert.match(client, /const dxIssueOverlayPayload = \(data\) =>/);
  assert.match(client, /const dxIssueResourceMatchesCurrentPage = \(payload\) =>/);
  assert.match(client, /dxStylesheetMatchesPayload\(payload\)/);
  assert.match(client, /dxAssetReferenceMatchesPayload\(payload\)/);
  assert.match(client, /instructionType === "report-issue"/);
  assert.match(client, /instructionType === "clear-issue"/);
  assert.match(
    client,
    /instructionType === "clear-issue" && !dxIssueResourceMatchesCurrentPage\(data\)/,
  );
  assert.match(client, /window\.__DX_HIDE_ERROR__\(\)/);
  assert.match(
    client,
    /instructionType === "report-issue" && !dxIssueResourceMatchesCurrentPage\(data\)/,
  );
  assert.match(client, /window\.__DX_SHOW_ERROR__\(dxIssueOverlayPayload\(data\)\)/);
  assert.match(client, /issue\.code_frame \|\| issue\.codeFrame/);
  const unrelatedRestartGuard = client.indexOf(
    'instructionType === "restart" && !dxRouteResourceMatchesCurrentPage(data)',
  );
  const hideOverlay = client.indexOf("window.__DX_HIDE_ERROR__()", unrelatedRestartGuard);
  assert.notEqual(unrelatedRestartGuard, -1, "restart frames must stay route-scoped");
  assert.notEqual(hideOverlay, -1, "successful current-route recovery must hide the overlay");
  assert.ok(
    unrelatedRestartGuard < hideOverlay,
    "unrelated route recovery frames must advance the token without hiding this page overlay",
  );
  assert.doesNotMatch(client, /turbopack-hmr|turbopack-subscribe|_next\/hmr/);
});
