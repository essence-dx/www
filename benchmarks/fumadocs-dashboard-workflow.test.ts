const assert = require("assert");
const crypto = require("crypto");
const { execFileSync } = require("node:child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "fumadocs");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function sha256(filePath) {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

const dashboardComponentPath = path.join(
  root,
  "examples",
  "dashboard",
  "src",
  "components",
  "FumadocsDocsWorkflow.tsx",
);
const dashboardLibPath = path.join(
  root,
  "examples",
  "dashboard",
  "src",
  "lib",
  "fumadocsDocsWorkflow.ts",
);

const dashboardComponent = read(dashboardComponentPath);
const dashboardLib = read(dashboardLibPath);
const dashboardPage = read(
  path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"),
);
const dashboardReadme = read(path.join(root, "examples", "dashboard", "README.md"));
const forge = read(path.join(root, "core", "src", "ecosystem", "forge_fumadocs.rs"));
const packageCatalog = read(
  path.join(root, "examples", "template", "package-catalog.ts"),
);
const launchShell = read(
  path.join(root, "examples", "template", "template-shell.tsx"),
);
const docsStatus = read(
  path.join(root, "examples", "template", "docs-status.tsx"),
);
const runtimeLaunchPage = read(
  path.join(root, "tools", "launch", "runtime-template", "pages", "index.html"),
);
const runtimeLaunchCss = read(
  path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.css"),
);
const runtimeLaunchJs = read(
  path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts"),
);
const templateRouteContract = read(
  path.join(root, "examples", "template", "template-route-contract.ts"),
);
const templateSurfaceRegistry = read(
  path.join(root, "examples", "template", "template-surface-registry.ts"),
);
const dxStudioEditContract = read(
  path.join(root, "examples", "template", "dx-studio-edit-contract.ts"),
);
const forgePackageStatusReadModel = read(
  path.join(root, "examples", "template", "forge-package-status-read-model.ts"),
);
const forgePackageStatusSource = read(
  path.join(root, "examples", "template", "forge-package-status.ts"),
);
const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
const studioManifest = read(
  path.join(root, "dx-www", "src", "cli", "studio_manifest.rs"),
);
const packageDoc = read(
  path.join(root, "docs", "packages", "content-fumadocs-next.md"),
);
const dashboardReceiptPath = path.join(
  root,
  "examples",
  "www-template",
  ".dx",
  "forge",
  "receipts",
  "2026-05-22-content-fumadocs-dashboard-workflow.json",
);

test("upstream fumadocs source proves the dashboard workflow uses real public APIs", () => {
  const packageJson = read(path.join(mirror, "packages", "core", "package.json"));
  const breadcrumb = read(path.join(mirror, "packages", "core", "src", "breadcrumb.tsx"));
  const pageTree = read(path.join(mirror, "packages", "core", "src", "page-tree", "utils.ts"));
  const llmsSource = read(path.join(mirror, "packages", "core", "src", "source", "llms.ts"));

  assert.match(packageJson, /"\.\/breadcrumb"/);
  assert.match(packageJson, /"\.\/page-tree"/);
  assert.match(packageJson, /"\.\/source\/llms"/);
  assert.match(breadcrumb, /export function getBreadcrumbItems/);
  assert.match(pageTree, /export function flattenTree/);
  assert.match(pageTree, /export function findNeighbour/);
  assert.match(pageTree, /export function getPageTreePeers/);
  assert.match(llmsSource, /export function llms/);
});

test("dashboard starter visibly consumes the fumadocs package in a workflow", () => {
  assert.match(dashboardPage, /import \{ FumadocsDocsWorkflow \}/);
  assert.match(dashboardPage, /<FumadocsDocsWorkflow \/>/);
  assert.match(dashboardComponent, /data-dx-package="content\/fumadocs-next"/);
  assert.match(
    dashboardComponent,
    /data-dx-component="dashboard-fumadocs-docs-workflow"/,
  );
  assert.match(
    dashboardComponent,
    /data-dx-fumadocs-dashboard-workflow="docs-ops"/,
  );
  assert.match(dashboardComponent, /<dx-icon name="pack:fumadocs"/);
  assert.match(dashboardComponent, /aria-pressed=\{selected \? 'true' : 'false'\}/);
  assert.match(dashboardComponent, /role="status"/);
  assert.match(dashboardComponent, /aria-live="polite"/);
  assert.match(
    dashboardComponent,
    /data-dx-fumadocs-interaction="page-tree-selector"/,
  );
  assert.match(
    dashboardComponent,
    /data-dx-fumadocs-action="safe-local-route-preview"/,
  );
  assert.match(dashboardComponent, /data-dx-fumadocs-local-response=/);
  assert.match(dashboardLib, /content\/fumadocs-next/);
  assert.match(dashboardLib, /getBreadcrumbItems from fumadocs-core\/breadcrumb/);
  assert.match(dashboardLib, /flattenTree from fumadocs-core\/page-tree/);
  assert.match(dashboardLib, /llms from fumadocs-core\/source/);
  assert.doesNotMatch(dashboardComponent, /#[0-9a-fA-F]{3,8}/);
});

test("forge fumadocs metadata exposes the dashboard workflow slice", () => {
  assert.match(forge, /js\/lib\/fumadocs\/dashboard-workflow\.ts/);
  assert.match(forge, /js\/components\/dashboard\/fumadocs-docs-workflow\.tsx/);
  assert.match(forge, /FUMADOCS_DASHBOARD_WORKFLOW_TS/);
  assert.match(forge, /FUMADOCS_DASHBOARD_WORKFLOW_TSX/);
  assert.match(forge, /aliases: \["fumadocs", "fumadocs-next", "docs"\]/);
  assert.match(forge, /sourceMirror: "G:\/WWW\/inspirations\/fumadocs"/);
  assert.match(forge, /aria-pressed=\{selected\}/);
  assert.match(forge, /aria-live="polite"/);
  assert.match(forge, /provenance:/);
  assert.match(forge, /receiptPaths:/);
  assert.match(forge, /docs\/packages\/content-fumadocs-next\.md/);
  assert.match(forge, /dashboardWorkflowApiFile: "lib\/fumadocs\/dashboard-workflow\.ts"/);
  assert.match(forge, /dashboardWorkflowFile: "components\/dashboard\/fumadocs-docs-workflow\.tsx"/);
  assert.match(forge, /createFumadocsNavigationReceipt/);
  assert.match(forge, /benchmarks\/fumadocs-dashboard-workflow\.test\.ts/);
  assert.match(forge, /FumadocsDocsWorkflow/);
  assert.match(
    forge,
    /from "@\/lib\/fumadocs\/dashboard-workflow"/,
  );
});

test("launch package catalog carries professional fumadocs metadata", () => {
  assert.match(packageCatalog, /packageId: "content\/fumadocs-next"/);
  assert.match(packageCatalog, /aliases: \["fumadocs", "fumadocs-next", "docs"\]/);
  assert.match(packageCatalog, /sourceMirror: "G:\/WWW\/inspirations\/fumadocs"/);
  assert.match(packageCatalog, /dxIcon: "pack:fumadocs"/);
  assert.match(packageCatalog, /dashboardUsage: \{/);
  assert.match(packageCatalog, /route: "\/"/);
  assert.match(packageCatalog, /component: "launch-fumadocs-docs-workflow"/);
  assert.match(packageCatalog, /sourceFile: "examples\/template\/docs-status\.tsx"/);
  assert.match(packageCatalog, /data-dx-product-surface="dashboard-help-content"/);
  assert.match(packageCatalog, /data-dx-fumadocs-interaction="page-tree-selector"/);
  assert.match(packageCatalog, /data-dx-fumadocs-page-option/);
  assert.match(packageCatalog, /data-dx-fumadocs-rendered-markdown/);
  assert.match(packageCatalog, /data-dx-fumadocs-changelog/);
  assert.match(packageCatalog, /data-dx-fumadocs-rendered-route/);
  assert.match(packageCatalog, /data-dx-fumadocs-selected-page/);
  assert.match(packageCatalog, /data-dx-fumadocs-toc-count/);
  assert.match(packageCatalog, /data-dx-fumadocs-receipt-route/);
  assert.match(packageCatalog, /data-dx-fumadocs-missing-config/);
  assert.match(packageCatalog, /data-dx-docs-openapi-code-usage/);
  assert.match(packageCatalog, /data-dx-docs-openapi-proxy/);
  assert.match(packageCatalog, /docs\/packages\/content-fumadocs-next\.md/);
  assert.match(
    packageCatalog,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json/,
  );
  assert.match(packageCatalog, /lib\/fumadocs\/dashboard-workflow\.ts/);
  assert.match(packageCatalog, /lib\/fumadocs\/openapi-code-usage\.ts/);
  assert.match(packageCatalog, /components\/dashboard\/fumadocs-docs-workflow\.tsx/);
  [
    /lib\/fumadocs\/layout\.tsx/,
    /components\/mdx\.tsx/,
    /components\/api-page\.tsx/,
    /components\/api-page\.client\.tsx/,
    /app\/docs\/layout\.tsx/,
    /app\/docs\/\[\[...slug\]\]\/page\.tsx/,
    /app\/llms\.txt\/route\.ts/,
    /app\/llms-full\.txt\/route\.ts/,
    /app\/llms\.mdx\/docs\/\[\[...slug\]\]\/route\.ts/,
    /app\/api\/search\/route\.ts/,
    /app\/api\/search-static\/route\.ts/,
    /app\/api\/openapi\/proxy\/route\.ts/,
    /content\/docs\/meta\.json/,
    /content\/docs\/index\.mdx/,
    /openapi\/dx-launch\.yaml/,
    /lib\/fumadocs\/README\.md/,
  ].forEach((filePattern) => assert.match(packageCatalog, filePattern));
  assert.match(packageCatalog, /Documentation System/);
  assert.match(packageCatalog, /createFumadocsNavigationReceipt/);
  assert.match(packageCatalog, /FumadocsDocsWorkflow/);
  [
    /dx config boundary/,
    /dxFumadocsIconPlugin/,
    /statusBadgesPlugin/,
    /slugsFromData/,
    /DocsLayout/,
    /RootProvider/,
    /DocsPage/,
    /createRelativeLink/,
    /getBreadcrumbItems/,
    /getTableOfContents/,
    /TOCItemType/,
    /llms\(source\)/,
    /page\.data\.getText\(\\"processed\\"\)/,
    /createOpenAPI/,
    /staticSource/,
    /loaderPlugin/,
    /createProxy/,
    /proxyUrl/,
    /createAPIPage/,
    /createCodeUsageGeneratorRegistry/,
    /createFromSource/,
    /staticGET/,
    /useDocsSearch/,
  ].forEach((apiPattern) => assert.match(packageCatalog, apiPattern));
  assert.match(dashboardReadme, /Documentation System Workflow/);
  assert.match(dashboardReadme, /content\/fumadocs-next/);
});

test("documentation system is the official front-facing package name", () => {
  assert.match(packageCatalog, /officialName: "Documentation System"/);
  assert.match(packageCatalog, /upstreamPackage: "fumadocs"/);
  assert.match(packageCatalog, /basedOn: "G:\/WWW\/inspirations\/fumadocs"/);
  assert.match(packageCatalog, /name: "Documentation System"/);
  assert.doesNotMatch(packageCatalog, /name: "Fumadocs Docs Help Workflow"/);

  assert.match(forge, /officialName: "Documentation System"/);
  assert.match(forge, /upstreamPackage: "fumadocs"/);
  assert.match(forge, /basedOn: "G:\/WWW\/inspirations\/fumadocs"/);

  assert.match(dashboardLib, /officialName: "Documentation System"/);
  assert.match(dashboardLib, /upstreamPackage: "fumadocs"/);
  assert.match(dashboardComponent, />Documentation System Workflow</);
  assert.doesNotMatch(dashboardComponent, />Fumadocs Docs Workflow</);

  assert.match(docsStatus, />Documentation System</);
  assert.match(docsStatus, /Documentation System powers the dashboard help panel/);
  assert.doesNotMatch(docsStatus, /Fumadocs route signals/);
  assert.doesNotMatch(docsStatus, /local Fumadocs route receipt/);

  assert.match(packageDoc, /^# Documentation System/m);
  assert.match(packageDoc, /Official DX package name: `Documentation System`/);
  assert.match(packageDoc, /Upstream package: `fumadocs`/);
  assert.doesNotMatch(packageDoc, /^# content\/fumadocs-next/m);

  const receipt = JSON.parse(read(dashboardReceiptPath));
  assert.equal(receipt.official_dx_package_name, "Documentation System");
  assert.equal(receipt.package_name, "Documentation System");
  assert.equal(receipt.upstream_package, "fumadocs");
  assert.equal(receipt.based_on, "G:/WWW/inspirations/fumadocs");
});

test("documentation system receipt carries selected surfaces and source hashes", () => {
  assert.match(packageCatalog, /receiptIntegrity: \{/);
  assert.match(packageCatalog, /hashAlgorithm: "sha256"/);
  assert.match(packageCatalog, /staleReceiptPolicy:/);
  assert.match(forge, /receiptIntegrity: \{/);
  assert.match(forge, /hashAlgorithm: "sha256"/);
  assert.match(packageDoc, /## Receipt Integrity/);
  assert.match(packageDoc, /stale receipt/);

  const receipt = JSON.parse(read(dashboardReceiptPath));
  assert.equal(receipt.source_hashes.algorithm, "sha256");
  assert.match(receipt.source_hashes.stale_receipt_policy, /stale/i);

  const selectedSurfaceIds = receipt.selected_surfaces.map((surface) => surface.id);
  [
    "docs-app-router",
    "dashboard-help-workflow",
    "llm-export",
    "openapi-reference",
    "search-index",
  ].forEach((surfaceId) => {
    assert.ok(selectedSurfaceIds.includes(surfaceId), `${surfaceId} should be receipt-backed`);
  });

  const hashByPath = new Map(
    receipt.source_hashes.files.map((entry) => [entry.path, entry.sha256]),
  );
  [
    "core/src/ecosystem/forge_fumadocs.rs",
    "examples/template/package-catalog.ts",
    "examples/template/docs-status.tsx",
    "examples/dashboard/src/lib/fumadocsDocsWorkflow.ts",
    "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
    "docs/packages/content-fumadocs-next.md",
  ].forEach((relativePath) => {
    const recorded = hashByPath.get(relativePath);

    assert.match(recorded, /^[a-f0-9]{64}$/);
    assert.equal(
      recorded,
      sha256(path.join(root, relativePath)),
      `${relativePath} hash should match the current source surface`,
    );
  });
});

test("documentation system UI surfaces declare dx-style compatibility", () => {
  assert.match(packageCatalog, /dxStyleCompatibility: \{/);
  assert.match(forge, /dxStyleCompatibility: \{/);
  assert.match(packageDoc, /## DX-Style Compatibility/);

  const receipt = JSON.parse(read(dashboardReceiptPath));
  const compatibility = receipt.dx_style_compatibility;

  assert.equal(compatibility.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(compatibility.token_source, "styles/globals.css");
  assert.equal(compatibility.generated_css, "styles/globals.css");
  assert.equal(compatibility.runtime_proof, false);
  assert.match(compatibility.runtime_limitations.join(" "), /governed browser QA/);
  assert.ok(compatibility.visible_surfaces.includes("dashboard-help-workflow"));
  assert.ok(compatibility.visible_surfaces.includes("docs-app-router"));
  assert.ok(compatibility.visible_surfaces.includes("openapi-reference"));
  assert.ok(
    compatibility.source_files.includes("examples/template/docs-status.tsx"),
  );
  assert.ok(
    compatibility.source_files.includes(
      "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
    ),
  );

  [docsStatus, dashboardComponent].forEach((surfaceSource) => {
    assert.match(surfaceSource, /data-dx-style-surface="documentation-system"/);
    assert.doesNotMatch(surfaceSource, /style=\{\{/);
    assert.doesNotMatch(surfaceSource, /#[0-9a-fA-F]{3,8}/);
  });
});

test("documentation system feeds shared package status read model", () => {
  assert.match(forgePackageStatusReadModel, /export const documentationSystemPackageVisibility/);
  assert.match(forgePackageStatusReadModel, /dxStyleCompatibility: \{/);
  assert.match(forgePackageStatusReadModel, /sourceHashes: \{/);
  assert.match(forgePackageStatusSource, /documentationSystemPackageVisibility/);
  assert.match(forgePackageStatusSource, /documentationSystemVisibility/);
  assert.match(packageDoc, /package-status read model/);

  const status = JSON.parse(
    read(path.join(root, "examples", "template", ".dx", "forge", "package-status.json")),
  );
  const receipt = JSON.parse(read(dashboardReceiptPath));
  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "content/fumadocs-next",
  );

  assert.ok(visibility, "Documentation System package-status visibility row is missing");
  assert.equal(visibility.official_package_name, "Documentation System");
  assert.equal(visibility.upstream_package, "fumadocs");
  assert.equal(visibility.upstream_version, "16.8.12");
  assert.equal(visibility.status, "present");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(
    visibility.package_receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
  );

  [
    "docs-app-router",
    "dashboard-help-workflow",
    "llm-export",
    "openapi-reference",
    "search-index",
  ].forEach((surfaceId) => {
    assert.ok(
      visibility.selected_surfaces.some((surface) => surface.surface_id === surfaceId),
      `${surfaceId} missing from Documentation System package-status visibility row`,
    );
  });

  for (const metric of [
    "documentation_system_receipt_present",
    "documentation_system_receipt_stale",
    "documentation_system_missing_receipt",
    "documentation_system_blocked_surface",
    "documentation_system_unsupported_surface",
    "documentation_system_hash_manifest_present",
    "documentation_system_hash_mismatch",
    "documentation_system_dx_style_compatibility_present",
    "documentation_system_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Documentation System visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(forgePackageStatusReadModel, new RegExp(metric));
  }

  assert.equal(visibility.source_hashes.algorithm, receipt.source_hashes.algorithm);
  assert.equal(
    visibility.source_hashes.files["examples/template/docs-status.tsx"],
    receipt.source_hashes.files.find(
      (entry) => entry.path === "examples/template/docs-status.tsx",
    ).sha256,
  );
  assert.equal(
    visibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(visibility.dx_style_compatibility.status, "present");
  assert.equal(visibility.dx_style_compatibility.token_source, "styles/globals.css");
  assert.ok(
    visibility.dx_style_compatibility.visible_surfaces.includes("dashboard-help-workflow"),
  );
  assert.ok(
    status.zed_receipt_surfaces.includes("documentation-system:docs-help-changelog"),
    "Documentation System Zed receipt surface is missing",
  );
});

test("template surface registry keeps fumadocs discoverable as a docs workflow", () => {
  assert.match(templateSurfaceRegistry, /id: "content-docs"/);
  assert.match(templateSurfaceRegistry, /slot: "docs-help-changelog"/);
  assert.match(
    templateSurfaceRegistry,
    /componentSelector: '\[data-dx-component="launch-fumadocs-docs-workflow"\]'/,
  );
  assert.match(
    templateSurfaceRegistry,
    /packageIds: \["content\/fumadocs-next", "content\/react-markdown", "i18n\/next-intl"\]/,
  );
  assert.match(
    templateSurfaceRegistry,
    /"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );
  assert.match(templateSurfaceRegistry, /"safe docs receipt"/);
  assert.match(templateSurfaceRegistry, /"static docs-only cards"/);
  assert.match(
    templateSurfaceRegistry,
    /sourceGuard: "dx run --test \.\\\\benchmarks\\\\fumadocs-dashboard-workflow\.test\.ts"/,
  );
});

test("dx studio contract maps fumadocs workflow interactions back to source", () => {
  assert.match(dxStudioEditContract, /id: "docs-help-changelog-workflow"/);
  assert.match(
    dxStudioEditContract,
    /selector: '\[data-dx-component="launch-fumadocs-docs-workflow"\]'/,
  );
  assert.match(
    dxStudioEditContract,
    /sourceFile: "examples\/template\/docs-status\.tsx"/,
  );
  assert.match(
    dxStudioEditContract,
    /\[data-dx-fumadocs-action="safe-local-route-preview"\]/,
  );
  assert.match(dxStudioEditContract, /\[data-dx-fumadocs-page-option\]/);
  assert.match(
    dxStudioEditContract,
    /\[data-dx-fumadocs-rendered-markdown="active-page"\]/,
  );
  assert.match(dxStudioEditContract, /data-dx-fumadocs-local-response/);
  assert.match(dxStudioEditContract, /data-dx-fumadocs-receipt-route/);
  assert.match(dxStudioEditContract, /data-dx-fumadocs-missing-config/);
  assert.match(dxStudioEditContract, /data-dx-docs-openapi-code-usage/);
  assert.match(dxStudioEditContract, /data-dx-docs-openapi-proxy/);
  assert.match(
    dxStudioEditContract,
    /receiptPath:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );
});

test("fumadocs dashboard receipt records the visible launch workflow", () => {
  assert.ok(
    fs.existsSync(dashboardReceiptPath),
    "Fumadocs dashboard workflow receipt should be source-owned",
  );

  const receipt = JSON.parse(read(dashboardReceiptPath));

  assert.equal(receipt.schema, "dx.forge.package_dashboard_workflow_receipt");
  assert.equal(receipt.package_id, "content/fumadocs-next");
  assert.equal(receipt.route, "/");
  assert.equal(receipt.component, "launch-fumadocs-docs-workflow");
  assert.equal(receipt.workflow, "docs-help-changelog");
  assert.equal(receipt.runtime_identity, "docs-workflow");
  assert.equal(
    receipt.runtime_manifest_selector,
    '[data-dx-component="launch-fumadocs-docs-workflow"]',
  );
  assert.equal(receipt.status, "source-coded-runtime-pending");
  assert.ok(receipt.no_runtime_execution);
  assert.equal(receipt.reality_audit.verdict, "REAL");
  assert.match(receipt.reality_audit.classification_scope, /Forge package/);
  assert.match(receipt.reality_audit.partial_scope, /live Fumadocs renderer/);
  assert.equal(receipt.reality_audit.upstream_source.mirror, "G:/WWW/inspirations/fumadocs");
  assert.ok(
    receipt.reality_audit.upstream_source.evidence_files.includes(
      "packages/core/src/source/loader.ts",
    ),
  );
  assert.ok(
    receipt.reality_audit.upstream_source.public_apis.includes("createOpenAPI"),
  );
  assert.ok(
    receipt.reality_audit.forge_package_files.includes(
      "core/src/ecosystem/forge_fumadocs.rs",
    ),
  );
  assert.ok(
    receipt.reality_audit.dashboard_consumers.includes(
      "examples/template/docs-status.tsx",
    ),
  );
  assert.ok(
    receipt.reality_audit.receipt_manifest_files.includes(
      "examples/template/template-surface-registry.ts",
    ),
  );
  assert.ok(
    receipt.reality_audit.guard_files.includes(
      "benchmarks/fumadocs-dashboard-workflow.test.ts",
    ),
  );
  assert.ok(
    receipt.reality_audit.visible_workflow_proof.includes(
      "preview-local-route-receipt",
    ),
  );
  assert.equal(
    receipt.template_surface_registry.file,
    "examples/template/template-surface-registry.ts",
  );
  assert.equal(receipt.template_surface_registry.surface_id, "content-docs");
  assert.equal(
    receipt.template_surface_registry.component_selector,
    '[data-dx-component="launch-fumadocs-docs-workflow"]',
  );
  assert.equal(
    receipt.template_surface_registry.source_guard,
    "dx run --test .\\benchmarks\\fumadocs-dashboard-workflow.test.ts",
  );
  assert.equal(
    receipt.studio_edit_contract.file,
    "examples/template/dx-studio-edit-contract.ts",
  );
  assert.equal(receipt.studio_edit_contract.surface_id, "docs-help-changelog-workflow");
  assert.equal(
    receipt.studio_edit_contract.component_selector,
    '[data-dx-component="launch-fumadocs-docs-workflow"]',
  );
  assert.equal(
    receipt.studio_edit_contract.receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
  );
  assert.ok(
    receipt.studio_edit_contract.interaction_selectors.includes(
      '[data-dx-fumadocs-action="safe-local-route-preview"]',
    ),
  );
  assert.ok(
    receipt.studio_edit_contract.interaction_selectors.includes(
      "[data-dx-fumadocs-page-option]",
    ),
  );
  assert.ok(
    receipt.studio_edit_contract.interaction_selectors.includes(
      '[data-dx-fumadocs-rendered-markdown="active-page"]',
    ),
  );
  assert.ok(
    receipt.studio_edit_contract.state_markers.includes(
      "data-dx-fumadocs-local-response",
    ),
  );
  assert.ok(
    receipt.studio_edit_contract.state_markers.includes(
      "data-dx-docs-openapi-code-usage",
    ),
  );
  assert.ok(
    receipt.studio_edit_contract.state_markers.includes(
      "data-dx-docs-openapi-proxy",
    ),
  );
  assert.equal(
    receipt.runtime_preview_manifest.generator,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(receipt.runtime_preview_manifest.file, "public/preview-manifest.json");
  assert.equal(receipt.runtime_preview_manifest.surface_id, "launch-runtime-docs");
  assert.ok(
    receipt.runtime_preview_manifest.interaction_selectors.includes(
      "[data-dx-fumadocs-page-option]",
    ),
  );
  assert.ok(
    receipt.runtime_preview_manifest.interaction_selectors.includes(
      '[data-dx-fumadocs-rendered-markdown="active-page"]',
    ),
  );
  assert.ok(
    receipt.runtime_preview_manifest.interaction_selectors.includes(
      '[data-dx-fumadocs-changelog="launch-docs"]',
    ),
  );
  assert.ok(
    receipt.runtime_preview_manifest.state_markers.includes(
      "data-dx-fumadocs-missing-config",
    ),
  );
  assert.ok(
    receipt.runtime_preview_manifest.state_markers.includes(
      "data-dx-docs-openapi-code-usage",
    ),
  );
  assert.ok(
    receipt.runtime_preview_manifest.state_markers.includes(
      "data-dx-docs-openapi-proxy",
    ),
  );
  assert.equal(
    receipt.zed_studio_manifest.file,
    "dx-www/src/cli/studio_manifest.rs",
  );
  assert.equal(receipt.zed_studio_manifest.package_surface, "content/fumadocs-next");
  assert.ok(
    receipt.zed_studio_manifest.interaction_selectors.includes(
      "[data-dx-fumadocs-page-option]",
    ),
  );
  assert.ok(
    receipt.zed_studio_manifest.data_dx_markers.includes(
      "data-dx-fumadocs-rendered-markdown",
    ),
  );
  assert.ok(
    receipt.zed_studio_manifest.data_dx_markers.includes(
      "data-dx-fumadocs-missing-config",
    ),
  );
  assert.ok(
    receipt.zed_studio_manifest.data_dx_markers.includes(
      "data-dx-docs-openapi-code-usage",
    ),
  );
  assert.ok(
    receipt.zed_studio_manifest.data_dx_markers.includes(
      "data-dx-docs-openapi-proxy",
    ),
  );
  assert.deepEqual(receipt.required_env, ["DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"]);
  assert.ok(
    receipt.app_owned_boundaries.includes("runtime dependency installation"),
  );
  assert.ok(
    receipt.app_owned_boundaries.includes("OpenAPI allowed origins"),
  );
  assert.ok(
    receipt.app_owned_boundaries.includes("governed browser proof"),
  );
  assert.ok(
    receipt.missing_runtime_features.includes("live Fumadocs renderer proof"),
  );
  assert.ok(
    receipt.missing_runtime_features.includes("hosted search indexing"),
  );
  assert.ok(
    receipt.missing_runtime_features.includes("production OpenAPI proxy execution"),
  );
  assert.ok(
    receipt.missing_runtime_features.includes("governed browser QA"),
  );
  assert.ok(
    receipt.source_files.includes("examples/template/docs-status.tsx"),
  );
  assert.ok(
    receipt.source_files.includes("examples/template/template-surface-registry.ts"),
  );
  assert.ok(
    receipt.materialized_files.includes("components/template-app/docs-status.tsx"),
  );
  assert.ok(
    receipt.materialized_files.includes("lib/fumadocs/openapi-code-usage.ts"),
  );
  [
    "lib/fumadocs/layout.tsx",
    "components/mdx.tsx",
    "components/api-page.tsx",
    "components/api-page.client.tsx",
    "app/docs/layout.tsx",
    "app/docs/[[...slug]]/page.tsx",
    "app/api/search/route.ts",
    "app/api/openapi/proxy/route.ts",
    "openapi/dx-www.yaml",
  ].forEach((filePath) => {
    assert.ok(
      receipt.materialized_files.includes(filePath),
      `${filePath} should stay in the Fumadocs dashboard workflow receipt`,
    );
  });
  assert.ok(
    receipt.materialized_files.includes(
      ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
    ),
  );
  assert.ok(
    receipt.upstream_public_apis.includes("getBreadcrumbItems"),
  );
  assert.ok(
    receipt.upstream_public_apis.includes("llms"),
  );
  [
    "defineDocs",
    "defineConfig",
    "lucideIconsPlugin",
    "statusBadgesPlugin",
    "slugsFromData",
    "DocsLayout",
    "RootProvider",
    "DocsPage",
    "TOCItemType",
    'page.data.getText("processed")',
    "staticSource",
    "loaderPlugin",
    "createProxy",
    "proxyUrl",
    "createAPIPage",
    "createRelativeLink",
    "defaultMdxComponents",
  ].forEach((apiName) => {
    assert.ok(
      receipt.upstream_public_apis.includes(apiName),
      `${apiName} should stay recorded as a Fumadocs UI/API dependency`,
    );
  });
  assert.ok(
    receipt.upstream_public_apis.includes("dx config boundary"),
    "dx config boundary should stay recorded as the app-owned config boundary",
  );
  assert.ok(
    !receipt.upstream_public_apis.includes("createMDX"),
    "Fumadocs receipt should not advertise the removed template-local Next MDX adapter",
  );
  assert.ok(
    receipt.app_owned_boundaries.includes(
      "extensionless dx config owns WWW/Fumadocs adapter settings",
    ),
  );
  assert.ok(
    receipt.upstream_public_apis.includes("createCodeUsageGeneratorRegistry"),
  );
  assert.ok(receipt.upstream_public_apis.includes("registerDefault"));
  assert.ok(receipt.upstream_public_apis.includes("defineClientConfig"));
  assert.ok(
    receipt.local_readiness_interactions.includes("select-docs-route"),
  );
  assert.ok(
    receipt.local_readiness_interactions.includes("sync-docs-route-to-mission-control"),
  );
  assert.ok(
    receipt.local_readiness_interactions.includes("preview-local-route-receipt"),
  );
  assert.equal(receipt.local_demo_interactions, undefined);
  assert.ok(
    receipt.stable_markers.includes('data-dx-package="content/fumadocs-next"'),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-component="launch-fumadocs-docs-workflow"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-dashboard-workflow="docs-help-changelog"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes('data-dx-dashboard-card="docs-help"'),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-fumadocs-dashboard-target="mission-control-docs"',
    ),
  );
  assert.ok(
    receipt.stable_markers.includes(
      'data-dx-fumadocs-interaction="page-tree-selector"',
    ),
  );
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-page-option"));
  assert.ok(
    receipt.stable_markers.includes("data-dx-fumadocs-rendered-markdown"),
  );
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-changelog"));
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-rendered-route"));
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-selected-page"));
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-toc-count"));
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-receipt-route"));
  assert.ok(receipt.stable_markers.includes("data-dx-fumadocs-missing-config"));
  assert.ok(
    receipt.stable_markers.includes("data-dx-docs-openapi-code-usage"),
  );
  assert.ok(receipt.stable_markers.includes("data-dx-docs-openapi-proxy"));
  assert.equal(
    receipt.accessibility_contract.page_tree_buttons,
    "aria-pressed mirrors data-dx-fumadocs-page-selected",
  );
  assert.match(receipt.accessibility_contract.local_receipt, /aria-live=polite/);
  assert.ok(
    receipt.guards.includes(
      "node --test .\\benchmarks\\fumadocs-dashboard-workflow.test.ts",
    ),
  );
});

test("fumadocs dashboard receipt is materialized with the generated launch route", () => {
  assert.match(templateRouteContract, /fumadocsDocsDashboardWorkflow/);
  assert.match(
    templateRouteContract,
    /"\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );
  assert.match(
    templateRouteContract,
    /file: "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );
  assert.match(
    templateRouteContract,
    /materializedFile: "components\/template-app\/docs-status\.tsx"/,
  );
  assert.match(
    cli,
    /NEXT_FAMILIAR_DOCS_DASHBOARD_RECEIPT_JSON/,
  );
  assert.match(
    cli,
    /include_str!\(\s*"..\/..\/..\/examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"\s*\)/,
  );
  assert.match(
    cli,
    /"\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );
  assert.match(
    studioManifest,
    /"receipt_paths": \[[\s\S]*examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json/,
  );
  assert.match(templateRouteContract, /data-dx-fumadocs-page-option/);
  assert.match(templateRouteContract, /data-dx-fumadocs-rendered-markdown/);
  assert.match(templateRouteContract, /data-dx-fumadocs-missing-config/);
  assert.match(templateRouteContract, /data-dx-docs-openapi-code-usage/);
  assert.match(templateRouteContract, /data-dx-docs-openapi-proxy/);
  assert.match(studioManifest, /"interaction_selectors": \[[\s\S]*\[data-dx-fumadocs-page-option\]/);
  assert.match(studioManifest, /data-dx-fumadocs-rendered-markdown/);
  assert.match(studioManifest, /data-dx-fumadocs-missing-config/);
  assert.match(studioManifest, /data-dx-docs-openapi-code-usage/);
  assert.match(studioManifest, /data-dx-docs-openapi-proxy/);
});

test("fumadocs materializer output keeps the docs workflow visible and editable", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-fumadocs-runtime-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

  const launch = read(path.join(dir, "pages", "index.html"));
  assert.match(launch, /data-dx-component="launch-fumadocs-docs-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="docs-help-changelog"/);
  assert.match(launch, /data-dx-fumadocs-action="safe-local-route-preview"/);
  assert.match(launch, /data-dx-fumadocs-local-response="idle"/);
  assert.match(launch, /aria-pressed="true"/);
  assert.match(launch, /role="status"[\s\S]{0,80}aria-live="polite"/);
  assert.doesNotMatch(launch, /data-dx-component="fumadocs-docs-navigation-proof"/);
  assert.doesNotMatch(launch, /docs-proof/);

  const runtime = read(path.join(dir, "public", "launch-runtime.js"));
  assert.match(runtime, /\[data-dx-component="launch-fumadocs-docs-workflow"\]/);
  assert.match(runtime, /data-dx-fumadocs-local-response/);
  assert.match(runtime, /"aria-pressed"/);

  const manifest = JSON.parse(read(path.join(dir, "public", "preview-manifest.json")));
  const surface = manifest.editContract.editableSurfaces.find(
    (candidate) => candidate.id === "launch-runtime-docs",
  );

  assert.ok(surface, "expected Fumadocs docs workflow edit surface");
  assert.equal(surface.selector, '[data-dx-component="launch-fumadocs-docs-workflow"]');
  assert.equal(surface.sourceFile, "pages/index.html");
  assert.ok(surface.packageIds.includes("content/fumadocs-next"));
  assert.ok(surface.packageIds.includes("content/react-markdown"));
  assert.ok(surface.operations.includes("move_reorder_section"));
  assert.ok(surface.operations.includes("update_text_content"));
  assert.ok(surface.operations.includes("insert_icon_media"));
  assert.ok(
    surface.interactionSelectors.includes(
      '[data-dx-fumadocs-action="safe-local-route-preview"]',
    ),
  );
  assert.ok(surface.interactionSelectors.includes("[data-dx-fumadocs-page-option]"));
  assert.ok(
    surface.interactionSelectors.includes(
      '[data-dx-fumadocs-rendered-markdown="active-page"]',
    ),
  );
  assert.ok(
    surface.interactionSelectors.includes(
      '[data-dx-fumadocs-changelog="launch-docs"]',
    ),
  );
  assert.ok(surface.stateMarkers.includes("data-dx-fumadocs-local-response"));
  assert.ok(surface.stateMarkers.includes("data-dx-fumadocs-missing-config"));
  assert.ok(surface.stateMarkers.includes("data-dx-docs-openapi-code-usage"));
  assert.ok(surface.stateMarkers.includes("data-dx-docs-openapi-proxy"));
  assert.equal(
    surface.receiptPath,
    ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
  );
  const launchRoute = manifest.routes.find(
    (candidate) =>
      candidate.route === "/" &&
      candidate.sourceFile === "pages/index.html" &&
      candidate.dataDxMarkers?.includes("data-dx-fumadocs-page-option"),
  );
  assert.ok(launchRoute, "expected root dashboard route metadata in generated preview manifest");
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-fumadocs-page-option"));
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-fumadocs-rendered-markdown"),
  );
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-fumadocs-changelog"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-fumadocs-missing-config"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-docs-openapi-code-usage"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-docs-openapi-proxy"));
  assert.ok(
    !manifest.editContract.editableSurfaces.some((candidate) =>
      candidate.selector.includes("fumadocs-docs-navigation-proof"),
    ),
  );
});

test("launch route uses fumadocs as a dashboard docs workflow", () => {
  assert.match(
    launchShell,
    /data-dx-component="launch-fumadocs-docs-workflow"/,
  );
  assert.match(
    launchShell,
    /data-dx-dashboard-workflow="docs-help-changelog"/,
  );
  assert.match(
    launchShell,
    /data-dx-product-surface="dashboard-help-content"/,
  );
  assert.doesNotMatch(launchShell, /data-dx-component="fumadocs-proof"/);
  assert.doesNotMatch(launchShell, /data-dx-edit-id="launch\.docs"[\s\S]{0,160}data-dx-edit-kind="proof-card"/);
  assert.match(
    docsStatus,
    /from "@\/lib\/fumadocs\/dashboard-workflow"/,
  );
  assert.match(docsStatus, /dxFumadocsDashboardPages/);
  assert.match(docsStatus, /createFumadocsNavigationReceipt/);
  assert.match(
    docsStatus,
    /data-dx-component="launch-fumadocs-docs-workflow"/,
  );
  assert.match(
    docsStatus,
    /data-dx-dashboard-workflow="docs-help-changelog"/,
  );
  assert.match(
    docsStatus,
    /data-dx-fumadocs-action="safe-local-route-preview"/,
  );
  assert.match(docsStatus, /aria-pressed=\{selected\}/);
  assert.match(docsStatus, /role="status"/);
  assert.match(docsStatus, /aria-live="polite"/);
  assert.match(docsStatus, /data-dx-fumadocs-local-response=/);
  assert.match(docsStatus, /data-dx-fumadocs-receipt-route=/);
  assert.match(docsStatus, /data-dx-editable-text="launch-docs-help-title"/);
  assert.match(
    runtimeLaunchPage,
    /data-dx-component="launch-fumadocs-docs-workflow"/,
  );
  assert.match(
    runtimeLaunchPage,
    /id="fumadocs-workflow"[\s\S]{0,120}class="card wide docs-workflow"/,
  );
  assert.doesNotMatch(
    runtimeLaunchPage,
    /id="fumadocs-workflow"[\s\S]{0,120}docs-proof/,
  );
  assert.match(runtimeLaunchCss, /\.docs-workflow/);
  assert.doesNotMatch(runtimeLaunchCss, /\.docs-proof/);
  assert.match(
    runtimeLaunchPage,
    /data-dx-dashboard-workflow="docs-help-changelog"/,
  );
  assert.match(
    runtimeLaunchPage,
    /data-dx-product-surface="dashboard-help-content"/,
  );
  assert.match(
    runtimeLaunchPage,
    /data-dx-fumadocs-action="safe-local-route-preview"/,
  );
  assert.match(runtimeLaunchPage, /aria-pressed="true"/);
  assert.match(runtimeLaunchPage, /role="status"[\s\S]{0,80}aria-live="polite"/);
  assert.match(runtimeLaunchPage, /id="fumadocs-receipt"/);
  assert.match(
    launchShell,
    /data-dx-dashboard-target="mission-control-docs"/,
  );
  assert.match(
    runtimeLaunchPage,
    /data-dx-dashboard-card="docs-help"/,
  );
  assert.match(
    runtimeLaunchPage,
    /data-dx-fumadocs-dashboard-target="mission-control-docs"/,
  );
  assert.match(
    runtimeLaunchPage,
    /id="mission-docs-status"[\s\S]{0,120}data-dx-dashboard-metric="docs-help"/,
  );
  assert.match(runtimeLaunchPage, /id="mission-docs-detail"/);
  assert.match(
    runtimeLaunchJs,
    /\[data-dx-component="launch-fumadocs-docs-workflow"\]/,
  );
  assert.match(runtimeLaunchJs, /data-dx-fumadocs-local-response/);
  assert.match(runtimeLaunchJs, /"aria-pressed"/);
  assert.match(runtimeLaunchJs, /dxDashboardDocsRoute/);
  assert.match(runtimeLaunchJs, /dxDashboardDocsReceipt/);
  assert.match(runtimeLaunchJs, /#mission-docs-status/);
  assert.match(runtimeLaunchJs, /#mission-docs-detail/);
  assert.match(
    runtimeLaunchJs,
    /Documentation System route updated the launch dashboard/,
  );
  assert.match(
    runtimeLaunchJs,
    /Documentation System route receipt previewed in mission control/,
  );
});

test("fumadocs package docs name the real and deferred boundaries", () => {
  assert.match(packageDoc, /# Documentation System/);
  assert.match(packageDoc, /G:\\WWW\\inspirations\\fumadocs/);
  assert.match(packageDoc, /getBreadcrumbItems/);
  assert.match(packageDoc, /flattenTree/);
  assert.match(packageDoc, /llms\(source\)/);
  assert.match(packageDoc, /createOpenAPI/);
  assert.match(packageDoc, /createCodeUsageGeneratorRegistry/);
  assert.match(packageDoc, /lib\/fumadocs\/layout\.tsx/);
  assert.match(packageDoc, /components\/api-page\.tsx/);
  assert.match(packageDoc, /app\/docs\/layout\.tsx/);
  assert.match(packageDoc, /app\/docs\/\[\[...slug\]\]\/page\.tsx/);
  assert.match(packageDoc, /app\/api\/openapi\/proxy\/route\.ts/);
  assert.match(packageDoc, /openapi\/dx-launch\.yaml/);
  assert.match(packageDoc, /createFumadocsNavigationReceipt/);
  assert.match(packageDoc, /data-dx-component="dashboard-fumadocs-docs-workflow"/);
  assert.match(packageDoc, /<dx-icon name="pack:fumadocs" \/>/);
  assert.match(packageDoc, /Reality audit/);
  assert.match(packageDoc, /classifies the slice as `REAL`/);
  assert.match(packageDoc, /Zed manifest handoff/);
  assert.match(packageDoc, /DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS/);
  assert.match(packageDoc, /dx run --test \.\\benchmarks\\fumadocs-dashboard-workflow\.test\.ts/);
  assert.match(packageDoc, /Intentionally Deferred/);
});
