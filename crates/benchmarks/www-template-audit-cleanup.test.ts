import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

function withoutScopedPackageScores(source: string) {
  return source.replace(
    /class="forge-reality-score"\s+data-dx-score-scope="package-readiness-row"\s+data-dx-package-score="\d+">(?:\d+)\/100/g,
    'class="forge-reality-score" data-dx-score-scope="package-readiness-row" data-dx-package-score="package-row">package-row-score',
  );
}

function packageIdsFromCatalog() {
  return [
    ...read("examples/template/package-catalog.ts").matchAll(
      /packageId:\s*"([^"]+)"/g,
    ),
  ].map((match) => match[1]);
}

function collectStrings(value: unknown, output: string[] = []) {
  if (typeof value === "string") {
    output.push(value);
    return output;
  }

  if (Array.isArray(value)) {
    for (const item of value) {
      collectStrings(item, output);
    }
    return output;
  }

  if (value && typeof value === "object") {
    for (const item of Object.values(value)) {
      collectStrings(item, output);
    }
  }

  return output;
}

function benchmarkRefsFromReceipt(receipt: unknown) {
  return [
    ...new Set(
      collectStrings(receipt)
        .flatMap((value) => [
          ...value.matchAll(/benchmarks[\\/][^"'\s,]+\.test\.[cm]?[jt]s/g),
        ])
        .map((match) => match[0].replace(/\\/g, "/")),
    ),
  ].sort();
}

function receiptJsonPaths(relativeRoot: string) {
  const absoluteRoot = path.join(root, relativeRoot);
  const pending = [absoluteRoot];
  const receipts: string[] = [];

  while (pending.length > 0) {
    const directory = pending.pop();
    assert.ok(directory, "receipt directory traversal should stay bounded");

    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const absolutePath = path.join(directory, entry.name);

      if (entry.isDirectory()) {
        pending.push(absolutePath);
      } else if (entry.isFile() && entry.name.endsWith(".json")) {
        receipts.push(path.relative(root, absolutePath).replace(/\\/g, "/"));
      }
    }
  }

  return receipts.sort();
}

test("launch template does not expose legacy non-catalog backend package claims", () => {
  const catalogPackageIds = new Set(packageIdsFromCatalog());
  const forbiddenPackageIds = ["backend/convex-compatible", "backend/convex"];
  const checkedSources = [
    "examples/template/template-shell.tsx",
    "examples/template/dx-studio-edit-contract.ts",
    "tools/launch/runtime-template/pages/backend.html",
    "tools/launch/runtime-template/pages/backend.html",
    ".dx/template-app-browser-preview/pages/backend.html",
    "examples/template/public/preview-.dx/build-cache/manifest.json",
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
    "tools/launch/materialize-www-template.ts",
  ];

  for (const forbiddenPackageId of forbiddenPackageIds) {
    assert.equal(
      catalogPackageIds.has(forbiddenPackageId),
      false,
      "legacy backend packages must not be reintroduced into the public catalog",
    );

    for (const sourcePath of checkedSources) {
      assert.doesNotMatch(
        read(sourcePath),
        new RegExp(forbiddenPackageId.replace("/", "\\/")),
        `${sourcePath} should not advertise ${forbiddenPackageId}`,
      );
    }
  }

  for (const manifestPath of [
    "examples/template/public/preview-.dx/build-cache/manifest.json",
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
  ]) {
    const manifest = readJson(manifestPath);
    for (const route of manifest.routes ?? []) {
      for (const packageId of route.forgePackages ?? []) {
        assert.ok(
          catalogPackageIds.has(packageId),
          `${manifestPath} route ${route.route} exposes non-catalog package ${packageId}`,
        );
      }
    }
  }
});

test("conversion proof public routes do not expose stale package or demo claims", () => {
  const catalogPackageIds = new Set(packageIdsFromCatalog());
  const conversionSources = [
    "examples/conversion-proof/pages/index.html",
    "examples/conversion-proof/pages/backend.html",
    "examples/conversion-proof/pages/index.html",
    "examples/conversion-proof/forge/route-discovery/conversion-routes.json",
    "examples/conversion-proof/public/launch-runtime.js",
    "examples/conversion-proof/.dx/vercel-landing/launch-runtime.js",
    "examples/conversion-proof/public/preview-.dx/build-cache/manifest.json",
    "examples/conversion-proof/.dx/vercel-landing/preview-.dx/build-cache/manifest.json",
  ];
  const stalePublicClaims =
    /backend\/convex(?:-compatible)?|icons\/lucide-react|data-dx-[a-z-]*demo|live-demo|local-demo|wasm-bindgen-live-demo|data-dx-wasm-demo-enabled|dxWasmDemoEnabled|local-add-demo|local-demo-ready|no-node-modules-demo|Local demo remains available|Local WebAssembly demo failed|Drizzle query demos|Session demo|No local demo|Demo email|Live connector demo|dxZodDemo/;

  for (const sourcePath of conversionSources) {
    assert.doesNotMatch(
      read(sourcePath),
      stalePublicClaims,
      `${sourcePath} should not expose stale package ids or demo-named launch markers`,
    );
  }

  for (const manifestPath of [
    "examples/conversion-proof/public/preview-.dx/build-cache/manifest.json",
    "examples/conversion-proof/.dx/vercel-landing/preview-.dx/build-cache/manifest.json",
  ]) {
    const manifest = readJson(manifestPath);
    for (const route of manifest.routes ?? []) {
      for (const packageId of route.forgePackages ?? []) {
        assert.ok(
          catalogPackageIds.has(packageId),
          `${manifestPath} route ${route.route} exposes non-catalog package ${packageId}`,
        );
      }
    }
  }

  const routeDiscovery = readJson(
    "examples/conversion-proof/forge/route-discovery/conversion-routes.json",
  );
  for (const route of routeDiscovery.routes ?? []) {
    const selector = route.studio_preview?.package_selector;
    const packageId = selector?.match(/\[data-dx-package="([^"]+)"\]/)?.[1];
    if (packageId) {
      assert.ok(
        catalogPackageIds.has(packageId),
        `route-discovery route ${route.route} exposes non-catalog package selector ${packageId}`,
      );
    }
  }
});

test("conversion proof landing uses preview language instead of public demo copy", () => {
  const sourcePath = "examples/conversion-proof/pages/index.html";
  const source = read(sourcePath);

  assert.doesNotMatch(
    source,
    /\bdemo video\b|\bdemo surface\b|\bwatch demo\b|\bdemo routing\b/i,
    `${sourcePath} should use preview/workflow language instead of public demo copy`,
  );
});

test("package-status catalog count matches the TypeScript launch catalog", () => {
  const catalogPackageIds = packageIdsFromCatalog();
  const uniqueCatalogPackageIds = new Set(catalogPackageIds);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );

  assert.equal(
    catalogPackageIds.length,
    uniqueCatalogPackageIds.size,
    "launch package catalog must not contain duplicate package ids",
  );
  assert.equal(status.catalog_package_count, catalogPackageIds.length);
  assert.match(
    readModel,
    new RegExp(`catalogPackageCount: ${catalogPackageIds.length}`),
  );
});

