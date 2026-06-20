import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");

const expectedPackages = [
  {
    id: "content/react-markdown",
    slug: "content-react-markdown",
    upstream: "react-markdown",
    version: "10.1.0-dx.2",
    sourceLocator: "components/content/markdown.tsx",
    requiredFiles: [
      "components/content/markdown.tsx",
      "components/content/mdx-provider.tsx",
      "server/content/mdx.ts",
      "lib/markdown-mdx-content/receipt.ts",
    ],
  },
  {
    id: "content/fumadocs-next",
    slug: "content-fumadocs-next",
    upstream: "fumadocs",
    version: "16.8.12-dx.11",
    sourceLocator: "lib/fumadocs/source.ts",
    requiredFiles: [
      "lib/fumadocs/source.ts",
      "lib/fumadocs/search.ts",
      "lib/fumadocs/readiness.ts",
      "app/docs/[[...slug]]/page.tsx",
      "app/docs/readiness/route.ts",
      "content/docs/index.mdx",
    ],
  },
  {
    id: "i18n/next-intl",
    slug: "i18n-next-intl",
    upstream: "next-intl",
    version: "4.12.0-dx.0",
    sourceLocator: "i18n/routing.ts",
    requiredFiles: [
      "i18n/routing.ts",
      "i18n/request.ts",
      "i18n/messages/en.json",
      "i18n/messages/bn.json",
    ],
  },
];

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("content, docs, and i18n are promoted into the lock-backed Forge package set", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  );
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");

  for (const expectedPackage of expectedPackages) {
    const sourcePackage = sourceManifest.packages.find(
      (entry: { package_id: string }) => entry.package_id === expectedPackage.id,
    );
    assert.ok(sourcePackage, `source manifest should expose ${expectedPackage.id}`);
    assert.equal(sourcePackage.upstream_name, expectedPackage.upstream);
    assert.equal(sourcePackage.version, expectedPackage.version);
    assert.ok(sourcePackage.rollback_receipt, `${expectedPackage.id} needs rollback receipt`);

    const packageEntry = lock.packages.find(
      (entry: { name: string }) => entry.name === expectedPackage.id,
    );
    assert.ok(packageEntry, `package lock should expose ${expectedPackage.id}`);
    assert.equal(packageEntry.source_kind, "local-slice");
    assert.equal(packageEntry.source_locator, expectedPackage.sourceLocator);
    assert.equal(packageEntry.integrity_state, "valid");
    assert.equal(packageEntry.files.length, sourcePackage.files.length);
    assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
    assert.ok(packageEntry.rollback_receipt_path);
    assert.ok(packageEntry.safety_archive_receipt_path);

    for (const expectedFile of expectedPackage.requiredFiles) {
      assert.ok(
        packageEntry.files.some((file: { path: string }) => file.path === expectedFile),
        `missing ${expectedPackage.id} locked file ${expectedFile}`,
      );
      assert.ok(
        fs.existsSync(path.join(templateRoot, expectedFile)),
        `missing materialized ${expectedPackage.id} source ${expectedFile}`,
      );
    }

    const packageReceiptPath = packageEntry.receipt_paths.find((candidate: string) =>
      candidate.includes(`receipts/packages/${expectedPackage.slug}.json`),
    );
    assert.ok(packageReceiptPath, `${expectedPackage.id} should reference package-add receipt`);

    const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
    assert.equal(packageReceipt.schema, "forge.package_add_receipt");
    assert.equal(packageReceipt.package.name, expectedPackage.id);
    assert.equal(
      packageReceipt.boundary,
      "forge-owned source slice; no node_modules install performed",
    );
    assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);

    const safetyArchive = readJson(
      `examples/template/${packageEntry.safety_archive_receipt_path}`,
    );
    assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
    assert.equal(safetyArchive.package.name, expectedPackage.id);
    assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
    assert.ok(
      safetyArchive.archive.files.every((file: { cache_path: string }) =>
        fs.existsSync(path.join(templateRoot, file.cache_path)),
      ),
      `${expectedPackage.id} archive should reference existing cache files`,
    );

    assert.ok(status.locked_package_names.includes(expectedPackage.id));
    assert.ok(
      status.cache.manifests.includes(
        `.dx/forge/cache/${expectedPackage.slug}/${expectedPackage.version}/.dx/build-cache/manifest.json`,
      ),
      `package-status cache manifests should include ${expectedPackage.id}`,
    );
  }

  assert.equal(status.package_count, lock.packages.length);
  assert.equal(status.locked_package_count, lock.packages.length);
  assert.equal(
    status.cache.cache_file_count,
    lock.packages.reduce((count: number, entry: { files: unknown[] }) => count + entry.files.length, 0),
  );
  assert.equal(status.safety_archive.rollback_covered_package_count, lock.packages.length);
});

test("default template exposes real content, docs, and i18n App Router surfaces", () => {
  const requiredRoutes = [
    "app/content/page.tsx",
    "app/docs/[[...slug]]/page.tsx",
    "app/i18n/page.tsx",
  ];

  for (const route of requiredRoutes) {
    assert.ok(fs.existsSync(path.join(templateRoot, route)), `missing template route ${route}`);
  }

  assert.match(read("examples/template/app/content/page.tsx"), /TemplateContentPage/);
  assert.match(read("examples/template/app/docs/[[...slug]]/page.tsx"), /source\.getPage/);
  assert.match(read("examples/template/app/i18n/page.tsx"), /TemplateI18nPage/);
  assert.match(
    read("examples/template/components/template-app/dashboard-page.tsx"),
    /ContentDocsI18nPanel/,
  );
  assert.match(
    read("examples/template/components/template-app/template-data.ts"),
    /content\/react-markdown[\s\S]*content\/fumadocs-next[\s\S]*i18n\/next-intl/,
  );
});

