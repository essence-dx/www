const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "shadcn-ui");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("shadcn/ui powers a real /launch dashboard control workflow", () => {
  const upstreamButton = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "button.tsx"),
  );
  const upstreamItem = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "item.tsx"),
  );
  const upstreamLabel = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "label.tsx"),
  );
  const upstreamField = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "field.tsx"),
  );
  const upstreamCard = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "card.tsx"),
  );
  const component = read(
    path.join(root, "examples", "template", "shadcn-dashboard-controls.tsx"),
  );
  const contractPath = path.join(
    root,
    "examples",
    "www-template",
    "shadcn-dashboard-controls-contract.tsx",
  );
  assert.ok(fs.existsSync(contractPath), "expected shadcn dashboard controls contract file");
  const contract = read(contractPath);
  const shell = read(path.join(root, "examples", "template", "template-shell.tsx"));
  const routeContract = read(
    path.join(root, "examples", "template", "template-route-contract.ts"),
  );
  const studioContract = read(
    path.join(root, "examples", "template", "dx-studio-edit-contract.ts"),
  );
  const catalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
  const receiptPath = path.join(
    root,
    "examples",
    "www-template",
    ".dx",
    "forge",
    "receipts",
    "2026-05-22-shadcn-dashboard-controls.json",
  );
  const retiredProofPath = path.join(
    root,
    "examples",
    "www-template",
    "shadcn-ui-proof.tsx",
  );
  const receiptText = read(receiptPath);
  const receipt = JSON.parse(receiptText);
  const runtimePage = read(
    path.join(root, "tools", "launch", "runtime-template", "pages", "index.html"),
  );
  const runtimeJs = read(
    path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts"),
  );
  const runtimeCss = read(
    path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.css"),
  );
  const materializer = read(path.join(root, "tools", "launch", "materialize-www-template.ts"));
  const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
  const newCommand = read(path.join(root, "dx-www", "src", "cli", "new_command.rs"));
  const todo = read(path.join(root, "TODO.md"));
  const changelog = read(path.join(root, "CHANGELOG.md"));
  const dx = read(path.join(root, "DX.md"));

  assert.match(upstreamButton, /data-slot="button"/);
  assert.match(upstreamButton, /data-variant=\{variant\}/);
  assert.match(upstreamButton, /data-size=\{size\}/);
  assert.match(upstreamItem, /data-slot="item"/);
  assert.match(upstreamItem, /ItemActions/);
  assert.match(upstreamLabel, /export \{ Label \}/);
  assert.match(upstreamLabel, /data-slot="label"/);
  assert.match(upstreamField, /FieldGroup/);
  assert.match(upstreamField, /FieldLabel/);
  assert.match(upstreamField, /FieldDescription/);
  assert.match(upstreamCard, /CardHeader/);
  assert.match(upstreamCard, /CardContent/);

  assert.match(component, /export function LaunchShadcnDashboardControls/);
  assert.match(contract, /export const shadcnLaunchDashboardMetadata/);
  assert.match(contract, /export function createShadcnLaunchDashboardReceipt/);
  assert.match(contract, /export const shadcnDashboardQueueOptions/);
  assert.match(contract, /export const shadcnDashboardDensityOptions/);
  assert.match(contract, /sourceMirror: "G:\/WWW\/inspirations\/shadcn-ui"/);
  assert.match(contract, /"ui\/label"/);
  assert.match(contract, /"components\/ui\/label\.tsx"/);
  assert.match(contract, /"FieldGroup"/);
  assert.match(contract, /"ItemActions"/);
  assert.match(component, /from "\.\/shadcn-dashboard-controls-contract"/);
  assert.match(component, /createShadcnLaunchDashboardReceipt/);
  assert.match(contract, /"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-shadcn-dashboard-controls\.json"/);
  assert.match(component, /data-dx-component="shadcn-dashboard-controls"/);
  assert.match(component, /data-dx-dashboard-workflow="operator-controls"/);
  assert.match(component, /data-dx-package="state\/zustand,shadcn\/ui\/button"/);
  assert.match(
    component,
    /data-dx-package="shadcn\/ui\/button"[\s\S]*data-dx-shadcn-dashboard-action="set-density"/,
  );
  assert.match(
    component,
    /data-dx-package="shadcn\/ui\/button"[\s\S]*data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/,
  );
  assert.match(component, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(component, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(component, /data-dx-shadcn-dashboard-action="set-density"/);
  assert.match(component, /data-dx-shadcn-dashboard-action="select-queue"/);
  assert.match(component, /data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"/);
  assert.match(component, /data-dx-shadcn-dashboard-action="focus-target-card"/);
  assert.match(component, /function focusSelectedDashboardTarget/);
  assert.match(component, /data-dx-shadcn-dashboard-focus-target=\{activeQueue\.controlsId\}/);
  assert.match(component, /closest<HTMLElement>\('\[data-dx-dashboard-card\]'\)/);
  assert.match(component, /querySelectorAll\("\[data-dx-shadcn-dashboard-target-focused\]"\)/);
  assert.match(component, /setAttribute\("data-dx-shadcn-dashboard-target-focused", "false"\)/);
  assert.match(component, /data-dx-shadcn-dashboard-target-focused/);
  assert.match(component, /function handleQueueKeyDown/);
  assert.match(component, /case "ArrowRight"/);
  assert.match(component, /case "ArrowLeft"/);
  assert.match(component, /case "Home"/);
  assert.match(component, /case "End"/);
  assert.match(component, /aria-current=\{queue === option\.id \? "true" : undefined\}/);
  assert.match(component, /data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/);
  assert.match(component, /data-dx-shadcn-dashboard-receipt/);
  assert.match(component, /aria-pressed=\{density === option\.id\}/);
  assert.match(component, /aria-pressed=\{queue === option\.id\}/);
  assert.match(component, /aria-controls="shadcn-dashboard-receipt-preview"/);
  assert.match(component, /aria-controls=\{option\.controlsId\}/);
  assert.match(component, /data-dx-shadcn-dashboard-controls-target=\{activeQueue\.controlsId\}/);
  assert.match(component, /aria-live="polite"/);
  assert.match(component, /data-slot="button"/);
  assert.match(component, /data-slot="badge"/);
  assert.match(component, /data-slot="card"/);
  assert.match(component, /data-slot="field"/);
  assert.match(component, /data-slot="input"/);
  assert.match(component, /data-slot="textarea"/);
  assert.match(component, /data-slot="item"/);
  assert.match(component, /data-slot="separator"/);
  assert.match(component, /<dx-icon name="pack:settings"/);
  assert.doesNotMatch(component, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);
  assert.doesNotMatch(component, /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray|emerald|cyan|amber)-/);

  assert.match(shell, /import \{ LaunchShadcnDashboardControls \}/);
  assert.match(shell, /<LaunchShadcnDashboardControls \/>/);
  assert.match(shell, /data-dx-section="launch-dashboard-controls"/);
  assert.match(shell, /data-dx-editable-section="launch-dashboard-controls"/);
  assert.doesNotMatch(shell, /LaunchShadcnUiProof/);
  assert.doesNotMatch(shell, /data-dx-component="shadcn-ui-proof-card"/);
  assert.ok(
    !fs.existsSync(retiredProofPath),
    "old shadcn-ui-proof.tsx source must stay out of the launch template",
  );

  assert.match(routeContract, /"components\/template-app\/shadcn-dashboard-controls\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/shadcn-dashboard-controls-contract\.tsx"/);
  assert.doesNotMatch(routeContract, /shadcn-ui-proof/);
  assert.match(studioContract, /id: "shadcn-dashboard-controls"/);
  assert.match(studioContract, /selector: '\[data-dx-component="shadcn-dashboard-controls"\]'/);
  assert.match(studioContract, /"shadcn\/ui\/button"/);
  assert.match(studioContract, /"shadcn\/ui\/item"/);

  assert.match(catalog, /packageId: "shadcn\/ui\/button"/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/shadcn-ui"/);
  assert.match(catalog, /"components\/template-app\/shadcn-dashboard-controls\.tsx"/);
  assert.match(catalog, /"components\/template-app\/shadcn-dashboard-controls-contract\.tsx"/);
  assert.match(catalog, /"examples\/template\/shadcn-dashboard-controls\.tsx"/);
  assert.match(catalog, /"examples\/template\/shadcn-dashboard-controls-contract\.tsx"/);
  assert.match(catalog, /"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-shadcn-dashboard-controls\.json"/);

  assert.equal(receipt.package_id, "shadcn/ui/button");
  assert.equal(receipt.route, "/");
  assert.equal(receipt.component.selector, '[data-dx-component="shadcn-dashboard-controls"]');
  assert.equal(
    receipt.runtime_component.selector,
    '[data-dx-component="shadcn-dashboard-controls-runtime"]',
  );
  assert.deepEqual(receipt.interactions, [
    "set-density",
    "select-queue",
    "keyboard-navigate-queue",
    "focus-target-card",
    "preview-dashboard-receipt",
  ]);
  assert.deepEqual(receipt.controlled_targets, [
    "mission-session-status",
    "mission-payment-status",
    "mission-database-status",
    "mission-dashboard-status",
  ]);
  assert.equal(receipt.node_modules_required, false);
  assert.ok(receipt.exported_files.includes("components/ui/label.tsx"));
  assert.ok(receipt.upstream_public_apis.includes("Label"));
  assert.ok(receipt.upstream_public_apis.includes("FieldGroup"));
  assert.ok(receipt.upstream_public_apis.includes("ItemActions"));
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-package="shadcn/ui/button" on visible button interactions',
    ),
    "receipt should record exact package markers on interactive shadcn buttons",
  );
  assert.deepEqual(receipt.retired_source_files, [
    "examples/template/shadcn-ui-proof.tsx",
  ]);

  assert.match(runtimePage, /data-dx-component="shadcn-dashboard-controls-runtime"/);
  assert.match(runtimePage, /data-dx-dashboard-workflow="operator-controls"/);
  assert.match(
    runtimePage,
    /data-dx-package="shadcn\/ui\/button"[\s\S]*data-dx-shadcn-dashboard-action="set-density"/,
  );
  assert.match(
    runtimePage,
    /data-dx-package="shadcn\/ui\/button"[\s\S]*data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/,
  );
  assert.doesNotMatch(runtimePage, /data-dx-component="shadcn-ui-runtime-proof"/);
  assert.doesNotMatch(runtimePage, /data-dx-shadcn-proof="runtime-source-primitives"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-action="set-density"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-action="select-queue"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-action="focus-target-card"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-focus-target="mission-payment-status"/);
  assert.match(runtimePage, /aria-current="true"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-receipt="idle"/);
  assert.match(runtimePage, /aria-pressed="true"/);
  assert.match(runtimePage, /aria-controls="mission-payment-status"/);
  assert.match(runtimePage, /data-dx-shadcn-dashboard-controls-target="mission-payment-status"/);
  assert.match(runtimePage, /aria-live="polite"/);
  assert.match(runtimePage, /data-dx-dashboard-card="controls"/);
  assert.match(runtimePage, /<dx-icon name="pack:settings"/);

  assert.match(runtimeJs, /function bindShadcnDashboardControls\(\)/);
  assert.doesNotMatch(runtimeJs, /\$\(\'\[data-dx-component="shadcn-ui-runtime-proof"\]\'\)/);
  assert.doesNotMatch(runtimeJs, /bindShadcnProof\(\)/);
  assert.match(runtimeJs, /data-dx-shadcn-dashboard-action='set-density'/);
  assert.match(runtimeJs, /data-dx-shadcn-dashboard-action='preview-dashboard-receipt'/);
  assert.match(runtimeJs, /function focusDashboardQueueButton/);
  assert.match(runtimeJs, /function focusShadcnDashboardTarget/);
  assert.match(runtimeJs, /data-dx-shadcn-dashboard-action='focus-target-card'/);
  assert.match(runtimeJs, /data-dx-shadcn-dashboard-target-focused/);
  assert.match(runtimeJs, /focus\(\{ preventScroll: true \}\)/);
  assert.match(runtimeJs, /case "ArrowRight"/);
  assert.match(runtimeJs, /case "ArrowLeft"/);
  assert.match(runtimeJs, /case "Home"/);
  assert.match(runtimeJs, /case "End"/);
  assert.match(runtimeJs, /function bindShadcnDashboardControls\(\)[\s\S]*setAttribute\("aria-pressed"/);
  assert.match(runtimeJs, /setAttribute\("aria-current"/);
  assert.match(runtimeJs, /setAttribute\("tabindex"/);
  assert.match(runtimeJs, /dxShadcnDashboardControlsTarget/);
  assert.match(runtimeJs, /dxShadcnDashboardReceipt/);
  assert.match(runtimeJs, /bindShadcnDashboardControls\(\)/);
  assert.match(runtimeCss, /\[data-dx-shadcn-dashboard-selected="true"\]/);
  assert.match(runtimeCss, /color-mix\(in srgb, var\(--dx-green\)/);

  assert.match(materializer, /"shadcn\/ui\/button"/);
  assert.match(materializer, /launch-runtime-dashboard/);
  assert.match(newCommand, /NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_TSX/);
  assert.match(newCommand, /NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_CONTRACT_TSX/);
  assert.match(newCommand, /components\/template-app\/shadcn-dashboard-controls\.tsx/);
  assert.match(newCommand, /components\/template-app\/shadcn-dashboard-controls-contract\.tsx/);
  assert.doesNotMatch(cli, /NEXT_FAMILIAR_SHADCN_UI_PROOF_TSX|shadcn-ui-proof/);
  assert.match(component, /LaunchShadcnDashboardControls/);
  assert.match(component, /data-dx-component="shadcn-dashboard-controls"/);
  assert.match(receiptText, /data-dx-shadcn-dashboard-keyboard=\\"arrow-roving-focus\\"/);
  assert.match(receiptText, /data-dx-shadcn-dashboard-action=\\"focus-target-card\\"/);
  assert.equal(
    path.relative(root, receiptPath).replace(/\\/g, "/"),
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
  );
  assert.match(todo, /shadcn\/ui \/launch dashboard controls/);
  assert.match(todo, /focus-target-card/);
  assert.match(changelog, /shadcn\/ui \/launch dashboard controls/);
  assert.match(changelog, /focus-target-card/);
  assert.match(dx, /shadcn\/ui \/launch dashboard controls/);
  assert.match(dx, /focus-target-card/);
});