test("launch template public route package claims use catalog package ids", () => {
  const catalogPackageIds = new Set(packageIdsFromCatalog());

  for (const manifestPath of [
    "examples/template/public/preview-.dx/build-cache/manifest.json",
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
  ]) {
    const manifest = readJson(manifestPath);
    for (const route of manifest.routes ?? []) {
      for (const packageId of route.forgePackages ?? []) {
        assert.ok(
          catalogPackageIds.has(packageId),
          `${manifestPath} route ${route.route} exposes non-catalog package ${packageId}`,
        );
      }
    }
  }

  for (const sourcePath of [
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/ui.html",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/ui.html",
    ".dx/template-app-browser-preview/pages/index.html",
    ".dx/template-app-browser-preview/pages/ui.html",
  ]) {
    assert.doesNotMatch(
      read(sourcePath),
      /data-dx-package="[^"]*(?:www\/template|dx-icons|shadcn\/ui(?=[,\s"]))/,
      `${sourcePath} should not expose legacy aggregate package ids`,
    );
  }
});

test("launch template source metadata does not keep template-only package ids", () => {
  const staleTemplatePackageClaims =
    /(?:packageId:\s*"www\/template"|data-dx-package="(?:www\/template|[^"]*dx-icons)|data-dx-forge="www\/template")/;
  const checkedSources = [
    "examples/template/template-surface-registry.ts",
    "examples/template/dx-studio-edit-contract.ts",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
    "examples/template/public/preview-.dx/build-cache/manifest.json",
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
  ];

  for (const sourcePath of checkedSources) {
    assert.doesNotMatch(
      read(sourcePath),
      staleTemplatePackageClaims,
      `${sourcePath} should not expose template-only package ids as Forge package metadata`,
    );
  }
});

