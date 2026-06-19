const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

test("dx imports scans components, composables, and utils deterministically", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");
  const config = read("dx-www/src/config.rs");
  const configSource = read("dx-www/src/config_source.rs");

  for (const marker of [
    '["components", "composables", "utils"]',
    "scan_roots",
    "kind: import_kind(root, &source_path).to_string()",
    '"component"',
    '"composable"',
    '"utility"',
    "entries.sort_by(import_entry_order)",
    "exports: Vec<ExportSymbol>",
    "default_export_name",
  ]) {
    assert.match(`${imports}\n${config}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(config, /pub scan_roots: Vec<String>/);
  assert.match(configSource, /imports\.scan_roots/);
});
