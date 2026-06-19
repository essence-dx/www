const assert = require("node:assert");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");
const { pathToFileURL } = require("node:url");

const root = path.resolve(__dirname, "..");
const inspirationRoot = "G:\\WWW\\inspirations";

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readInspiration(relativePath) {
  const filePath = path.join(inspirationRoot, relativePath);
  assert.ok(fs.existsSync(filePath), `missing upstream ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function extractRustRawString(source, constName) {
  const marker = `const ${constName}: &str = r#"`;
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `missing Rust raw string ${constName}`);
  const bodyStart = start + marker.length;
  const end = source.indexOf('"#;', bodyStart);
  assert.notEqual(end, -1, `unterminated Rust raw string ${constName}`);
  return source.slice(bodyStart, end);
}

async function importMaterializedTypescript(source, filename) {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-mdx-receipt-"));
  const modulePath = path.join(tempDir, filename);
  fs.writeFileSync(modulePath, source, "utf8");

  try {
    return await import(`${pathToFileURL(modulePath).href}?t=${Date.now()}`);
  } finally {
    fs.unlinkSync(modulePath);
    fs.rmdirSync(tempDir);
  }
}

test("Markdown & MDX Content lane is grounded in the assigned upstream mirrors", () => {
  const reactMarkdownIndex = readInspiration("react-markdown/index.js");
  const mdxCompilerIndex = readInspiration("mdx/packages/mdx/index.js");
  const mdxReactIndex = readInspiration("mdx/packages/react/index.js");

  assert.match(reactMarkdownIndex, /MarkdownAsync/);
  assert.match(reactMarkdownIndex, /MarkdownHooks/);
  assert.match(reactMarkdownIndex, /defaultUrlTransform/);

  assert.match(mdxCompilerIndex, /export \{compile, compileSync\}/);
  assert.match(mdxCompilerIndex, /createProcessor/);
  assert.match(mdxCompilerIndex, /nodeTypes/);

  assert.match(mdxReactIndex, /MDXProvider/);
  assert.match(mdxReactIndex, /useMDXComponents/);
});

