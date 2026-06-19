const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "shadcn-ui");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("shadcn/ui exposes a real dashboard control workflow", () => {
  const upstreamButton = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "button.tsx"),
  );
  const upstreamBadge = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "badge.tsx"),
  );
  const upstreamCard = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "card.tsx"),
  );
  const upstreamField = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "field.tsx"),
  );
  const upstreamLabel = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "label.tsx"),
  );
  const upstreamInput = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "input.tsx"),
  );
  const upstreamTextarea = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "textarea.tsx"),
  );
  const upstreamSeparator = read(
    path.join(mirror, "apps", "v4", "registry", "new-york-v4", "ui", "separator.tsx"),
  );
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const dashboard = read(
    path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"),
  );
  const workflow = read(
    path.join(
      root,
      "examples",
      "dashboard",
      "src",
      "components",
      "ShadcnDashboardControls.tsx",
    ),
  );
  const workflowModel = read(
    path.join(
      root,
      "examples",
      "dashboard",
      "src",
      "lib",
      "shadcnDashboardControls.ts",
    ),
  );
  const readme = read(path.join(root, "examples", "dashboard", "README.md"));
  const packageDocs = read(path.join(root, "docs", "forge-shadcn-migration.md"));
  const todo = read(path.join(root, "TODO.md"));
  const changelog = read(path.join(root, "CHANGELOG.md"));
  const dx = read(path.join(root, "DX.md"));

  assert.match(upstreamButton, /export \{ Button, buttonVariants \}/);
  assert.match(upstreamButton, /data-slot="button"/);
  assert.match(upstreamButton, /data-variant=\{variant\}/);
  assert.match(upstreamButton, /data-size=\{size\}/);
  assert.match(upstreamBadge, /export \{ Badge, badgeVariants \}/);
  assert.match(upstreamCard, /CardAction/);
  assert.match(upstreamField, /FieldSet/);
  assert.match(upstreamField, /FieldGroup/);
  assert.match(upstreamField, /FieldLabel/);
  assert.match(upstreamLabel, /export \{ Label \}/);
  assert.match(upstreamLabel, /data-slot="label"/);
  assert.match(upstreamInput, /export \{ Input \}/);
  assert.match(upstreamInput, /data-slot="input"/);
  assert.match(upstreamTextarea, /export \{ Textarea \}/);
  assert.match(upstreamTextarea, /data-slot="textarea"/);
  assert.match(upstreamSeparator, /export \{ Separator \}/);
  assert.match(upstreamSeparator, /data-slot="separator"/);

  assert.match(registry, /"shadcn\/ui\/button"/);
  assert.match(registry, /"shadcn\/ui\/badge"/);
  assert.match(registry, /"shadcn\/ui\/card"/);
  assert.match(registry, /"shadcn\/ui\/label"/);
  assert.match(registry, /"shadcn\/ui\/separator"/);
  assert.match(registry, /"shadcn\/ui\/field"/);
  assert.match(registry, /"shadcn\/ui\/item"/);

  assert.match(dashboard, /import \{ ShadcnDashboardControls \}/);
  assert.match(dashboard, /<ShadcnDashboardControls \/>/);

  assert.match(workflow, /export const shadcnDashboardPackageMetadata/);
  assert.match(workflowModel, /packageId: 'shadcn\/ui\/button'/);
  assert.match(workflowModel, /aliases: \[/);
  assert.match(workflowModel, /sourceMirror: 'G:\/WWW\/inspirations\/shadcn-ui'/);
  assert.match(workflowModel, /provenance:/);
  assert.match(workflowModel, /exportedFiles: \[/);
  assert.match(workflowModel, /'Label'/);
  assert.match(workflowModel, /'FieldLabel'/);
  assert.match(workflowModel, /'Input'/);
  assert.match(workflowModel, /'Textarea'/);
  assert.match(workflowModel, /'ItemActions'/);
  assert.match(workflowModel, /'js\/ui\/label\.tsx'/);
  assert.match(workflowModel, /requiredEnv: \[\]/);
  assert.match(workflowModel, /appOwnedBoundaries: \[/);
  assert.match(workflowModel, /receiptPaths: \[/);
  assert.match(workflowModel, /createShadcnDashboardReceipt/);
  assert.match(workflow, /data-dx-package="shadcn\/ui\/button"/);
  assert.match(workflow, /data-dx-component="dashboard-shadcn-controls"/);
  assert.match(workflow, /data-shadcn-dashboard-workflow="settings-review"/);
  assert.match(workflow, /data-shadcn-dashboard-action="select-density"/);
  assert.match(workflow, /data-shadcn-dashboard-action="toggle-notification"/);
  assert.match(workflow, /data-shadcn-dashboard-action="preview-save-receipt"/);
  assert.match(workflow, /data-shadcn-dashboard-receipt/);
  assert.match(workflow, /data-slot="button"/);
  assert.match(workflow, /data-slot="badge"/);
  assert.match(workflow, /data-slot="card"/);
  assert.match(workflow, /data-slot="field"/);
  assert.match(workflow, /data-slot="field-label"/);
  assert.match(workflow, /data-slot="input"/);
  assert.match(workflow, /data-slot="textarea"/);
  assert.match(workflow, /data-slot="item"/);
  assert.match(workflow, /data-slot="separator"/);
  assert.match(workflow, /data-shadcn-dashboard-package=\{item\.id\}/);
  assert.match(workflow, /data-variant=/);
  assert.match(workflow, /data-size=/);
  assert.match(workflow, /<dx-icon name="pack:settings"/);
  assert.match(workflow, /<dx-icon name="pack:search"/);
  assert.match(workflow, /<dx-icon name="pack:check"/);
  assert.match(workflow, /useState/);
  assert.doesNotMatch(workflow, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);
  assert.doesNotMatch(workflow, /\b(?:bg|text|border)-(?:neutral|slate|zinc|stone|gray)-/);
  assert.doesNotMatch(workflow, /lucide|heroicons|phosphor/i);

  assert.match(readme, /UI Components dashboard controls/);
  assert.match(readme, /official \*\*UI Components\*\* package/);
  assert.match(readme, /shadcn\/ui\/label/);
  assert.match(packageDocs, /ShadcnDashboardControls/);
  assert.match(packageDocs, /safe local settings receipt/);
  assert.match(todo, /shadcn\/ui dashboard controls/);
  assert.match(changelog, /shadcn\/ui dashboard controls/);
  assert.match(dx, /shadcn\/ui dashboard controls/);
  assert.ok(
    !fs.existsSync(path.join(root, "examples", "dashboard", "node_modules")),
    "dashboard starter must not add a local node_modules workflow",
  );
});
