import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("dx dev has a typed server mode contract and automatic resolver", () => {
  const config = read("dx-www/src/config.rs");
  const configSource = read("dx-www/src/config_source.rs");
  const devOptions = read("dx-www/src/cli/dev_options.rs");
  const devCommand = read("dx-www/src/cli/dev_command.rs");
  const devServerMode = read("dx-www/src/cli/dev_server_mode.rs");
  const devTinyServer = read("dx-www/src/cli/dev_tiny_server.rs");
  const cliMod = read("dx-www/src/cli/mod.rs");

  assert.match(config, /pub enum DxDevServerMode/);
  for (const variant of ["Auto", "Axum", "MayMinihttp"]) {
    assert.match(config, new RegExp(`\\b${variant}\\b`));
  }
  assert.match(configSource, /"dev\.server_mode", "dev\.server"/);
  assert.match(devOptions, /"--server" \| "--server-mode"/);
  assert.match(devOptions, /server_mode: DxDevServerMode/);

  assert.match(cliMod, /^mod dev_server_mode;$/m);
  assert.match(cliMod, /^mod dev_tiny_server;$/m);
  assert.match(devCommand, /resolve_dev_server_mode\(/);
  assert.match(devCommand, /serve_tiny_dev_server\(/);
  assert.match(devCommand, /Dev server: \{\} \(\{\}\)/);

  assert.match(devServerMode, /hot reload uses Axum dev endpoints/);
  assert.match(devServerMode, /devtools use Axum dev endpoints/);
  assert.match(devServerMode, /static project without Axum-only dev capabilities/);
  assert.doesNotMatch(devServerMode, /template-axum|template-may-minihttp|examples\/template/);

  assert.match(devTinyServer, /TcpListener/);
  assert.match(devTinyServer, /read_http_wire_request/);
  assert.match(devTinyServer, /handle_http_wire_response_cached/);
});

test("tiny server reuses bounded DX wire helpers in normal dev-server builds", () => {
  const devWire = read("dx-www/src/cli/dev_wire.rs");
  const axumServer = read("dx-www/src/dev/axum_server.rs");

  assert.doesNotMatch(
    devWire,
    /#\[cfg\(not\(feature = "dev-server"\)\)\]/,
    "wire helpers should be callable from the normal dev-server build",
  );
  assert.match(devWire, /pub\(super\) fn read_http_wire_request/);
  assert.match(devWire, /pub\(super\) fn dev_wire_response_bytes/);
  assert.match(devWire, /DX_DEV_MAX_REQUEST_BYTES/);

  assert.match(
    axumServer,
    /!request_should_read_body\(parts\.method\.as_str\(\), &parts\.headers\)[\s\S]*Bytes::new\(\)/,
    "GET and HEAD requests should skip Axum body collection before app dispatch",
  );
});

test("new server mode templates are not hardcoded into Rust template content", () => {
  const rustSources = [
    "dx-www/src/cli/default_template_sources.rs",
    "dx-www/src/cli/default_template_materializer.rs",
    "dx-www/src/cli/new_command.rs",
  ]
    .map(read)
    .join("\n");

  assert.doesNotMatch(rustSources, /examples\/template-axum/);
  assert.doesNotMatch(rustSources, /examples\/template-may-minihttp/);
  assert.doesNotMatch(rustSources, /dx-www-template-axum/);
  assert.doesNotMatch(rustSources, /dx-www-template-may-minihttp/);
});