test("InstantDB cursor package uses DX theme tokens instead of hardcoded colors", () => {
  const cursorSources = [
    "examples/template/components/instant/instant-cursors.tsx",
    "examples/template/.dx/forge/cache/instantdb-react/0.0.0-dx.0/components/instant/instant-cursors.tsx",
  ];

  for (const sourcePath of cursorSources) {
    const cursorSource = read(sourcePath);

    assert.doesNotMatch(
      cursorSource,
      /#[0-9a-fA-F]{6}/,
      `${sourcePath} should use DX theme tokens instead of literal colors`,
    );
    assert.match(cursorSource, /var\(--dx-instant-cursor-color/);
    assert.match(cursorSource, /userCursorColor=\{color\}/);
  }
});

test("InstantDB cursor package manifests lock the accepted tokenized source hash", () => {
  const cursorPath = "components/instant/instant-cursors.tsx";
  const expectedHash =
    "e06b3f56fd0cd45c7fc674745eb91c0dc9974ae28738851430a772b3241d81b6";
  const expectedBytes = 733;

  for (const sourcePath of [
    `examples/template/${cursorPath}`,
    `examples/template/.dx/forge/cache/instantdb-react/0.0.0-dx.0/${cursorPath}`,
  ]) {
    assert.equal(
      fs.statSync(path.join(root, sourcePath)).size,
      expectedBytes,
      `${sourcePath} byte length should match the tokenized cursor receipt`,
    );
  }

  const sourcePackage = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  ).packages.find(
    (entry: { package_id?: string }) => entry.package_id === "instantdb/react",
  );
  const sourceFile = sourcePackage.files.find(
    (entry: { path?: string }) => entry.path === cursorPath,
  );
  assert.equal(sourceFile.hash, expectedHash);
  assert.equal(sourceFile.bytes, expectedBytes);

  const lockPackage = readJson(
    "examples/template/.dx/forge/package-lock.json",
  ).packages.find(
    (entry: { name?: string }) => entry.name === "instantdb/react",
  );
  const lockedFile = lockPackage.files.find(
    (entry: { path?: string }) => entry.path === cursorPath,
  );
  assert.equal(lockedFile.content_hash, expectedHash);
  assert.equal(lockedFile.expected_hash, expectedHash);
  assert.equal(lockedFile.bytes, expectedBytes);

  const cacheFile = readJson(
    "examples/template/.dx/forge/cache/instantdb-react/0.0.0-dx.0/.dx/build-cache/manifest.json",
  ).cached_files.find(
    (entry: { path?: string }) => entry.path === cursorPath,
  );
  assert.equal(cacheFile.content_hash, expectedHash);
  assert.equal(cacheFile.bytes, expectedBytes);

  const packageReceipt = readJson(
    "examples/template/.dx/forge/receipts/packages/instantdb-react.json",
  );
  const receiptFile = packageReceipt.package.files.find(
    (entry: { path?: string }) => entry.path === cursorPath,
  );
  const receiptCacheFile = packageReceipt.cache.cached_files.find(
    (entry: { path?: string }) => entry.path === cursorPath,
  );
  assert.equal(receiptFile.content_hash, expectedHash);
  assert.equal(receiptCacheFile.content_hash, expectedHash);
  assert.equal(receiptFile.bytes, expectedBytes);
  assert.equal(receiptCacheFile.bytes, expectedBytes);

  const packageDocs = read("examples/template/.dx/forge/docs/instantdb-react.md");
  assert.match(
    packageDocs,
    new RegExp(`\\| \`${cursorPath.replace("/", "\\/")}\` \\| .* \\| \`${expectedBytes}\` \\| \`${expectedHash}\` \\|`),
  );
});

test("golden-path status uses the current score gate instead of stale green copy", () => {
  const contract = read("examples/template/forge-golden-path-contract.ts");
  const launchPages = [
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ];

  assert.match(contract, /dxCheckScore: 89/);
  assert.match(contract, /dxCheckTraffic: "score-gated"/);
  assert.match(contract, /capped at 89\/100 until browser or live-provider proof/i);
  assert.doesNotMatch(contract, /green no-op|green traffic|87\/100/i);

  for (const sourcePath of launchPages) {
    const source = read(sourcePath);

    assert.match(source, /data-dx-forge-golden-path-dx-check-score="89"/);
    assert.match(source, /data-dx-forge-golden-path-dx-check-traffic="score-gated"/);
    assert.doesNotMatch(
      withoutScopedPackageScores(source),
      /green no-op|green traffic|87\/100/i,
    );
  }
});