test("Markdown content route exposes a source-owned safety audit", () => {
  const markdownSource = read("examples/template/components/content/markdown.tsx");
  const contentRoute = read("examples/template/components/template-app/content-page.tsx");

  assert.match(markdownSource, /export const dxMarkdownSafetyPolicy/);
  assert.match(markdownSource, /export function auditDxMarkdownSource/);
  assert.match(markdownSource, /unsafe-url/);
  assert.match(markdownSource, /raw-html/);
  assert.match(markdownSource, /skipHtml = true/);
  assert.match(markdownSource, /dxSafeMarkdownUrl/);

  assert.match(contentRoute, /auditDxMarkdownSource/);
  assert.match(contentRoute, /dxMarkdownSafetyPolicy/);
  assert.match(contentRoute, /data-dx-markdown-safety-audit="source-owned"/);
  assert.match(contentRoute, /data-dx-markdown-finding-kind=\{finding\.kind\}/);
  assert.match(contentRoute, /Raw HTML/);
  assert.match(contentRoute, /unsafe URL/);
});

test("i18n route exposes source-owned catalog validation", () => {
  const catalogValidation = read("examples/template/i18n/catalog-validation.ts");
  const i18nRoute = read("examples/template/components/template-app/i18n-page.tsx");

  assert.match(catalogValidation, /export function validateDxLaunchCatalogs/);
  assert.match(i18nRoute, /validateDxLaunchCatalogs/);
  assert.match(i18nRoute, /enMessages/);
  assert.match(i18nRoute, /bnMessages/);
  assert.match(i18nRoute, /data-dx-i18n-catalog-validation="source-owned"/);
  assert.match(i18nRoute, /data-dx-i18n-catalog-status=\{/);
  assert.match(i18nRoute, /catalogValidationResults/);
  assert.match(i18nRoute, /Catalog validation/);
});

test("i18n catalog validation route is receipt-hash tracked", () => {
  const helper = read("examples/template/internationalization-receipt-hashes.ts");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
  );
  const readModel = read("examples/template/forge-package-status-read-model.ts");

  assert.match(helper, /CATALOG_VALIDATION_SURFACE_ID/);
  assert.match(helper, /components\/template-app\/i18n-page\.tsx/);
  assert.match(helper, /i18n\/catalog-validation\.ts/);
  assert.ok(
    receipt.files.includes("examples/template/components/template-app/i18n-page.tsx"),
    "i18n receipt should hash-track the template i18n route surface",
  );
  assert.match(readModel, /internationalization-catalog-validation-route/);
});

test("Documentation System uses DX icons instead of a lucide-react package surface", () => {
  const sourcePlugins = read("examples/template/lib/fumadocs/source-plugins.tsx");
  const routeContract = read("examples/template/lib/fumadocs/route-contract.ts");
  const metadata = read("examples/template/lib/fumadocs/metadata.ts");
  const readme = read("examples/template/lib/fumadocs/README.md");
  const docsIndex = read("examples/template/content/docs/index.mdx");

  assert.doesNotMatch(sourcePlugins, /lucideIconsPlugin/);
  assert.doesNotMatch(sourcePlugins, /lucide-react/);
  assert.doesNotMatch(routeContract, /lucide-react/);
  assert.doesNotMatch(metadata, /lucide-react/);
  assert.doesNotMatch(readme, /lucide-react/);
  assert.match(sourcePlugins, /iconPlugin/);
  assert.match(sourcePlugins, /Icon/);
  assert.match(sourcePlugins, /dxFumadocsIconPlugin/);
  assert.match(sourcePlugins, /data-dx-docs-icon-source="dx-icons"/);
  assert.match(docsIndex, /icon: action:rocket/);
});

test("Documentation System exposes an honest source-owned readiness route", () => {
  const readiness = read("examples/template/lib/fumadocs/readiness.ts");
  const route = read("examples/template/app/docs/readiness/route.ts");
  const routeContract = read("examples/template/lib/fumadocs/route-contract.ts");
  const metadata = read("examples/template/lib/fumadocs/metadata.ts");
  const templateData = read("examples/template/components/template-app/template-data.ts");

  assert.match(readiness, /schema: "dx\.fumadocs\.runtime_readiness"/);
  assert.match(readiness, /source-ready/);
  assert.match(readiness, /source-incomplete/);
  assert.match(readiness, /status,/);
  assert.match(readiness, /routeHandlerMaterialized: true/);
  assert.match(readiness, /runtimeExecution: false/);
  assert.match(readiness, /liveRouteExecutionProof: false/);
  assert.doesNotMatch(readiness, /runtimeExecution: true/);
  assert.match(readiness, /liveFumadocsRendererProof: false/);
  assert.match(readiness, /iconSurface: dxFumadocsRouteContract\.sourcePluginIconSurface/);
  assert.match(readiness, /missingRoutes/);
  assert.match(readiness, /missingMaterializedFiles/);
  assert.match(route, /createDxFumadocsReadinessReport/);
  assert.match(route, /data-dx-docs-readiness/);
  assert.match(route, /Response\.json/);
  assert.match(routeContract, /readinessRoute: "\/docs\/readiness"/);
  assert.match(routeContract, /readinessRouteFile: "app\/docs\/readiness\/route\.ts"/);
  assert.match(routeContract, /runtimeStatus: "source-owned-readiness-route"/);
  assert.match(metadata, /docs-runtime-readiness/);
  assert.match(metadata, /lib\/fumadocs\/readiness\.ts/);
  assert.match(metadata, /app\/docs\/readiness\/route\.ts/);
  assert.match(templateData, /\/docs\/readiness/);
  assert.match(templateData, /readiness route/);
});
