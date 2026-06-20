const fs = require("node:fs");
const path = require("node:path");

const repo = path.resolve(__dirname, "..");
const wwwTemplate = path.join(repo, "examples", "template");

function readText(relativePath) {
  return fs.readFileSync(path.join(repo, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

const rootDx = readText("examples/template/dx");
assert(
  rootDx.includes("project(name=dx-www-template version=0.1.0 kind=www-app)"),
  "template root dx must declare the serializer project header",
);
assert(
  rootDx.includes("forge(policy=forge-first-no-node-modules)"),
  "root dx must keep the Forge policy in the compact serializer config",
);
assert(
  rootDx.includes("check(score_scale=500 lighthouse=true)"),
  "root dx must keep the 500-point check contract in the compact serializer config",
);
assert(
  !rootDx.includes("packages[id version source surfaces](") &&
    !rootDx.includes("tools[name command enabled output]("),
  "root dx must not overwhelm first users with generated package/tool receipt rows",
);

const registryManifest = readJson(
  "examples/template/.dx/forge/registry/local/packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json",
);
assert(
  registryManifest.source["root-dx"],
  "local registry package must be published from the root dx manifest",
);
assert(
  registryManifest.allow_selective_imports === true,
  "local registry package must allow selective imports",
);
assert(
  registryManifest.exports.some((entry) => entry.name === "button"),
  "local registry package must expose the button export",
);

const statusReceipt = readJson(
  "examples/template/.dx/receipts/forge/status-latest.json",
);
assert(
  statusReceipt.status === "remote-health-blocked",
  "Forge public status should honestly block remote install while R2 health is unmeasured",
);
assert(
  statusReceipt.root_package?.status !== "invalid",
  "Forge status must not parse the compact project dx config as an invalid package manifest",
);
assert(
  statusReceipt.local_registry.present === true,
  "Forge status must still see the local registry when the root dx file is project config",
);
assert(
  statusReceipt.warnings.some((warning) => /R2 remote is not configured/.test(warning)),
  "Forge status must keep the missing remote configuration warning visible",
);

for (const relativePath of [
  "examples/template/.dx/forge/registry/local/receipts/20260522T130612Z-shadcn-ui-button.json",
  "examples/template/.dx/forge/receipts/20260522T130626134038800Z-shadcn-ui-button--variant-export-button.json",
  "examples/template/.dx/forge/receipts/20260522T130639403843800Z-shadcn-ui-button--variant-export-button.json",
  "examples/template/.dx/forge/receipts/safety/shadcn-ui-button-archive.json",
]) {
  assert(fs.existsSync(path.join(repo, relativePath)), `${relativePath} must exist`);
}

const contract = readText("examples/template/forge-golden-path-contract.ts");
for (const marker of [
  "dx.forge.golden_path_status",
  "root-dx-manifest",
  "local-publish",
  "selective-visible-install",
  "status-lock",
  "update-dry-run",
  "accepted-update",
  "remove-plan",
  "archive-restore",
  "dx-check-score",
  "dashboard-row",
  "dxCheckScore: 89",
  'dxCheckTraffic: "score-gated"',
]) {
  assert(contract.includes(marker), `golden path contract missing ${marker}`);
}

const panel = readText("examples/template/forge-golden-path-panel.tsx");
assert(
  panel.includes("data-dx-forge-golden-path-step-state"),
  "dashboard panel must expose step states as data attributes",
);

const launchPg = readText("tools/launch/runtime-template/pages/index.html");
for (const marker of [
  'data-dx-component="forge-golden-path-status"',
  'data-dx-forge-golden-path-state="partial"',
  'data-dx-forge-golden-path-real-steps="10"',
  'data-dx-forge-golden-path-partial-steps="0"',
  'data-dx-forge-golden-path-dx-check-score="89"',
  'data-dx-forge-golden-path-dx-check-traffic="score-gated"',
  'data-dx-forge-golden-path-step="archive-restore"',
  'data-dx-forge-golden-path-step-state="real"',
  'data-dx-safety-archive-state="covered"',
  "100% rollback coverage",
]) {
  assert(launchPg.includes(marker), `static launch fixture missing ${marker}`);
}

console.log(
  JSON.stringify(
    {
      status: "ok",
      fixture: path.relative(repo, wwwTemplate),
      package: registryManifest.package_id,
      golden_path_steps: 10,
      golden_path_real_steps: 10,
      dx_check_score: 89,
      boundary:
        "local publish/add/update/remove/restore/status/dashboard and persisted source dry-run receipts are proven; browser/provider proof and governed binary refresh are still required",
    },
    null,
    2,
  ),
);