test("automation readiness receipt markers do not use public demo naming", () => {
  const sources = [
    "examples/template/automations-status.tsx",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ];

  for (const sourcePath of sources) {
    const source = read(sourcePath);

    assert.match(
      source,
      /data-dx-automation-local-receipt="draft-workflow-receipt"/,
    );
    assert.doesNotMatch(source, /data-dx-automation-local-demo=/);
  }
});

test("static launch pages use readiness field classes instead of demo field classes", () => {
  for (const sourcePath of [
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ]) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /\bauth-demo-field\b/,
      `${sourcePath} should not expose demo-named form field classes`,
    );
    assert.match(
      source,
      /\bauth-readiness-field\b/,
      `${sourcePath} should keep a readiness-named form field class`,
    );
  }
});

test("3D Scene System public markers use readiness naming instead of demo naming", () => {
  const sources = [
    "examples/template/components/scene/launch-scene.tsx",
    "examples/template/launch-scene.tsx",
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/public/launch-runtime.js",
    ".dx/template-app-browser-preview/pages/index.html",
  ];

  for (const sourcePath of sources) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /data-dx-scene-demo-/,
      `${sourcePath} should not expose demo-named 3D scene markers`,
    );
    assert.match(
      source,
      /data-dx-scene-(readiness|workflow)-/,
      `${sourcePath} should expose readiness/workflow-named 3D scene markers`,
    );
  }
});

test("WebAssembly Bridge public markers use readiness naming instead of live demo naming", () => {
  const sources = [
    "examples/template/wasm-interop-status.tsx",
    "examples/template/package-catalog.ts",
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ];

  for (const sourcePath of sources) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /wasm-bindgen-live-demo|data-dx-wasm-demo-enabled|dxWasmDemoEnabled|local-add-demo/,
      `${sourcePath} should not expose WebAssembly demo/live-demo markers`,
    );
    assert.match(
      source,
      /wasm-bindgen-readiness-workflow|data-dx-wasm-local-readiness-enabled|dxWasmLocalReadinessEnabled|local-add-readiness/,
      `${sourcePath} should expose readiness-named WebAssembly markers`,
    );
  }
});

test("WebAssembly Bridge source generators use readiness surface names", () => {
  const sources = [
    "core/src/ecosystem/forge_wasm_bindgen.rs",
    "core/src/ecosystem/dx_check_receipt.rs",
  ];

  for (const sourcePath of sources) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /wasm-bindgen-live-demo|data-dx-component=\\"wasm-bindgen-live-demo\\"|live-demo surfaces/,
      `${sourcePath} should not reintroduce live-demo WebAssembly surface ids`,
    );
    assert.match(
      source,
      /wasm-bindgen-readiness-workflow/,
      `${sourcePath} should use the launch readiness WebAssembly surface id`,
    );
  }
});

test("launch template registry uses readiness language in guard copy", () => {
  const registry = read("examples/template/template-surface-registry.ts");

  assert.doesNotMatch(registry, /unvalidated demo submit/i);
  assert.match(registry, /unvalidated readiness submit/i);
});

test("public package docs use readiness language instead of demo copy", () => {
  const packageDocs = [
    "docs/packages/tanstack-query.md",
    "docs/packages/automations-n8n.md",
  ];

  for (const sourcePath of packageDocs) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /\bdemo controls\b|\bdemo card\b/i,
      `${sourcePath} should not describe launch package surfaces as demos`,
    );
  }

  assert.match(read("docs/packages/tanstack-query.md"), /readiness controls/i);
  assert.match(read("docs/packages/automations-n8n.md"), /product workflow/i);
});

test("launch notes do not preserve stale WebAssembly live-demo identifiers", () => {
  const launchNotes = ["DX.md", "TODO.md", "CHANGELOG.md"];

  for (const sourcePath of launchNotes) {
    const source = read(sourcePath);

    assert.doesNotMatch(
      source,
      /wasm-bindgen-live-demo|dxWasmDemoEnabled|data-dx-wasm-demo-enabled|local-add-demo/,
      `${sourcePath} should not keep stale WebAssembly demo/live-demo identifiers in public launch notes`,
    );
    assert.match(
      source,
      /wasm-bindgen-readiness-workflow|dxWasmLocalReadinessEnabled/,
      `${sourcePath} should keep current WebAssembly readiness identifiers`,
    );
  }
});