test("Forge materializes official Markdown & MDX Content metadata and selected MDX surfaces", () => {
  const forge = read("core/src/ecosystem/forge_react_markdown.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");

  assert.match(forge, /officialDxPackageName: "Markdown & MDX Content"/);
  assert.match(forge, /upstreamPackages: \[/);
  assert.match(forge, /"react-markdown"/);
  assert.match(forge, /"@mdx-js\/mdx"/);
  assert.match(forge, /"@mdx-js\/react"/);
  assert.match(forge, /"js\/components\/content\/mdx-provider\.tsx"/);
  assert.match(forge, /"js\/server\/content\/mdx\.ts"/);
  assert.match(forge, /"js\/lib\/mdx\/metadata\.ts"/);
  assert.match(forge, /from "@mdx-js\/react"/);
  assert.match(forge, /from "@mdx-js\/mdx"/);
  assert.match(forge, /MDXProvider/);
  assert.match(forge, /useMDXComponents/);
  assert.match(forge, /compileDxMdxContent/);
  assert.match(forge, /createDxMdxProcessor/);
  assert.match(forge, /dxCheckVisibility/);
  assert.match(forge, /"present"/);
  assert.match(forge, /"stale"/);
  assert.match(forge, /"missing-receipt"/);
  assert.match(forge, /"blocked"/);
  assert.match(forge, /"unsupported-surface"/);
  assert.match(forge, /data-dx-component="dx-mdx-provider"/);
  assert.match(forge, /data-dx-style-surface="markdown-mdx-content"/);
  assert.match(forge, /data-dx-zed-surface="content-mdx-provider"/);

  assert.match(registry, /"markdown-mdx-content" => "content\/react-markdown"/);
  assert.match(registry, /"mdx\/content" => "content\/react-markdown"/);
  assert.match(registry, /@mdx-js\/mdx@3\.1\.1/);
});

test("launch catalog and docs use the official lane name while keeping upstream as provenance", () => {
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/content-react-markdown.md");

  assert.match(catalog, /officialName: "Markdown & MDX Content"/);
  assert.match(catalog, /officialPackageName: "Markdown & MDX Content"/);
  assert.match(catalog, /command: "dx add markdown-mdx-content --write"/);
  assert.match(catalog, /upstreamPackage: "react-markdown; @mdx-js\/mdx; @mdx-js\/react"/);
  assert.match(catalog, /sourceMirror: "G:\/WWW\/inspirations\/react-markdown; G:\/WWW\/inspirations\/mdx"/);
  assert.match(catalog, /"components\/content\/mdx-provider\.tsx"/);
  assert.match(catalog, /"server\/content\/mdx\.ts"/);
  assert.match(catalog, /name: "Markdown & MDX Content"/);
  assert.doesNotMatch(catalog, /name: "React Markdown Preview"/);

  assert.match(docs, /^# Markdown & MDX Content/m);
  assert.match(docs, /Official DX package name: `Markdown & MDX Content`/);
  assert.match(docs, /Upstream package: `react-markdown`; `@mdx-js\/mdx`; `@mdx-js\/react`/);
  assert.match(docs, /`MarkdownAsync`/);
  assert.match(docs, /`MDXProvider`/);
  assert.match(docs, /`compile`/);
  assert.match(docs, /`components\/content\/mdx-provider\.tsx`/);
  assert.match(docs, /`server\/content\/mdx\.ts`/);
  assert.match(docs, /dx-check visibility/);
  assert.match(docs, /present, stale, missing receipt, blocked, and unsupported surface/);
  assert.match(docs, /Honesty label: `SOURCE-ONLY`/);
  assert.match(docs, /dx run --test \.\\benchmarks\\markdown-mdx-content-slice\.test\.ts/);
});

test("Markdown & MDX Content exposes a typed Forge receipt helper", () => {
  const forge = read("core/src/ecosystem/forge_react_markdown.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/content-react-markdown.md");

  assert.match(forge, /"js\/lib\/markdown-mdx-content\/receipt\.ts"/);
  assert.match(forge, /MarkdownMdxContentReceiptStatus/);
  assert.match(forge, /MarkdownMdxContentReceiptFile/);
  assert.match(forge, /createMarkdownMdxContentReceipt/);
  assert.match(forge, /schema: "dx\.forge\.markdown_mdx_content_receipt"/);
  assert.match(forge, /hashes: \{/);
  assert.match(forge, /sha256\?: string/);
  assert.match(forge, /blake3\?: string/);
  assert.match(forge, /upstreamProvenance/);
  assert.match(forge, /requiredEnv/);
  assert.match(forge, /appOwnedBoundaries/);
  assert.match(forge, /runtimeLimitations/);
  assert.match(forge, /MarkdownMdxContentReceiptMaterializedSource/);
  assert.match(forge, /markdownMdxContentReceiptMaterializedSource/);
  assert.match(forge, /materializedSource: input\.materializedSource \?\?/);
  assert.match(forge, /MarkdownMdxContentDxStyleCompatibility/);
  assert.match(
    forge,
    /dxStyleCompatibility\?: MarkdownMdxContentDxStyleCompatibility/,
  );
  assert.match(forge, /markdownMdxContentDxStyleCompatibility/);
  assert.match(
    forge,
    /dxStyleCompatibility: input\.dxStyleCompatibility \?\?/,
  );
  assert.match(forge, /schema: "dx\.forge\.package\.dx_style_compatibility"/);
  assert.match(forge, /data-dx-style-surface="markdown-mdx-content"/);
  assert.match(forge, /honestyLabel: "SOURCE-ONLY"/);

  assert.match(catalog, /"lib\/markdown-mdx-content\/receipt\.ts"/);
  assert.match(catalog, /"forge-receipt-helper"/);
  assert.match(catalog, /"createMarkdownMdxContentReceipt"/);

  assert.match(docs, /## Forge Receipt Shape/);
  assert.match(docs, /`createMarkdownMdxContentReceipt`/);
  assert.match(docs, /`dx\.forge\.markdown_mdx_content_receipt`/);
  assert.match(docs, /file hashes/);
  assert.match(docs, /`materializedSource`/);
  assert.match(docs, /`dx\.forge\.package\.materialized_source`/);
  assert.match(docs, /`dxStyleCompatibility`/);
  assert.match(docs, /`dx\.forge\.package\.dx_style_compatibility`/);
  assert.match(docs, /runtime limitations/);
});

test("generated starter can execute the materialized Markdown & MDX Content receipt helper", async () => {
  const forge = read("core/src/ecosystem/forge_react_markdown.rs");
  const receiptSource = extractRustRawString(forge, "MARKDOWN_MDX_RECEIPT_TS");
  const receiptModule = await importMaterializedTypescript(
    receiptSource,
    "content-react-markdown-receipt.ts",
  );
  const receipt = receiptModule.createMarkdownMdxContentReceipt({
    selectedSurfaces: ["mdx-provider", "forge-receipt-helper"],
    files: [
      {
        path: "components/content/mdx-provider.tsx",
        surface: "mdx-provider",
        hashes: {
          sha256: "f".repeat(64),
        },
        provenance: {
          upstreamPackage: "@mdx-js/react",
          sourceMirror: "G:/WWW/inspirations/mdx/packages/react",
          inspectedSourceFile: "G:/WWW/inspirations/mdx/packages/react/index.js",
          upstreamApi: "MDXProvider",
        },
      },
    ],
    generatedAt: "2026-05-22T00:00:00.000Z",
  });

  assert.equal(receipt.schema, "dx.forge.markdown_mdx_content_receipt");
  assert.equal(receipt.officialDxPackageName, "Markdown & MDX Content");
  assert.equal(receipt.package.packageId, "content/react-markdown");
  assert.equal(receipt.honestyLabel, "SOURCE-ONLY");
  assert.equal(
    receipt.materializedSource.schema,
    "dx.forge.package.materialized_source",
  );
  assert.equal(
    receipt.materializedSource.sourceFile,
    "lib/markdown-mdx-content/receipt.ts",
  );
  assert.equal(receipt.materializedSource.surface, "forge-receipt-helper");
  assert.equal(receipt.materializedSource.runtimeProof, false);
  assert.match(
    receipt.materializedSource.executionGuard,
    /markdown-mdx-content-slice\.test\.ts/,
  );
  assert.equal(receipt.dxCheckVisibility.status, "present");
  assert.equal(receipt.files[0].hashes.sha256, "f".repeat(64));
  assert.equal(
    receipt.dxStyleCompatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dxStyleCompatibility.status, "present");
  assert.equal(receipt.dxStyleCompatibility.runtimeProof, false);
  assert.ok(receipt.dxStyleCompatibility.visibleSurfaces.includes("mdx-provider"));
  assert.ok(
    receipt.dxStyleCompatibility.sourceFiles.includes(
      "components/content/mdx-provider.tsx",
    ),
  );
  assert.ok(
    receipt.dxStyleCompatibility.dataDxMarkers.includes(
      'data-dx-style-surface="markdown-mdx-content"',
    ),
  );
});

test("Markdown & MDX Content is visible in the package-status read model with current package receipts", () => {
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageStatus = read("examples/template/forge-package-status.ts");
  const packageStatusJson = readJson("examples/template/.dx/forge/package-status.json");
  const catalog = read("examples/template/package-catalog.ts");
  const docs = read("docs/packages/content-react-markdown.md");

  assert.match(readModel, /export const markdownMdxContentPackageVisibility = \{/);
  assert.match(readModel, /officialName: "Markdown & MDX Content"/);
  assert.match(readModel, /packageId: "content\/react-markdown"/);
  assert.match(readModel, /receiptStatus: "present"/);
  assert.match(readModel, /packageReceiptPath:\s*"\.dx\/forge\/receipts\/packages\/content-react-markdown\.json"/);
  assert.match(readModel, /surfaceId: "safe-markdown-renderer"/);
  assert.match(readModel, /surfaceId: "mdx-provider"/);
  assert.match(readModel, /surfaceId: "mdx-server-compile"/);
  assert.match(readModel, /surfaceId: "forge-receipt-helper"/);
  assert.match(readModel, /markdown_mdx_content_receipt_present/);
  assert.match(readModel, /markdown_mdx_content_missing_receipt/);
  assert.match(readModel, /data-dx-zed-surface="content-mdx-provider"/);
  assert.match(readModel, /markdown-mdx-content:receipt-hash-refresh/);
  assert.match(readModel, /materializedSource: \{/);
  assert.match(readModel, /schema: "dx\.forge\.package\.materialized_source"/);
  assert.match(readModel, /sourceFile: "lib\/markdown-mdx-content\/receipt\.ts"/);
  assert.match(readModel, /materializedFile: "lib\/markdown-mdx-content\/receipt\.ts"/);
  assert.match(readModel, /surface: "forge-receipt-helper"/);
  assert.match(
    readModel,
    /executionGuard: "dx run --test \.\\\\benchmarks\\\\markdown-mdx-content-slice\.test\.ts"/,
  );
  assert.match(readModel, /runtimeProof: false/);

  const visibility = packageStatusJson.package_lane_visibility.find(
    (entry) => entry.package_id === "content/react-markdown",
  );
  assert.ok(visibility, "Markdown & MDX Content package-status row is missing");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(
    visibility.package_receipt_path,
    ".dx/forge/receipts/packages/content-react-markdown.json",
  );
  assert.ok(
    fs.existsSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/packages/content-react-markdown.json",
      ),
    ),
    "Markdown & MDX Content package receipt should be checked in",
  );
  assert.equal(
    visibility.materialized_source.schema,
    "dx.forge.package.materialized_source",
  );
  assert.equal(visibility.materialized_source.source_file, "lib/markdown-mdx-content/receipt.ts");
  assert.equal(
    visibility.materialized_source.materialized_file,
    "lib/markdown-mdx-content/receipt.ts",
  );
  assert.equal(visibility.materialized_source.surface, "forge-receipt-helper");
  assert.equal(visibility.materialized_source.runtime_proof, false);
  assert.match(
    visibility.materialized_source.execution_guard,
    /markdown-mdx-content-slice\.test\.ts/,
  );
  for (const metric of [
    "markdown_mdx_content_materialized_source_present",
    "markdown_mdx_content_materialized_source_missing",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Markdown & MDX Content visibility row`,
    );
    assert.ok(
      packageStatusJson.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(packageStatus, /markdownMdxContentPackageVisibility/);
  assert.match(packageStatus, /markdownMdxContentVisibility: markdownMdxContentPackageVisibility/);

  assert.match(
    catalog,
    /packageId: "content\/react-markdown"[\s\S]*?dxCheckVisibility: \{[\s\S]*?currentStatus: "present"/,
  );
  assert.match(catalog, /"\.dx\/forge\/receipts\/packages\/content-react-markdown\.json"/);
  assert.match(docs, /## Package Status Read Model/);
  assert.match(docs, /The launch template now reports `present`/);
  assert.match(docs, /`materializedSource` package-status and read-model mirror/);
  assert.match(docs, /markdown_mdx_content_materialized_source_present/);
  assert.match(docs, /markdown-mdx-content-missing-materialized-source/);
});

test("Markdown & MDX Content dx-style compatibility is package-status and dx-check visible", () => {
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageStatus = readJson("examples/template/.dx/forge/package-status.json");
  const checker = read("core/src/ecosystem/project_check/markdown_mdx_content_dx_check.rs");
  const docs = read("docs/packages/content-react-markdown.md");

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "content/react-markdown",
  );
  assert.ok(visibility, "Markdown & MDX Content package-status row is missing");
  assert.equal(
    visibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(visibility.dx_style_compatibility.status, "present");
  assert.equal(visibility.dx_style_compatibility.token_source, "styles/globals.css");
  assert.equal(visibility.dx_style_compatibility.generated_css, "styles/globals.css");
  assert.equal(visibility.dx_style_compatibility.runtime_proof, false);
  assert.ok(visibility.dx_style_compatibility.visible_surfaces.includes("mdx-provider"));
  assert.ok(
    visibility.dx_style_compatibility.source_files.includes(
      "components/content/mdx-provider.tsx",
    ),
  );
  assert.ok(
    visibility.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="markdown-mdx-content"',
    ),
  );

  for (const metric of [
    "markdown_mdx_content_dx_style_compatibility_present",
    "markdown_mdx_content_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Markdown & MDX Content visibility row`,
    );
    assert.ok(
      packageStatus.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(checker, new RegExp(metric));
  }

  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(readModel, /visibleSurfaces: \["mdx-provider"\]/);
  assert.match(checker, /dx_style_compatibility_is_present/);
  assert.match(checker, /markdown-mdx-content-missing-dx-style-compatibility/);
  assert.match(docs, /markdown_mdx_content_dx_style_compatibility_present/);
  assert.match(docs, /markdown-mdx-content-missing-dx-style-compatibility/);
});

test("Markdown & MDX Content has a Rust dx-check package-status emitter", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const checker = read("core/src/ecosystem/project_check/markdown_mdx_content_dx_check.rs");
  const docs = read("docs/packages/content-react-markdown.md");

  assert.match(checker, /use std::\{\s*fs,\s*path::\{Path, PathBuf\},\s*\};/);
  assert.match(checker, /use sha2::\{Digest, Sha256\};/);
  assert.match(projectCheck, /mod markdown_mdx_content_dx_check;/);
  assert.match(projectCheck, /forge_markdown_mdx_content_package_metrics/);
  assert.match(projectCheck, /dx_check_reports_markdown_mdx_content_package_status_visibility/);

  assert.match(checker, /MARKDOWN_MDX_CONTENT_PACKAGE_ID: &str = "content\/react-markdown"/);
  assert.match(checker, /MARKDOWN_MDX_CONTENT_OFFICIAL_NAME: &str = "Markdown & MDX Content"/);
  assert.match(checker, /MARKDOWN_MDX_CONTENT_PACKAGE_RECEIPT: &str =\s*"\.dx\/forge\/receipts\/packages\/content-react-markdown\.json"/);
  assert.match(checker, /markdown_mdx_content_package_present/);
  assert.match(checker, /markdown_mdx_content_receipt_present/);
  assert.match(checker, /markdown_mdx_content_receipt_stale/);
  assert.match(checker, /markdown_mdx_content_missing_receipt/);
  assert.match(checker, /markdown_mdx_content_blocked_surface/);
  assert.match(checker, /markdown_mdx_content_unsupported_surface/);
  assert.match(checker, /markdown_mdx_content_hash_manifest_present/);
  assert.match(checker, /markdown_mdx_content_hash_mismatch/);
  assert.match(checker, /markdown-mdx-content-missing-receipt/);
  assert.match(checker, /markdown-mdx-content-hash-mismatch/);
  assert.match(checker, /count_receipt_hash_mismatches/);
  assert.match(checker, /count_surface_hash_mismatches/);
  assert.match(checker, /sha256_project_file/);
  assert.match(checker, /format!\("\{digest:x\}"\)/);
  assert.match(checker, /markdown_mdx_content_hash_mismatches_are_byte_derived/);
  assert.match(checker, /markdown_mdx_content_package_metrics_reports_hash_mismatch_metric_and_finding/);
  assert.match(checker, /markdown_mdx_content_materialized_source_present/);
  assert.match(checker, /markdown_mdx_content_materialized_source_missing/);
  assert.match(checker, /markdown-mdx-content-missing-materialized-source/);
  assert.match(checker, /materialized_source_is_present/);
  assert.match(checker, /missing-receipt" \| "missing receipt"/);

  assert.match(docs, /## Rust dx-check Emitter/);
  assert.match(docs, /`markdown_mdx_content_package_present`/);
  assert.match(docs, /`markdown_mdx_content_hash_manifest_present`/);
  assert.match(docs, /`markdown_mdx_content_materialized_source_present`/);
  assert.match(docs, /`markdown-mdx-content-hash-mismatch`/);
  assert.match(docs, /`markdown-mdx-content-missing-materialized-source`/);
  assert.match(docs, /`markdown-mdx-content-missing-receipt`/);
});

test("Markdown & MDX Content check-panel row surfaces materialized-source visibility", () => {
  const receiptReader = read("core/src/ecosystem/dx_check_receipt.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const docs = read("docs/packages/content-react-markdown.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const runbookFixturePath =
    "docs/packages/content-react-markdown.source-guard-runbook.json";
  const runbookFixture = readJson(runbookFixturePath);

  assert.match(receiptReader, /MARKDOWN_MDX_CONTENT_PACKAGE_ID: &str = "content\/react-markdown"/);
  assert.match(receiptReader, /MARKDOWN_MDX_CONTENT_OFFICIAL_NAME: &str = "Markdown & MDX Content"/);
  assert.match(receiptReader, /rows\.extend\(markdown_mdx_content_package_lane_row\(root\)\)/);
  assert.match(receiptReader, /fn markdown_mdx_content_package_lane_row\(root: &Path\)/);
  assert.match(receiptReader, /markdown_mdx_content_materialized_source_present/);
  assert.match(receiptReader, /markdown_mdx_content_materialized_source_missing/);
  assert.match(receiptReader, /markdown_mdx_content_materialized_source_is_present/);
  assert.match(receiptReader, /markdown_mdx_content_next_action/);
  assert.match(
    receiptReader,
    /dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row/,
  );
  assert.match(receiptReader, /write_markdown_mdx_content_package_status/);
  assert.match(
    receiptReader,
    /Regenerate the Markdown & MDX Content materializedSource row/,
  );

  assert.match(docs, /DX Studio\/check-panel package row/);
  assert.match(docs, /materialized-source present\/missing metrics/);
  assert.match(
    docs,
    /cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib/,
  );

  assert.match(studioManifest, /markdown-mdx-content-materialized-source-fixture/);
  assert.match(studioManifest, /content\/react-markdown source-only Studio discovery/);
  assert.match(
    studioManifest,
    /cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib/,
  );
  assert.match(studioManifest, /markdown_mdx_content_materialized_source_present/);
  assert.match(studioManifest, /markdown_mdx_content_materialized_source_missing/);
  assert.match(
    frameworkDocs,
    /markdown-mdx-content-materialized-source-fixture/,
  );
  assert.match(studioManifest, /source_guard_fixture_paths_for_route/);
  assert.match(studioManifest, /source_guard_contract_with_fixture/);
  assert.match(studioManifest, /source_guard_command_with_fixture/);
  assert.match(studioManifest, /docs\/packages\/content-react-markdown\.source-guard-runbook\.json/);
  assert.match(docs, /content-react-markdown\.source-guard-runbook\.json/);
  assert.match(frameworkDocs, /content-react-markdown\.source-guard-runbook\.json/);

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(
    runbookFixture.package.official_package_name,
    "Markdown & MDX Content",
  );
  assert.equal(runbookFixture.package.package_id, "content/react-markdown");
  assert.equal(
    runbookFixture.package.upstream_package,
    "react-markdown; @mdx-js/mdx; @mdx-js/react",
  );
  assert.equal(
    runbookFixture.package.upstream_version,
    "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1",
  );
  assert.deepEqual(runbookFixture.package.source_mirrors, [
    "G:/WWW/inspirations/react-markdown",
    "G:/WWW/inspirations/mdx",
  ]);

  assert.equal(
    runbookFixture.guard.id,
    "markdown-mdx-content-materialized-source-fixture",
  );
  assert.deepEqual(runbookFixture.guard.routes, ["/"]);
  assert.equal(
    runbookFixture.guard.guard_file,
    "core/src/ecosystem/dx_check_receipt.rs",
  );
  assert.equal(
    runbookFixture.guard.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
  );
  assert.equal(runbookFixture.guard.fixture_path, runbookFixturePath);
  assert.ok(
    runbookFixture.guard.proves.includes(
      "docs/packages/content-react-markdown.source-guard-runbook.json",
    ),
    "runbook fixture should be listed in its own guard proof",
  );
  assert.ok(
    runbookFixture.guard.proves.includes(
      "content/react-markdown source-only Studio discovery",
    ),
  );

  const sourceOnlyFlags = [
    "writes_files",
    "starts_server",
    "runs_package_install",
    "runs_full_build",
    "node_modules_required",
  ];
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  for (const flag of sourceOnlyFlags) {
    assert.equal(runbookFixture.guard[flag], false, `guard ${flag}`);
  }

  const generatedRunbook = runbookFixture.source_guard_runbook_index;
  assert.equal(generatedRunbook.index_field, "source_guard_runbook_index");
  assert.equal(generatedRunbook.route, "/");
  assert.ok(
    generatedRunbook.fixture_paths.some(
      (entry) =>
        entry.source_guard_id === "markdown-mdx-content-materialized-source-fixture" &&
        entry.package_id === "content/react-markdown" &&
        entry.fixture_path === runbookFixturePath &&
        entry.schema === "dx.forge.package.source_guard_runbook_fixture",
    ),
    "source_guard_runbook_index fixture_paths should expose materialized-source fixture",
  );
  assert.ok(
    generatedRunbook.source_guard_ids.includes(
      "markdown-mdx-content-materialized-source-fixture",
    ),
  );
  assert.equal(
    generatedRunbook.contract.id,
    "markdown-mdx-content-materialized-source-fixture",
  );
  assert.equal(
    generatedRunbook.contract.evidence_field,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
  );
  assert.equal(generatedRunbook.contract.fixture_path, runbookFixturePath);
  assert.equal(generatedRunbook.contract.scope, "source-only");
  assert.equal(generatedRunbook.contract.reads_runtime_artifacts, false);
  assert.equal(
    generatedRunbook.command.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
  );
  assert.match(
    generatedRunbook.command.purpose,
    /without live renderer proof/,
  );
  assert.equal(generatedRunbook.command.fixture_path, runbookFixturePath);
  assert.equal(generatedRunbook.execution_policy, "source-only");
  for (const flag of [
    "requires_server",
    "requires_package_install",
    "requires_full_build",
    "writes_files",
    "node_modules_required",
  ]) {
    assert.equal(generatedRunbook[flag], false, `runbook ${flag}`);
  }

  assert.ok(
    runbookFixture.provenance.inspected_source_files.includes(
      "G:/WWW/inspirations/react-markdown/index.js",
    ),
  );
  assert.ok(
    runbookFixture.provenance.inspected_source_files.includes(
      "G:/WWW/inspirations/mdx/packages/mdx/index.js",
    ),
  );
  assert.ok(
    runbookFixture.provenance.inspected_source_files.includes(
      "G:/WWW/inspirations/mdx/packages/react/index.js",
    ),
  );
  for (const upstreamApi of [
    "Markdown",
    "MarkdownAsync",
    "MarkdownHooks",
    "defaultUrlTransform",
    "compile",
    "compileSync",
    "createProcessor",
    "nodeTypes",
    "MDXProvider",
    "useMDXComponents",
  ]) {
    assert.ok(
      runbookFixture.provenance.upstream_public_apis.includes(upstreamApi),
      `${upstreamApi} missing from fixture provenance`,
    );
  }
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("live Markdown/MDX renderer proof"),
    ),
  );
});

test("Markdown & MDX Content publishes helper-freshness runbook metadata for Studio", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const docs = read("docs/packages/content-react-markdown.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const runbookFixturePath =
    "docs/packages/content-react-markdown.source-guard-runbook.json";
  const runbookFixture = readJson(runbookFixturePath);
  const sourceGuardId = "markdown-mdx-content-check-panel-helper-freshness";
  const command =
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib";
  const metrics = [
    "markdown_mdx_content_receipt_hash_refresh_current",
    "markdown_mdx_content_receipt_hash_refresh_stale",
    "markdown_mdx_content_receipt_hash_refresh_missing",
    "markdown_mdx_content_hash_mismatch",
  ];

  assert.match(studioManifest, new RegExp(sourceGuardId));
  assert.match(studioManifest, new RegExp(command.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(studioManifest, /source_guard_fixture_paths_for_route/);
  assert.match(studioManifest, /source_guard_contract_with_fixture/);
  assert.match(studioManifest, /source_guard_command_with_fixture/);
  assert.match(studioManifest, /docs\/packages\/content-react-markdown\.source-guard-runbook\.json/);

  const helperGuard = runbookFixture.helper_freshness_guard;
  assert.ok(helperGuard, "helper_freshness_guard is missing from runbook fixture");
  assert.equal(helperGuard.id, sourceGuardId);
  assert.equal(helperGuard.guard_file, "core/src/ecosystem/dx_check_receipt.rs");
  assert.equal(helperGuard.command, command);
  assert.equal(helperGuard.fixture_path, runbookFixturePath);
  assert.equal(helperGuard.execution_policy, "source-only");
  for (const flag of [
    "writes_files",
    "starts_server",
    "runs_package_install",
    "runs_full_build",
    "node_modules_required",
  ]) {
    assert.equal(helperGuard[flag], false, `helper guard ${flag}`);
  }

  const generatedRunbook = runbookFixture.source_guard_runbook_index;
  assert.ok(
    generatedRunbook.fixture_paths.some(
      (entry) =>
        entry.source_guard_id === sourceGuardId &&
        entry.package_id === "content/react-markdown" &&
        entry.fixture_path === runbookFixturePath,
    ),
    "source_guard_runbook_index fixture_paths should expose helper freshness",
  );
  assert.ok(generatedRunbook.source_guard_ids.includes(sourceGuardId));
  assert.ok(
    generatedRunbook.contracts.some(
      (entry) =>
        entry.id === sourceGuardId &&
        entry.evidence_field === command &&
        entry.fixture_path === runbookFixturePath &&
        entry.scope === "source-only" &&
        entry.reads_runtime_artifacts === false &&
        entry.writes_files === false &&
        entry.node_modules_required === false,
    ),
    "source_guard_runbook_index contracts should expose helper freshness",
  );
  assert.ok(
    generatedRunbook.commands.some(
      (entry) =>
        entry.command === command &&
        entry.fixture_path === runbookFixturePath &&
        entry.starts_server === false &&
        entry.runs_package_install === false &&
        entry.runs_full_build === false &&
        entry.writes_files === false &&
        entry.node_modules_required === false,
    ),
    "source_guard_runbook_index commands should expose helper freshness",
  );

  for (const metric of metrics) {
    assert.ok(
      helperGuard.proves.includes(metric),
      `${metric} missing from helper freshness proof list`,
    );
    assert.ok(
      runbookFixture.dx_check_metrics.includes(metric),
      `${metric} missing from runbook dx_check_metrics`,
    );
    assert.match(docs, new RegExp(metric));
    assert.match(frameworkDocs, new RegExp(metric));
  }

  assert.ok(
    helperGuard.proves.includes("markdown_mdx_content_hash_mismatch stays byte-derived"),
  );
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("live Markdown/MDX renderer proof"),
    ),
  );
});
