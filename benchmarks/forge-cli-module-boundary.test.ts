const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const cliDirCandidates = [
  path.join(root, "dx-www", "src", "cli"),
  path.join(root, "www", "dx-www", "src", "cli"),
];
const cliDir = cliDirCandidates.find((candidate) => fs.existsSync(candidate));

assert.ok(cliDir, "expected dx-www CLI source directory to exist");

const modPath = path.join(cliDir, "mod.rs");

test("portable CI snippets report builder lives in a focused CLI module", () => {
  const modSource = fs.readFileSync(modPath, "utf8");
  const modulePath = path.join(cliDir, "forge_ci_snippets.rs");
  const moduleSource = fs.existsSync(modulePath) ? fs.readFileSync(modulePath, "utf8") : "";

  assert.ok(fs.existsSync(modulePath), "expected forge_ci_snippets.rs module to exist");
  assert.match(modSource, /^mod forge_ci_snippets;/m);
  assert.doesNotMatch(modSource, /fn build_forge_ci_snippets_report\b/);
  assert.doesNotMatch(modSource, /fn forge_ci_snippets_markdown\b/);
  assert.doesNotMatch(modSource, /struct DxForgeCiSnippetsReport\b/);
  assert.match(moduleSource, /pub\(super\) fn build_forge_ci_snippets_report\b/);
  assert.match(moduleSource, /pub\(super\) fn forge_ci_snippets_markdown\b/);
});

test("release triage report builder lives in a focused CLI module", () => {
  const modSource = fs.readFileSync(modPath, "utf8");
  const modulePath = path.join(cliDir, "forge_release_triage.rs");
  const moduleSource = fs.existsSync(modulePath) ? fs.readFileSync(modulePath, "utf8") : "";

  assert.ok(fs.existsSync(modulePath), "expected forge_release_triage.rs module to exist");
  assert.match(modSource, /^mod forge_release_triage;/m);
  assert.doesNotMatch(modSource, /fn build_forge_release_triage_report\b/);
  assert.doesNotMatch(modSource, /fn forge_release_triage_markdown\b/);
  assert.doesNotMatch(modSource, /struct DxForgeReleaseTriageReport\b/);
  assert.match(moduleSource, /pub\(super\) fn build_forge_release_triage_report\b/);
  assert.match(moduleSource, /pub\(super\) fn forge_release_triage_markdown\b/);
});

test("release bundle inspector report builder lives in a focused CLI module", () => {
  const modSource = fs.readFileSync(modPath, "utf8");
  const modulePath = path.join(cliDir, "forge_release_bundle_inspect.rs");
  const moduleSource = fs.existsSync(modulePath) ? fs.readFileSync(modulePath, "utf8") : "";

  assert.ok(fs.existsSync(modulePath), "expected forge_release_bundle_inspect.rs module to exist");
  assert.match(modSource, /^mod forge_release_bundle_inspect;/m);
  assert.doesNotMatch(modSource, /fn build_forge_release_bundle_inspect_report\b/);
  assert.doesNotMatch(modSource, /fn forge_release_bundle_inspect_markdown\b/);
  assert.doesNotMatch(modSource, /struct DxForgeReleaseBundleInspectReport\b/);
  assert.match(moduleSource, /pub\(super\) fn build_forge_release_bundle_inspect_report\b/);
  assert.match(moduleSource, /pub\(super\) fn forge_release_bundle_inspect_markdown\b/);
});
