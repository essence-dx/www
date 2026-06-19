const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const neutralPackageId = "dx-www/template-shell";
const stalePackageId = "www" + "/www-template";
const neutralPackageSlug = "dx-www-template-shell";
const stalePackageSlug = "www-www-template";
const neutralPackageName = "@dx-www/template-shell";
const stalePackageName = "@dx-www" + "/www-template";

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("launch template package identity is source-owned and event-neutral", () => {
  const packageJson = readRequiredFile("examples/template/package.json");
  const launchShell = readRequiredFile("examples/template/template-shell.tsx");
  const routeContract = readRequiredFile("examples/template/template-route-contract.ts");
  const shadcnContract = readRequiredFile("examples/template/shadcn-dashboard-controls-contract.tsx");
  const launchReadinessBundle = readRequiredFile("dx-www/src/cli/launch_readiness_bundle.rs");
  const templateReadiness = readRequiredFile("dx-www/src/cli/template_readiness.rs");
  const studioManifest = readRequiredFile("dx-www/src/cli/studio_manifest.rs");
  const cli = readRequiredFile("dx-www/src/cli/mod.rs");
  const slugSourceLabels = new Set(["route contract", "template readiness", "cli"]);

  assert.equal(JSON.parse(packageJson).name, neutralPackageName);

  for (const [label, source] of [
    ["package.json", packageJson],
    ["launch shell", launchShell],
    ["route contract", routeContract],
    ["shadcn dashboard contract", shadcnContract],
    ["launch readiness bundle", launchReadinessBundle],
    ["template readiness", templateReadiness],
    ["studio manifest", studioManifest],
    ["cli", cli],
  ]) {
    assert.match(source, new RegExp(neutralPackageId.replace("/", "\\/")), `${label} should use ${neutralPackageId}`);
    assert.doesNotMatch(source, new RegExp(stalePackageId.replace("/", "\\/")), `${label} still uses ${stalePackageId}`);
    assert.doesNotMatch(source, new RegExp(stalePackageSlug), `${label} still uses ${stalePackageSlug}`);
    assert.doesNotMatch(source, new RegExp(stalePackageName.replace("/", "\\/")), `${label} still uses ${stalePackageName}`);
    if (slugSourceLabels.has(label)) {
      assert.match(source, new RegExp(neutralPackageSlug), `${label} should use ${neutralPackageSlug}`);
    }
  }
});
