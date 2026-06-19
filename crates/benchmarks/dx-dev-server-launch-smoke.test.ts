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

test("DX dev server launch smoke serves source-owned styles without node_modules", () => {
  const axumServer = read("dx-www/src/dev/axum_server.rs");

  assert.match(axumServer, /fn hot_reload_response\(\s*state: &DxDevAxumState,\s*resource_id: String,\s*include_body: bool,/);
  assert.match(axumServer, /path_without_query\(&path\) == DX_HOT_RELOAD_VERSION_ENDPOINT[\s\S]*hot_reload_response\(\s*&state,\s*dx_hot_reload_resource_from_path\(&path\),\s*parts\.method\.as_str\(\) == "GET",\s*\)/);
  assert.match(axumServer, /async fn axum_dev_router_serves_head_hot_reload_version_without_body_or_responder\(\)/);
  assert.match(axumServer, /\.method\("HEAD"\)[\s\S]*\.uri\("\/_dx\/hot-reload\/version\?resource=route%3A%2F"\)/);
  assert.match(axumServer, /assert!\(body\.is_empty\(\)\)[\s\S]*assert_eq!\(\*responder_hits\.lock\(\)\.unwrap\(\), 0\)/);
  assert.match(axumServer, /fn static_dev_asset_response\(\s*project_root: &Path,\s*request_path: &str,\s*include_body: bool,/);
  assert.match(axumServer, /parts\.method\.as_str\(\) == "GET" \|\| parts\.method\.as_str\(\) == "HEAD"/);
  assert.match(axumServer, /static_dev_asset_response\(&state\.project_root, &path, parts\.method\.as_str\(\) == "GET"\)/);
  assert.match(axumServer, /path\.strip_prefix\("\/styles\/"\)/);
  assert.match(axumServer, /path\.strip_prefix\('\/'\)/);
  assert.match(axumServer, /project_root\.join\("styles"\)\.join\(relative\)/);
  assert.match(axumServer, /project_root\.join\("public"\)\.join\(relative\)/);
  assert.match(axumServer, /Some\("ico"\) => "image\/x-icon"/);
  assert.match(axumServer, /Some\("webmanifest"\) => "application\/manifest\+json"/);
  assert.match(axumServer, /Some\("txt"\) => "text\/plain; charset=utf-8"/);
  assert.match(axumServer, /async fn axum_dev_router_serves_minimal_style_asset_before_responder\(\)/);
  assert.match(axumServer, /uri\("\/styles\/generated\.css\?dev=1"\)/);
  assert.match(axumServer, /assert_eq!\(\*responder_hits\.lock\(\)\.unwrap\(\), 0\)/);
  assert.match(axumServer, /async fn axum_dev_router_serves_head_static_asset_without_body_or_responder\(\)/);
  assert.match(axumServer, /\.method\("HEAD"\)/);
  assert.match(axumServer, /assert!\(body\.is_empty\(\)\)/);
  assert.match(axumServer, /root\.join\("public\/favicon\.ico"\)/);
  assert.match(axumServer, /root\.join\("public\/manifest\.webmanifest"\)/);
  assert.match(axumServer, /root\.join\("public\/robots\.txt"\)/);
  assert.match(axumServer, /tcp_get\(addr, "\/favicon\.ico\?dev=1"\)/);
  assert.match(axumServer, /content-type: image\/x-icon/);
  assert.match(axumServer, /tcp_get\(addr, "\/manifest\.webmanifest"\)/);
  assert.match(axumServer, /content-type: application\/manifest\+json/);
  assert.match(axumServer, /tcp_get\(addr, "\/robots\.txt"\)/);
  assert.match(axumServer, /content-type: text\/plain; charset=utf-8/);
  assert.match(axumServer, /tcp_get\(addr, "\/logo\.svg"\)/);
  assert.match(axumServer, /assert!\(!logo\.contains\("data-path=\\"\/logo\.svg\\""\)\)/);
  assert.doesNotMatch(
    axumServer,
    /project_root\.join\("node_modules"\)|project_root\.join\("_next"\)|_next\/static|turbopack-hmr|turbopack-subscribe/,
  );
});
