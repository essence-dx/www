const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("dx forge remove has safe archive-backed command and receipt coverage", () => {
  const security = read("core/src/ecosystem/forge_security.rs");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(security, /RemoveDryRun/);
  assert.match(security, /RemoveWrite/);
  assert.match(security, /pub struct DxForgeRemoveOutcome/);
  assert.match(security, /pub fn plan_forge_remove_variant/);
  assert.match(security, /pub fn write_forge_remove_dry_run_variant/);
  assert.match(security, /pub fn write_forge_remove_variant/);
  assert.match(security, /fn write_forge_dry_run_receipt/);
  assert.match(security, /archive_root/);
  assert.match(security, /remove_file/);
  assert.match(security, /forge-remove-local-edit/);
  assert.match(security, /archive-before-remove/);
  assert.match(security, /manifest\.remove_package/);

  assert.match(cli, /"remove" => self\.cmd_forge_remove/);
  assert.match(cli, /fn cmd_forge_remove/);
  assert.match(cli, /write_forge_remove_dry_run_variant/);
  assert.match(cli, /write_forge_remove_variant/);
  assert.match(cli, /remove_outcome_markdown/);
  assert.match(cli, /dx forge remove <package>/);
});
