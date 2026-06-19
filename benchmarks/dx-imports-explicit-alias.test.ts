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

test("dx imports exposes #imports and #components alias contracts", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");
  const config = read("dx-www/src/config.rs");
  const configSource = read("dx-www/src/config_source.rs");
  const help = read("dx-www/src/cli/help_text.rs");

  for (const marker of [
    '"#imports"',
    '"#components"',
    "pub aliases: Vec<String>",
    "imports.aliases",
    "declare module",
    "alias == \"#components\"",
  ]) {
    assert.match(
      `${imports}\n${config}\n${configSource}\n${help}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }
});
