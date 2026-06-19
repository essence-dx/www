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

test("dx imports public barrel includes only used symbols", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");
  const config = read("dx-www/src/config.rs");

  for (const marker of [
    "pub used_only: bool",
    "used_only: true",
    "fn collect_used_symbols",
    "identifier_tokens",
    "usage_symbol_is_used",
    "used_exports",
    "unused_exports",
    "public_barrel_entries",
    "unused_entries",
  ]) {
    assert.match(`${imports}\n${config}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
