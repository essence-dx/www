import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath) {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

test("dx dev reuses an existing DX dev server instead of falling forward to a duplicate port", () => {
  const devOptions = readRepoFile("dx-www/src/cli/dev_options.rs");
  const devCommand = readRepoFile("dx-www/src/cli/dev_command.rs");

  assert.match(devOptions, /\bpub\(super\) enum DxDevServerBinding\b/);
  assert.match(devOptions, /\bpub\(super\) struct DxDevExistingServer\b/);
  assert.match(devOptions, /pub\(super\) probe_url: String/);
  assert.match(devOptions, /\bfn probe_existing_dx_dev_server\b/);
  assert.match(devOptions, /DX_HOT_RELOAD_VERSION_ENDPOINT/);
  assert.match(devOptions, /DX_HOT_RELOAD_PROTOCOL/);
  assert.match(devOptions, /DX_HOT_RELOAD_SOURCE/);
  assert.match(devOptions, /\bfn response_has_dx_hot_reload_poll_header\b/);
  assert.match(devOptions, /x-dx-hot-reload/);
  assert.match(devOptions, /response_has_dx_hot_reload_poll_header\(response\)/);
  assert.match(devOptions, /bind_dev_listener_rejects_protocol_body_without_dx_hot_reload_header/);
  assert.match(devOptions, /DxDevServerBinding::Existing/);
  assert.match(devOptions, /DxDevServerBinding::Bound/);
  assert.match(devOptions, /dev_probe_host/);
  assert.match(devOptions, /127\.0\.0\.1/);
  assert.match(devOptions, /\bfn occupied_explicit_dev_port_error\b/);
  assert.match(devOptions, /did not answer as a DX dev server/);
  assert.match(devOptions, /No duplicate dev server was started/);
  assert.match(devOptions, /choose a different --port/);

  assert.match(devCommand, /match bind_dev_listener\(&options\.host, options\.port, options\.port_explicit\)\?/);
  assert.match(devCommand, /DxDevServerBinding::Existing\(existing\)/);
  assert.match(devCommand, /No duplicate dev server started/);
  assert.match(devCommand, /Verified by: \{\}/);
  assert.match(devCommand, /DxDevServerBinding::Bound \{ listener, port \}/);
  assert.doesNotMatch(devCommand, /let \(listener, port\) = bind_dev_listener/);
});
