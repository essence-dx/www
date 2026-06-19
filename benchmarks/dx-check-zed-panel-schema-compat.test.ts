import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("DX-WWW accepts current and legacy dx-check Zed panel schemas", () => {
  const panelSource = read("core/src/ecosystem/dx_check_receipt/panel.rs");
  const legacySchemaTests = read("core/src/ecosystem/dx_check_receipt/panel_parts/tests_a.rs");
  const currentSchemaFixtures = read("core/src/ecosystem/dx_check_receipt/panel_parts/tests_b.rs");

  assert.match(
    panelSource,
    /pub const DX_CHECK_ZED_PANEL_SCHEMA_VERSION: &str = "dx\.check\.zed_panel\.v1";/,
  );
  assert.match(
    panelSource,
    /DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION: &str = "dx\.check\.zed_panel";/,
  );
  assert.match(panelSource, /fn is_supported_zed_panel_schema\(schema_version: &str\) -> bool/);
  assert.match(panelSource, /is_supported_zed_panel_schema\(&receipt\.zed\.schema_version\)/);
  assert.match(currentSchemaFixtures, /"schema_version": "dx\.check\.zed_panel\.v1"/);
  assert.match(
    legacySchemaTests,
    /dx_check_latest_panel_still_reads_legacy_unversioned_zed_receipt/,
  );
  assert.match(legacySchemaTests, /replace\("dx\.check\.zed_panel\.v1", "dx\.check\.zed_panel"\)/);
});
