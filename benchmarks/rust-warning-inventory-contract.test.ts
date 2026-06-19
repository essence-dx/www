import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("hot reload resource normalizer is not compiled only for lib tests", () => {
  const source = readRepoFile("dx-www/src/hot_reload_protocol.rs");
  const declaration = "pub(crate) fn dx_hot_reload_normalize_resource_id";
  const declarationIndex = source.indexOf(declaration);

  assert.notEqual(declarationIndex, -1, "hot reload resource normalizer must exist");
  const cfgWindow = source.slice(Math.max(0, declarationIndex - 120), declarationIndex);

  assert.match(cfgWindow, /#\[cfg\(feature = "dev-server"\)\]/);
  assert.doesNotMatch(
    cfgWindow,
    /cfg\(any\(feature = "dev-server", test\)\)|cfg\(test\)/,
    "the dev-server-only normalizer must not be compiled into plain lib tests",
  );
});

test("CLI dev bridge does not re-export stale parse helper wrappers", () => {
  const devBridge = readRepoFile("dx-www/src/cli/dev_bridge.rs");
  const devHttp = readRepoFile("dx-www/src/cli/dev_http.rs");

  assert.doesNotMatch(
    devBridge,
    /\bpub\(super\)\s+fn\s+parse_http_request\b/,
    "parse_http_request belongs in dev_http/dev_response, not in the CLI bridge",
  );
  assert.match(devHttp, /\bpub\(super\)\s+fn\s+parse_http_request\b/);
});

test("diagnostic issue numeric extraction uses the live path-aware helper only", () => {
  const devFeedback = readRepoFile("dx-www/src/dev/dev_feedback.rs");

  assert.doesNotMatch(
    devFeedback,
    /\bfn\s+issue_usize\s*\(/,
    "stale issue_usize helper should stay removed after path-aware diagnostics landed",
  );
  assert.match(devFeedback, /\bfn\s+issue_usize_at_paths\s*\(/);
});

test("DxError diagnostic mapping has no unreachable catch-all arm", () => {
  const errorSource = readRepoFile("dx-www/src/error.rs");
  const start = errorSource.indexOf("fn to_dx_diagnostic(&self)");
  const end = errorSource.indexOf("/// Create a parse error with source context.", start);

  assert.notEqual(start, -1, "DxError::to_dx_diagnostic must exist");
  assert.ok(end > start, "DxError::to_dx_diagnostic block must be locatable");

  const diagnosticMapping = errorSource.slice(start, end);
  assert.doesNotMatch(
    diagnosticMapping,
    /\n\s*_ => None,\s*\n/,
    "DxError::to_dx_diagnostic is exhaustive; a catch-all arm becomes unreachable",
  );
});
