import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repo = path.resolve(__dirname, "..");
const cliModPath = path.join(repo, "dx-www", "src", "cli", "mod.rs");
const releaseDashboardPath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_release_dashboard.rs",
);

test("Forge release dashboard rendering lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_release_dashboard;/);

  for (const forbidden of [
    "fn forge_release_dashboard_markdown(",
    "fn forge_release_dashboard_check_row(",
    "fn forge_release_dashboard_failure_summary(",
  ]) {
    assert.equal(
      cliMod.includes(forbidden),
      false,
      `${forbidden} should be owned by forge_release_dashboard.rs`,
    );
  }

  const releaseDashboard = fs.readFileSync(releaseDashboardPath, "utf8");
  for (const required of [
    "pub(super) fn forge_release_dashboard_markdown(",
    "fn forge_release_dashboard_check_row(",
    "pub(super) fn forge_release_dashboard_failure_summary(",
    "# DX Forge Release Dashboard",
    "## Checks",
    "## Artifact Inputs",
    "## Release Proof",
    "No release-dashboard findings for the configured threshold.",
    "DX Forge release-dashboard did not pass:",
    "markdown_table_cell",
  ]) {
    assert.match(releaseDashboard, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
