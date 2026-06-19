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

test("dx imports preserves IDE type evidence and skipped entries", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");

  for (const marker of [
    "typed_entry_count",
    "untyped_entry_count",
    "skipped_entries",
    "SkippedImportEntry",
    "no public exports found",
    "export type",
    "typeof import",
    "source: normalize_relative_path(project, &file)",
  ]) {
    assert.match(imports, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(imports, /:\s*any[;\n]/);
});
