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

test("dx imports generates and checks imports.d.ts", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");
  const help = read("dx-www/src/cli/help_text.rs");

  for (const marker of [
    'const DEFAULT_DECLARATIONS_PATH: &str = ".dx/imports/imports.d.ts"',
    "fn imports_declarations",
    "write_project_file(\n        project,\n        &imports.config.declarations",
    "stale_declarations",
    "declare module",
    "declare global",
    "export {};",
  ]) {
    assert.match(imports, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(help, /\.dx\/imports\/imports\.d\.ts/);
});
