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

test("dx check and dx build fail stale auto-import maps", () => {
  const imports = read("dx-www/src/cli/public_framework_tools/imports.rs");
  const cli = `${read("dx-www/src/cli/mod.rs")}\n${read("dx-www/src/cli/mod_parts/cli_core_impl.rs")}\n${read("dx-www/src/cli/mod_parts/cli_forge_commands_c.rs")}`;

  for (const marker of [
    "ensure_dx_imports_current_for_build",
    "imports_project_root",
    "strip_windows_verbatim_prefix",
    "resolve dx imports project root",
    "dx build blocked because auto-import artifacts are stale",
    "auto-import-artifacts-stale",
    "stale_declarations",
    "stale_sync_receipt",
    "Run dx imports sync",
  ]) {
    assert.match(`${imports}\n${cli}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
