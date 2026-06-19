const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const shadcnMirror = path.resolve(root, "..", "..", "WWW", "inspirations", "shadcn-ui");
const radixMirror = path.resolve(root, "..", "..", "WWW", "inspirations", "radix-primitives");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(mirror, relativePath) {
  return fs.readFileSync(path.join(mirror, relativePath), "utf8");
}

function catalogEntry(catalog, packageId) {
  const escaped = packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = catalog.match(
    new RegExp(`\\{[\\s\\S]*?packageId: "${escaped}"[\\s\\S]*?\\n  \\},`),
  );
  assert.ok(match, `expected catalog entry for ${packageId}`);
  return match[0];
}

test("UI Components lane uses official package naming with shadcn and Radix provenance", () => {
  const shadcnPackage = JSON.parse(readMirror(shadcnMirror, "package.json"));
  const radixSlot = JSON.parse(
    readMirror(radixMirror, "packages/react/slot/package.json"),
  );
  const radixLabel = JSON.parse(
    readMirror(radixMirror, "packages/react/label/package.json"),
  );
  const radixSeparator = JSON.parse(
    readMirror(radixMirror, "packages/react/separator/package.json"),
  );
  const slotSource = readMirror(radixMirror, "packages/react/slot/src/slot.tsx");
  const labelSource = readMirror(radixMirror, "packages/react/label/src/label.tsx");
  const separatorSource = readMirror(
    radixMirror,
    "packages/react/separator/src/separator.tsx",
  );

  assert.equal(shadcnPackage.name, "ui");
  assert.equal(shadcnPackage.version, "0.0.1");
  assert.equal(radixSlot.name, "@radix-ui/react-slot");
  assert.equal(radixSlot.version, "1.2.4");
  assert.equal(radixLabel.name, "@radix-ui/react-label");
  assert.equal(radixLabel.version, "2.1.8");
  assert.equal(radixSeparator.name, "@radix-ui/react-separator");
  assert.equal(radixSeparator.version, "1.1.8");
  assert.match(slotSource, /export function createSlot/);
  assert.match(labelSource, /const Label = React\.forwardRef/);
  assert.match(separatorSource, /const Separator = React\.forwardRef/);

  const docs = read("docs/packages/ui-components.md");
  const catalog = read("examples/template/package-catalog.ts");
  const dashboardModel = read("examples/dashboard/src/lib/shadcnDashboardControls.ts");
  const dashboardComponent = read(
    "examples/dashboard/src/components/ShadcnDashboardControls.tsx",
  );
  const launchContract = read(
    "examples/template/shadcn-dashboard-controls-contract.tsx",
  );
  const receipt = JSON.parse(
    read(
      "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
    ),
  );
  const framework = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  for (const packageId of [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
  ]) {
    const entry = catalogEntry(catalog, packageId);
    assert.match(entry, /officialName: "UI Components"/);
    assert.match(entry, /docsPath: "docs\/packages\/ui-components\.md"/);
    assert.match(entry, /upstreamPackage: "shadcn-ui"/);
    assert.match(entry, /upstreamVersion: "0\.0\.1"/);
    assert.match(entry, /honestyLabel: "SOURCE-ONLY"/);
  }

  const buttonEntry = catalogEntry(catalog, "shadcn/ui/button");
  assert.match(buttonEntry, /upstreamPackages: \[/);
  assert.match(buttonEntry, /"@radix-ui\/react-slot@1\.2\.4"/);
  assert.match(buttonEntry, /"@radix-ui\/react-label@2\.1\.8"/);
  assert.match(buttonEntry, /"@radix-ui\/react-separator@1\.1\.8"/);
  assert.match(buttonEntry, /sourceMirrors: \[/);
  assert.match(buttonEntry, /"G:\/WWW\/inspirations\/shadcn-ui"/);
  assert.match(buttonEntry, /"G:\/WWW\/inspirations\/radix-primitives"/);
  assert.match(buttonEntry, /inspectedSourceFiles: \[/);
  assert.match(buttonEntry, /apps\/v4\/registry\/new-york-v4\/ui\/button\.tsx/);
  assert.match(buttonEntry, /packages\/react\/slot\/src\/slot\.tsx/);
  assert.match(buttonEntry, /packages\/react\/label\/src\/label\.tsx/);
  assert.match(buttonEntry, /packages\/react\/separator\/src\/separator\.tsx/);
  assert.match(buttonEntry, /selectedSurfaces: \[/);
  assert.match(buttonEntry, /"button"/);
  assert.match(buttonEntry, /"field"/);
  assert.match(buttonEntry, /"item"/);
  assert.match(buttonEntry, /dxCheckVisibility: \{/);
  assert.match(buttonEntry, /statuses: \[\s*"present",\s*"stale",\s*"missing receipt",\s*"blocked",\s*"unsupported surface",\s*\]/);
  assert.match(buttonEntry, /monitoredSurfaces: \[/);
  assert.match(buttonEntry, /"components\/ui\/button\.tsx"/);
  assert.match(buttonEntry, /"components\/template-app\/shadcn-dashboard-controls\.tsx"/);

  assert.match(dashboardModel, /officialName: 'UI Components'/);
  assert.match(dashboardModel, /upstreamPackage: 'shadcn-ui'/);
  assert.match(dashboardModel, /upstreamVersion: '0\.0\.1'/);
  assert.match(dashboardModel, /upstreamPackages: \[/);
  assert.match(dashboardModel, /docsPath: 'docs\/packages\/ui-components\.md'/);
  assert.match(dashboardModel, /dxCheckVisibility: \{/);
  assert.match(dashboardComponent, /<h2 data-slot="card-title">UI Components dashboard controls<\/h2>/);
  assert.doesNotMatch(dashboardComponent, /<h2 data-slot="card-title">shadcn\/ui/);

  assert.match(launchContract, /officialName: "UI Components"/);
  assert.match(launchContract, /upstreamPackage: "shadcn-ui"/);
  assert.match(launchContract, /upstreamVersion: "0\.0\.1"/);
  assert.match(launchContract, /docsPath: "docs\/packages\/ui-components\.md"/);
  assert.match(launchContract, /dxCheckVisibility: \{/);

  assert.equal(receipt.official_package_name, "UI Components");
  assert.equal(receipt.package_id, "shadcn/ui/button");
  assert.equal(receipt.upstream_package, "shadcn-ui");
  assert.equal(receipt.upstream_version, "0.0.1");
  assert.deepEqual(receipt.source_mirrors, [
    "G:/WWW/inspirations/shadcn-ui",
    "G:/WWW/inspirations/radix-primitives",
  ]);
  assert.ok(
    receipt.inspected_source_files.includes(
      "G:/WWW/inspirations/radix-primitives/packages/react/slot/src/slot.tsx",
    ),
  );
  assert.deepEqual(receipt.dx_check_visibility.statuses, [
    "present",
    "stale",
    "missing receipt",
    "blocked",
    "unsupported surface",
  ]);
  assert.equal(receipt.honesty_label, "SOURCE-ONLY");
  assert.ok(receipt.runtime_limitations.includes("governed browser proof deferred"));
  assert.equal(receipt.no_runtime_execution, true);

  assert.match(docs, /^# UI Components/m);
  assert.match(docs, /official DX package name: \*\*UI Components\*\*/);
  assert.match(docs, /upstream_package: `shadcn-ui`/);
  assert.match(docs, /source_mirror: `G:\/WWW\/inspirations\/shadcn-ui`/);
  assert.match(docs, /source_mirror: `G:\/WWW\/inspirations\/radix-primitives`/);
  assert.match(docs, /selected surfaces: `button`, `badge`, `card`, `label`, `separator`, `field`, `item`, `input`, `textarea`/);
  assert.match(docs, /dx-check visibility: `present`, `stale`, `missing receipt`, `blocked`, `unsupported surface`/);
  assert.match(docs, /dx-style compatibility/);
  assert.match(docs, /Zed\/DX Studio markers/);
  assert.match(docs, /Honesty label: `SOURCE-ONLY`/);
  assert.match(docs, /Runtime proof is deferred/);
  assert.doesNotMatch(docs, /official DX package name: \*\*shadcn\/ui/);

  assert.match(framework, /Lane 20 uses the official front-facing package name `UI Components`/);
  assert.match(dx, /UI Components lane update, 2026-05-22/);
  assert.match(todo, /UI Components lane next action/);
  assert.match(changelog, /Documented the UI Components lane/);
});

test("UI Components Forge registry metadata keeps official naming separate from upstream provenance", () => {
  const registry = read("core/src/ecosystem/forge_registry.rs");

  for (const description of [
    "Source-owned UI Components button surface based on shadcn-ui v4 and Radix Slot, with local class helpers.",
    "Source-owned UI Components badge surface based on shadcn-ui v4 and Radix Slot, with local class helpers.",
    "Source-owned UI Components card surface based on shadcn-ui v4, with local class helpers.",
    "Source-owned UI Components label surface based on shadcn-ui v4 and the Radix Label primitive, with local class helpers.",
    "Source-owned UI Components separator surface based on shadcn-ui v4 and the Radix Separator primitive, with local class helpers.",
    "Source-owned UI Components field primitives based on shadcn-ui v4, Radix Label, Radix Separator, and local class helpers.",
    "Source-owned UI Components item primitives based on shadcn-ui v4, Radix Slot, Radix Separator, and local class helpers.",
    "Source-owned UI Components input surface based on the shadcn-ui v4 registry shape.",
    "Source-owned UI Components textarea surface based on the shadcn-ui v4 registry shape.",
  ]) {
    assert.ok(
      registry.includes(description),
      `expected registry description: ${description}`,
    );
  }

  assert.match(
    registry,
    /"shadcn\/ui\/textarea" => "shadcn-ui",/,
    "UI Components registry upstream_name should keep shadcn-ui as provenance, not a fake DX package",
  );
  assert.doesNotMatch(registry, /@dx\/forge\/shadcn-ui/);
  assert.doesNotMatch(registry, /Source-owned shadcn-style/);

  for (const surface of [
    "Button",
    "Badge",
    "Card",
    "Label",
    "Separator",
    "Field",
    "Item",
    "Input",
    "Textarea",
  ]) {
    assert.match(
      registry,
      new RegExp(`official UI Components ${surface} surface`),
      `expected provenance note for ${surface}`,
    );
  }

  const docs = read("docs/packages/ui-components.md");
  assert.match(docs, /Forge registry metadata/);
  assert.match(
    docs,
    /Registry descriptions and generated package metadata present `UI Components` as the official package lane/,
  );
});
