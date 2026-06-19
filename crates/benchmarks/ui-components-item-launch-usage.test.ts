const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("launch package catalog exposes the source-owned UI Components item primitive", () => {
  const catalog = read("examples/template/package-catalog.ts");
  const help = read("dx-www/src/cli/help_text.rs");

  assert.match(catalog, /packageId: "ui\/item"/);
  assert.match(catalog, /officialName: "UI Components"/);
  assert.match(help, /dx add ui\/item/);
});
