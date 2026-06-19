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

test("dx imports writes source-owned sr receipts with scan and usage evidence", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");

  for (const marker of [
    ".dx/imports/sync.sr",
    ".dx/imports/check.sr",
    "write_imports_sync_sr",
    "write_imports_check_sr",
    "scan_roots",
    "used_roots",
    "used_symbol_count",
    "typed_entry_count",
    "untyped_entry_count",
    "skipped_entry_count",
    "source_hash",
    "legacy_json",
  ]) {
    assert.match(imports, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
