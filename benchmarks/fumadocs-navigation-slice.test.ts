const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "fumadocs");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

const forge = read(path.join(root, "core", "src", "ecosystem", "forge_fumadocs.rs"));
const docsStatus = read(path.join(root, "examples", "template", "docs-status.tsx"));
const launchShell = read(path.join(root, "examples", "template", "template-shell.tsx"));
const launchRoute = read(path.join(root, "examples", "template", "app", "page.tsx"));
const runtimeLaunch = read(path.join(root, "tools", "launch", "runtime-template", "pages", "index.html"));
const runtimeScript = read(path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts"));
const runtimeCss = read(path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.css"));
const packageCatalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
const scorecard = read(path.join(root, "core", "src", "ecosystem", "forge_scorecard.rs"));
const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));
const trustPolicy = read(path.join(root, "core", "src", "ecosystem", "forge_trust_policy.rs"));

test("upstream fumadocs exposes real navigation APIs", () => {
  const packageJson = read(path.join(mirror, "packages", "core", "package.json"));
  const breadcrumb = read(path.join(mirror, "packages", "core", "src", "breadcrumb.tsx"));
  const pageTree = read(path.join(mirror, "packages", "core", "src", "page-tree", "utils.ts"));
  const upstreamTest = read(path.join(mirror, "packages", "core", "test", "index.test.ts"));

  assert.match(packageJson, /"\.\/breadcrumb"/);
  assert.match(packageJson, /"\.\/page-tree"/);
  assert.match(breadcrumb, /export function getBreadcrumbItems/);
  assert.match(breadcrumb, /export function useBreadcrumb/);
  assert.match(pageTree, /export function flattenTree/);
  assert.match(pageTree, /export function findNeighbour/);
  assert.match(pageTree, /export function getPageTreePeers/);
  assert.match(upstreamTest, /getBreadcrumbItems\('/);
  assert.match(upstreamTest, /findNeighbour\(tree/);
});

test("fumadocs slice materializes a navigation contract", () => {
  assert.match(forge, /js\/lib\/fumadocs\/navigation\.ts/);
  assert.match(forge, /import \{ getBreadcrumbItems \} from "fumadocs-core\/breadcrumb"/);
  assert.match(forge, /import \{ findNeighbour, flattenTree, getPageTreePeers \} from "fumadocs-core\/page-tree"/);
  assert.match(forge, /dxFumadocsNavigationContract/);
  assert.match(forge, /getDxFumadocsPageBreadcrumbs/);
  assert.match(forge, /getDxFumadocsNavigationSnapshot/);
  assert.match(forge, /source\.getPageTree\(locale\)/);
  assert.match(forge, /flattenTree\(tree\.children\)/);
  assert.match(forge, /findNeighbour\(tree, url\)/);
  assert.match(forge, /getPageTreePeers\(tree, url\)/);
});

test("fumadocs navigation metadata is discoverable and honest", () => {
  assert.match(forge, /navigationFile: "lib\/fumadocs\/navigation\.ts"/);
  assert.match(forge, /getBreadcrumbItems from fumadocs-core\/breadcrumb/);
  assert.match(forge, /findNeighbour from fumadocs-core\/page-tree/);
  assert.match(packageCatalog, /navigation policy/);
  assert.match(scorecard, /navigation snapshot/);
  assert.match(security, /getBreadcrumbItems/);
  assert.match(trustPolicy, /navigation policy/);
  assert.match(cli, /dxFumadocsNavigationContract/);
});

test("launch template consumes the fumadocs navigation contract", () => {
  assert.match(docsStatus, /dxFumadocsNavigationContract/);
  assert.match(docsStatus, /data-dx-docs-navigation/);
  assert.match(docsStatus, /data-dx-fumadocs-navigation-snapshot/);
});

test("launch template shows an interactive fumadocs docs workflow", () => {
  assert.match(docsStatus, /"use client";/);
  assert.match(docsStatus, /data-dx-package="content\/fumadocs-next"/);
  assert.match(docsStatus, /data-dx-component="launch-fumadocs-docs-workflow"/);
  assert.doesNotMatch(docsStatus, /data-dx-component="fumadocs-docs-navigation-proof"/);
  assert.match(docsStatus, /data-dx-fumadocs-interaction="page-tree-selector"/);
  assert.match(docsStatus, /data-dx-fumadocs-selected-page=\{activePage\.id\}/);
  assert.match(docsStatus, /data-dx-fumadocs-rendered-route=\{activePage\.route\}/);
  assert.match(docsStatus, /data-dx-fumadocs-toc-count=\{activePage\.toc\.length\}/);
  assert.match(docsStatus, /aria-pressed=\{selected\}/);
  assert.match(docsStatus, /role="status"/);
  assert.match(docsStatus, /aria-live="polite"/);
  assert.match(docsStatus, /setActivePageId/);
  assert.match(docsStatus, /<DxMarkdown skipHtml>\{activeMarkdown\}<\/DxMarkdown>/);
  assert.match(launchShell, /<LaunchDocsStatus \/>/);
  assert.match(launchRoute, /<TemplateShell \/>/);
  assert.match(cli, /NEXT_FAMILIAR_DOCS_STATUS_TSX/);
  assert.match(cli, /NEXT_FAMILIAR_HOME_ROUTE_PAGE_TSX/);
  assert.match(cli, /"components\/template-app\/docs-status\.tsx"/);
});

test("materialized launch runtime shows fumadocs docs behavior", () => {
  assert.match(runtimeLaunch, /data-dx-route="\/"/);
  assert.match(runtimeLaunch, /data-dx-package="content\/fumadocs-next"/);
  assert.match(runtimeLaunch, /data-dx-component="launch-fumadocs-docs-workflow"/);
  assert.match(runtimeLaunch, /class="card wide docs-workflow"/);
  assert.doesNotMatch(runtimeLaunch, /id="fumadocs-workflow"[\s\S]{0,120}docs-proof/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-interaction="page-tree-selector"/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-selected-page="overview"/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-rendered-route="\/docs"/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-toc-count="3"/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-page-option="api-reference"/);
  assert.match(runtimeLaunch, /aria-pressed="true"/);
  assert.match(runtimeLaunch, /role="status"[\s\S]{0,80}aria-live="polite"/);
  assert.match(runtimeLaunch, /data-dx-fumadocs-rendered-markdown="active-page"/);
  assert.match(runtimeLaunch, /data-dx-component="markdown-docs-card"/);
  assert.match(runtimeLaunch, /data-dx-package="content\/react-markdown"/);
  assert.match(runtimeScript, /function bindFumadocsDocs\(\)/);
  assert.match(runtimeScript, /data-dx-fumadocs-page-option/);
  assert.match(runtimeScript, /data-dx-fumadocs-selected-page/);
  assert.match(runtimeScript, /data-dx-fumadocs-rendered-route/);
  assert.match(runtimeScript, /"aria-pressed"/);
  assert.match(runtimeScript, /bindFumadocsDocs\(\)/);
  assert.match(runtimeCss, /\.docs-workflow/);
  assert.doesNotMatch(runtimeCss, /\.docs-proof/);
  assert.match(runtimeCss, /\.docs-shell/);
});
