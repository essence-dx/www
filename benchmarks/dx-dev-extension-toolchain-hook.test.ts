const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("dx dev TSX edits run style and icon toolchain before hot reload publish", () => {
  const axumServer = read("dx-www/src/dev/axum_server.rs");
  const devMod = read("dx-www/src/dev/mod.rs");
  const extensionToolchain = read("dx-www/src/dev/extension_toolchain.rs");
  const watcher = read("dx-www/src/dev/watcher.rs");
  const templateDx = read("examples/template/dx");

  assert.match(axumServer, /run_dx_extension_toolchain_for_changed_paths\(&project_root, &change\.paths\)/);
  assert.match(axumServer, /super::extension_toolchain::run_dx_extension_toolchain_for_changed_paths/);
  assert.match(devMod, /mod extension_toolchain;/);
  assert.match(extensionToolchain, /DxConfig::load_project\(project_root\)/);
  assert.match(extensionToolchain, /MachineFormat::new/);
  assert.match(extensionToolchain, /machine_to_document/);
  assert.match(extensionToolchain, /SerializerOutputConfig::new\(\)/);
  assert.match(extensionToolchain, /is_serializer_source_path/);
  assert.match(extensionToolchain, /file_name\(\)\.and_then\(\|name\| name\.to_str\(\)\) == Some\("dx"\)/);
  assert.match(extensionToolchain, /extension\.eq_ignore_ascii_case\("sr"\)/);
  assert.match(extensionToolchain, /contains_classname_marker/);
  assert.match(extensionToolchain, /contains_icon_marker/);
  assert.match(extensionToolchain, /configured_icon_markers/);
  assert.match(extensionToolchain, /unwrap_or_else\(default_watch_extensions\)/);
  assert.match(extensionToolchain, /\("style", \["tsx", "jsx", "mdx", "html", "css"\]\.as_slice\(\)\)/);
  assert.match(extensionToolchain, /\("icons", \["tsx"\]\.as_slice\(\)\)/);
  assert.match(extensionToolchain, /\("imports", \["tsx"\]\.as_slice\(\)\)/);
  assert.match(extensionToolchain, /name:\s*"serializer",\s*args:\s*&\["serializer"\]/s);
  assert.match(extensionToolchain, /name:\s*"style",\s*args:\s*&\["style", "build", "--json"\]/s);
  assert.match(extensionToolchain, /name:\s*"icons",\s*args:\s*&\["icons", "sync", "--json"\]/s);
  assert.match(extensionToolchain, /name:\s*"imports",\s*args:\s*&\["imports", "sync", "--json"\]/s);
  assert.match(extensionToolchain, /\.dx\/run\/dev-extension-toolchain\.sr/);
  assert.match(extensionToolchain, /\.dx\/receipts\/run\/dev-extension-toolchain\.json/);
  assert.match(extensionToolchain, /"policy",\s*sr_string\("dx config driven/s);
  assert.doesNotMatch(extensionToolchain, /fn dx_string_config/);
  assert.match(watcher, /path_file_name_is\(relative, "dx"\)/);
  assert.match(watcher, /serializer_source_change/);
  assert.match(watcher, /"serializer" \| "run" \| "receipts" \| "www" \| "build"/);
  assert.doesNotMatch(watcher, /"\.dx\/run"/);
  assert.doesNotMatch(templateDx, /watch\[tool extensions\]/);
  assert.match(templateDx, /style\(\s*mode=generated-css/s);
  assert.match(templateDx, /icons\(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components\/icons\)/);
  assert.doesNotMatch(axumServer, /wwtsx|tseq/);
});