test("launch notes do not preserve stale package demo marker identifiers", () => {
  const launchNotes = ["DX.md", "TODO.md", "CHANGELOG.md"];
  const staleMarkerCopy =
    /data-dx-automation-local-demo|auth-demo-field|automations-n8n-live-demo|data-dx-automation-demo|data-dx-auth-local-demo-session|dx-better-auth-demo-session/i;

  for (const sourcePath of launchNotes) {
    assert.doesNotMatch(
      read(sourcePath),
      staleMarkerCopy,
      `${sourcePath} should not keep stale package demo marker identifiers in launch notes`,
    );
  }
});

test("launch notes use www-template wording instead of broad demo copy", () => {
  const launchNotes = ["DX.md", "TODO.md", "CHANGELOG.md"];
  const staleLaunchDemoCopy =
    /default package-demo|package-demo launch|launch demos|launch demo source coding|launch demo is strong/i;

  for (const sourcePath of launchNotes) {
    assert.doesNotMatch(
      read(sourcePath),
      staleLaunchDemoCopy,
      `${sourcePath} should use www-template or workflow language instead of broad demo copy`,
    );
  }
});

test("launch notes do not preserve stale green dx-check score copy", () => {
  const launchNotes = ["DX.md", "TODO.md", "CHANGELOG.md"];

  for (const sourcePath of launchNotes) {
    assert.doesNotMatch(
      read(sourcePath),
      /green no-op|green traffic|87\/100 green|dx-check`\s*87\/100 green/i,
      `${sourcePath} should not preserve stale green dx-check launch-score copy`,
    );
  }
});

test("package workflow receipts use readiness vocabulary for local interactions", () => {
  const receiptDirs = [
    "examples/template/.dx/forge/receipts",
    ".dx/template-app-browser-preview/.dx/forge/receipts",
  ];

  for (const receiptDir of receiptDirs) {
    const fullDir = path.join(root, receiptDir);
    const receiptFiles = fs
      .readdirSync(fullDir, { withFileTypes: true })
      .filter((entry) => entry.isFile() && entry.name.endsWith(".json"))
      .map((entry) => path.join(receiptDir, entry.name));

    for (const receiptPath of receiptFiles) {
      assert.doesNotMatch(
        read(receiptPath),
        /local_demo_interactions|data-dx-[a-z-]*demo/,
        `${receiptPath} should use readiness/workflow vocabulary in package receipt proof`,
      );
    }
  }
});

test("package workflow receipts reference existing benchmark guards", () => {
  const receiptPaths = [
    ...receiptJsonPaths("examples/template/.dx/forge/receipts"),
    ...receiptJsonPaths(".dx/template-app-browser-preview/.dx/forge/receipts"),
  ];

  for (const receiptPath of receiptPaths) {
    const receipt = readJson(receiptPath);
    const strings = collectStrings(receipt);
    const refs = benchmarkRefsFromReceipt(receipt);

    for (const value of strings) {
      assert.doesNotMatch(
        value,
        /\.test\.cjs\b/,
        `${receiptPath} references a stale .test.cjs guard`,
      );
    }

    for (const ref of refs) {
      assert.ok(
        fs.existsSync(path.join(root, ref)),
        `${receiptPath} references missing benchmark guard ${ref}`,
      );
    }
  }
});

test("package-status execution guards reference existing benchmark guards", () => {
  const statusPath = "examples/template/.dx/forge/package-status.json";
  const status = readJson(statusPath);
  const strings = collectStrings(status);
  const refs = benchmarkRefsFromReceipt(status);

  for (const value of strings) {
    assert.doesNotMatch(
      value,
      /\.test\.cjs\b/,
      `${statusPath} references a stale .test.cjs guard`,
    );
  }

  for (const ref of refs) {
    assert.ok(
      fs.existsSync(path.join(root, ref)),
      `${statusPath} references missing benchmark guard ${ref}`,
    );
  }
});

test("launch template does not keep nested generated package source copies", () => {
  const nestedRoot = path.join(
    root,
    "examples/template/examples/template",
  );
  const copiedSources = fs.existsSync(nestedRoot)
    ? fs
        .readdirSync(nestedRoot, { withFileTypes: true })
        .filter((entry) => entry.isFile())
        .map((entry) => entry.name)
        .sort()
    : [];

  assert.deepEqual(
    copiedSources,
    [],
    "examples/template/examples/template should not contain duplicated package sources",
  );
});
