import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const configPath = join(repoRoot, "dx-www", "src", "config.rs");
const configSourcePath = join(repoRoot, "dx-www", "src", "config_source.rs");
const adoptionPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "forge_adoption_beta.rs",
);
const cliTestsPath = join(repoRoot, "dx-www", "src", "cli", "tests", "part_01.rs");
const packageTemplatesPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_registry_parts",
  "package_templates.rs",
);
const packageLanesPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_registry_parts",
  "package_lanes.rs",
);
const scorecardPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_scorecard.rs",
);

const config = readFileSync(configPath, "utf8");
const configSource = readFileSync(configSourcePath, "utf8");
const adoption = readFileSync(adoptionPath, "utf8");
const cliTests = readFileSync(cliTestsPath, "utf8");
const packageTemplates = readFileSync(packageTemplatesPath, "utf8");
const packageLanes = readFileSync(packageLanesPath, "utf8");
const scorecard = readFileSync(scorecardPath, "utf8");

test("Forge UI is the primary tooling config name", () => {
  assert.match(config, /pub forge_ui: ForgeUiToolingConfig/);
  assert.match(config, /pub struct ForgeUiToolingConfig/);
  assert.match(config, /impl Default for ForgeUiToolingConfig/);
  assert.doesNotMatch(config, /pub shadcn: ShadcnToolingConfig/);
  assert.doesNotMatch(config, /pub struct ShadcnToolingConfig/);
});

test("legacy shadcn tooling config remains a compatibility alias only", () => {
  assert.match(config, /#\[serde\(alias = "shadcn"\)\]/);
  assert.match(configSource, /"tooling\.forge_ui\.style"[\s\S]*"tooling\.shadcn\.style"/);
  assert.match(configSource, /"tooling\.forge_ui\.aliases\.ui"[\s\S]*"tooling\.shadcn\.aliases\.ui"/);
  assert.match(config, /load_project_keeps_legacy_shadcn_tooling_aliases/);
  assert.match(config, /from_toml_keeps_legacy_shadcn_tooling_alias/);
});

test("generated Forge adoption config uses Forge UI vocabulary", () => {
  assert.match(adoption, /tooling\.forge_ui\.style=\\"new-york\\"/);
  assert.doesNotMatch(adoption, /tooling\.shadcn\.style=\\"new-york\\"/);
});

test("dx new tests assert the Forge UI config surface", () => {
  assert.match(cliTests, /config\.tooling\.forge_ui\.style/);
  assert.doesNotMatch(cliTests, /config\.tooling\.shadcn\.style/);
});

test("generated UI package docs use Forge UI Components as the product name", () => {
  for (const surface of [
    "Button",
    "Badge",
    "Label",
    "Separator",
    "Field",
    "Item",
    "Card",
    "Alert",
    "Avatar",
    "Skeleton",
  ]) {
    assert.match(packageTemplates, new RegExp(`# DX Forge UI Components: ${surface}`));
  }

  assert.match(packageTemplates, /Compatibility id: `shadcn\/ui\/button`/);
  assert.match(packageTemplates, /Upstream provenance: shadcn-ui v4/);
  assert.doesNotMatch(packageTemplates, /# DX Forge shadcn\/ui/);
  assert.doesNotMatch(packageTemplates, /This source-owned shadcn\/ui v4/);
  assert.doesNotMatch(packageTemplates, /This source-owned shadcn-style/);
  assert.doesNotMatch(packageTemplates, /full shadcn registry sync/);
});

test("Forge UI package metadata uses a Forge-owned generator identity", () => {
  assert.match(packageLanes, /=> "dx-forge\/ui-components"/);
  assert.doesNotMatch(packageLanes, /=> "dx-forge\/shadcn-ui"/);
  assert.doesNotMatch(packageLanes, /shadcn-style button package/);
  assert.doesNotMatch(packageLanes, /shadcn-style card package/);
  assert.doesNotMatch(packageLanes, /shadcn input package/);
  assert.doesNotMatch(packageLanes, /shadcn textarea package/);
  assert.match(packageLanes, /UI Components button surface/);
  assert.match(packageLanes, /UI Components textarea surface/);
});

test("Forge scorecard presents UI Components while preserving shadcn-ui provenance", () => {
  assert.match(scorecard, /Editable UI Components button source based on shadcn-ui v4/);
  assert.match(scorecard, /Editable UI Components textarea source based on the shadcn-ui v4 registry shape/);
  assert.match(scorecard, /not full upstream registry parity yet/);
  assert.doesNotMatch(scorecard, /Editable shadcn/);
  assert.doesNotMatch(scorecard, /shadcn-style/);
  assert.doesNotMatch(scorecard, /not full shadcn registry/);
});
