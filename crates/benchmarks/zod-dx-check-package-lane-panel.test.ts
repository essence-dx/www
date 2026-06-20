const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/zod/packages/zod");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

function sourceEditSurface(source, id) {
  const marker = `id: "${id}"`;
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `expected source edit surface ${id}`);
  const rest = source.slice(start);
  const endMatch = rest.match(/\r?\n  },\r?\n  \{/);
  const end = endMatch ? endMatch.index : -1;
  assert.notEqual(end, -1, `expected end of source edit surface ${id}`);
  return rest.slice(0, end);
}

function rustFunction(source, name) {
  const marker = `fn ${name}(`;
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `expected Rust function ${name}`);
  const rest = source.slice(start);
  const end = rest.indexOf("\n}\n\nfn ");
  assert.notEqual(end, -1, `expected end of Rust function ${name}`);
  return rest.slice(0, end + 2);
}

test("Validation & Schemas package-lane row exposes hash and helper freshness in the DX check panel", () => {
  const upstreamPackage = JSON.parse(readUpstream("package.json"));
  const upstreamExports = readUpstream("src/v4/classic/external.ts");
  const upstreamParse = readUpstream("src/v4/classic/parse.ts");
  const upstreamErrors = readUpstream("src/v4/core/errors.ts");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const packageDoc = read("docs/packages/validation-zod.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "zod");
  assert.equal(upstreamPackage.version, "4.4.3");
  assert.match(upstreamExports, /flattenError/);
  assert.match(upstreamExports, /treeifyError/);
  assert.match(upstreamExports, /toJSONSchema/);
  assert.match(upstreamParse, /export const safeParse/);
  assert.match(upstreamParse, /export const safeParseAsync/);
  assert.match(upstreamErrors, /export interface \$ZodIssueInvalidType/);

  assert.match(reader, /VALIDATION_SCHEMAS_PACKAGE_ID: &str = "validation\/zod"/);
  assert.match(reader, /VALIDATION_SCHEMAS_OFFICIAL_NAME: &str = "Validation & Schemas"/);
  assert.match(reader, /VALIDATION_SCHEMAS_UPSTREAM_PACKAGE: &str = "zod"/);
  assert.match(reader, /VALIDATION_SCHEMAS_UPSTREAM_VERSION: &str = "4\.4\.3"/);
  assert.match(
    reader,
    /VALIDATION_SCHEMAS_PACKAGE_STATUS_PATH: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    reader,
    /VALIDATION_SCHEMAS_PACKAGE_RECEIPT_PATH: &str =\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-validation-zod-dashboard-settings\.json"/,
  );
  assert.match(reader, /rows\.extend\(validation_schemas_package_lane_row\(root, package_status\)\)/);
  assert.match(reader, /fn validation_schemas_package_lane_row\(\s*root: &Path,\s*package_status: Option<&ForgePackageStatusReadModel>,/);
  assert.match(reader, /validation_schemas_hash_manifest_present/);
  assert.match(reader, /validation_schemas_hash_mismatch/);
  assert.match(reader, /validation_schemas_receipt_hash_refresh_current/);
  assert.match(reader, /validation_schemas_receipt_hash_refresh_stale/);
  assert.match(reader, /validation_schemas_receipt_hash_refresh_missing/);
  assert.match(reader, /receipt_hash_refresh_counts\(package\)/);
  assert.match(reader, /pub tracked_files: Vec<String>/);
  assert.match(reader, /pub source_guard_runbook_fixture: Option<String>/);
  assert.match(reader, /pub preview_manifest_materializer: Option<String>/);
  assert.match(reader, /tracked_files: json_string_array\(refresh, &\["tracked_files"\]\)/);
  assert.match(
    reader,
    /source_guard_runbook_fixture: json_text\(refresh, &\["source_guard_runbook_fixture"\]\)\s*\.map\(str::to_string\)/,
  );
  assert.match(
    reader,
    /preview_manifest_materializer: json_text\(refresh, &\["preview_manifest_materializer"\]\)\s*\.map\(str::to_string\)/,
  );
  assert.match(reader, /dx_check_latest_panel_exposes_validation_schemas_package_lane_hash_refresh_row/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["tracked_files"\]/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["current_files"\]/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["stale_files"\]/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["missing_files"\]/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["source_guard_runbook_fixture"\]/);
  assert.match(reader, /validation\["receipt_hash_refresh"\]\["preview_manifest_materializer"\]/);

  assert.match(runtimeLaunch, /data-dx-check-package-lane-template="validation\/zod"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-row="validation\/zod"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-name="Validation & Schemas"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-status="missing"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-status="missing-receipt"/,
  );
  assert.match(runtimeLaunch, /data-dx-check-package-lane-upstream-package="zod"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/zod"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-validation-zod-dashboard-settings\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/validation-schemas-receipt-hashes\.ts"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-zed="validation-schemas:receipt-hash-refresh"/,
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Validation & Schemas package row/);
    assert.match(source, /static \/launch package-lane markers for Validation & Schemas/);
    assert.match(source, /generated-starter materialization guard for Validation & Schemas/);
    assert.match(source, /source Studio dx-check panel package scope for Validation & Schemas/i);
    assert.match(source, /validation_schemas_hash_manifest_present/);
    assert.match(source, /validation_schemas_hash_mismatch/);
    assert.match(source, /validation_schemas_receipt_hash_refresh_current/);
    assert.match(source, /validation_schemas_receipt_hash_refresh_stale/);
    assert.match(source, /without claiming live Validation & Schemas runtime proof/);
    assert.match(source, /tracked_files/);
    assert.match(source, /source_guard_runbook_fixture/);
    assert.match(source, /preview_manifest_materializer/);
    assert.match(source, /current_files/);
    assert.match(source, /stale_files/);
    assert.match(source, /missing_files/);
  }
});

test("Validation & Schemas is scoped into source Studio dx-check panel surfaces", () => {
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const rustManifest = read("dx-www/src/cli/studio_manifest.rs");
  const sourcePanel = sourceEditSurface(editContract, "dx-check-health-panel");
  const rustPanel = rustFunction(rustManifest, "studio_dx_check_edit_surface");

  assert.match(sourcePanel, /selector: '\[data-dx-component="dx-check-health-panel"\]'/);
  assert.match(sourcePanel, /sourceFile: "examples\/template\/template-shell\.tsx"/);
  assert.match(sourcePanel, /materializedFile: "components\/template-app\/template-shell\.tsx"/);
  assert.match(
    sourcePanel,
    /"validation\/zod"/,
    "source Studio dx-check panel package scope must include Validation & Schemas",
  );

  assert.match(rustPanel, /"dx-check-health-panel"/);
  assert.match(rustPanel, /"examples\/template\/template-shell\.tsx"/);
  assert.match(rustPanel, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(
    rustPanel,
    /"validation\/zod"/,
    "Rust Studio manifest dx-check panel package scope must include Validation & Schemas",
  );
});

test("Validation & Schemas publishes a Studio source-guard runbook entry", () => {
  const rustManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/validation-zod.md");

  for (const source of [rustManifest, frameworkDoc, packageDoc]) {
    assert.match(source, /validation-schemas-generated-starter-materialization/);
    assert.match(source, /Validation & Schemas generated-starter materialization guard/);
    assert.equal(
      source.includes("dx run --test .\\benchmarks\\zod-dx-check-package-lane-panel.test.ts") ||
        source.includes(
          "dx run --test .\\\\benchmarks\\\\zod-dx-check-package-lane-panel.test.ts",
        ),
      true,
      "Studio source-guard docs must include the targeted Validation & Schemas command",
    );
  }

  assert.match(rustManifest, /data-dx-check-package-lane-row=\\"validation\/zod\\"/);
  assert.match(rustManifest, /data-dx-token-scope=\\"validation\/zod\\"/);
  assert.match(rustManifest, /validation-schemas:receipt-hash-refresh/);
  assert.match(rustManifest, /validation\/zod source-only Studio discovery/);
  assert.match(
    rustManifest,
    /without claiming live Validation & Schemas runtime proof/,
  );
  assert.match(
    rustManifest,
    /The generated starter preserves the Validation & Schemas package-lane row, helper freshness markers, dx-style markers, and package-scoped dx-check panel without live Validation & Schemas runtime proof\./,
  );
  assert.match(
    rustManifest,
    /Validate the source-only Validation & Schemas package-lane row, generated starter materialization, helper freshness markers, dx-style markers, and dx-check panel package scope without live Validation & Schemas runtime proof\./,
  );
  assert.match(
    rustManifest,
    /"validation-schemas-generated-starter-materialization"[\s\S]*"markdown-mdx-content-materialized-source-fixture"/,
  );
});

test("Validation & Schemas publishes a package-owned source-guard runbook fixture", () => {
  const fixturePath = "docs/packages/validation-schemas.source-guard-runbook.json";
  const fixtureAbsolutePath = path.join(root, fixturePath);
  const rustManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/validation-zod.md");

  assert.ok(fs.existsSync(fixtureAbsolutePath), `${fixturePath} is missing`);
  const fixture = JSON.parse(fs.readFileSync(fixtureAbsolutePath, "utf8"));

  assert.equal(fixture.schema, "dx.forge.package.source_guard_runbook_fixture");
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Validation & Schemas");
  assert.equal(fixture.package.package_id, "validation/zod");
  assert.equal(fixture.package.upstream_package, "zod");
  assert.equal(fixture.package.upstream_version, "4.4.3");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/zod");
  assert.deepEqual(fixture.selected_surfaces, [
    "dashboard-settings-validation",
    "template-forms-validation",
    "starter-dashboard-settings-validator",
    "generated-starter-materialization",
    "receipt-hash-refresh",
  ]);
  assert.deepEqual(fixture.upstream_public_apis, [
    "safeParse",
    "safeParseAsync",
    "z.strictObject",
    "z.flattenError",
    "z.treeifyError",
    "z.prettifyError",
    "z.toJSONSchema",
    ".meta()",
    ".readonly()",
  ]);
  assert.ok(
    fixture.inspected_upstream_files.includes("src/v4/classic/external.ts"),
  );
  assert.ok(
    fixture.inspected_upstream_files.includes("src/v4/core/to-json-schema.ts"),
  );
  assert.equal(
    fixture.guard.id,
    "validation-schemas-generated-starter-materialization",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\zod-dx-check-package-lane-panel.test.ts",
  );
  assert.ok(fixture.guard.proves.includes(fixturePath));
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);
  assert.equal(
    fixture.runbook.contract.id,
    "validation-schemas-generated-starter-materialization",
  );
  assert.equal(
    fixture.runbook.command.command,
    "dx run --test .\\benchmarks\\zod-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.receipt.zed_visibility, "validation-schemas:receipt-hash-refresh");
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.equal(
    fixture.generated_preview_manifest.fixture_path,
    "docs/packages/validation-schemas.source-guard-runbook.json",
  );
  assert.deepEqual(fixture.generated_preview_manifest.paths, [
    "sourceGuardRunbookFixtures",
    "routes[].sourceGuardRunbookFixtures",
  ]);
  assert.equal(
    fixture.generated_preview_manifest.zed_visibility,
    "validation-schemas:receipt-hash-refresh",
  );
  assert.equal(fixture.generated_preview_manifest.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /SOURCE-ONLY/);
  assert.match(
    fixture.runtime_limitations.join("\n"),
    /browser runtime proof, settings persistence, authorization, and dependency installation stay app-owned/i,
  );

  for (const source of [rustManifest, frameworkDoc, packageDoc]) {
    assert.match(source, /docs\/packages\/validation-schemas\.source-guard-runbook\.json/);
  }
});

test("Validation & Schemas package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-zod-package-lane-"));
  const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.match(launch, /data-dx-check-package-lane-row="validation\/zod"/);
  assert.match(launch, /data-dx-check-package-lane-name="Validation & Schemas"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/validation-schemas-receipt-hashes\.ts"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-zed="validation-schemas:receipt-hash-refresh"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-dx-style-status="present"/,
  );
  assert.match(launch, /data-dx-style-surface="validation-schemas"/);
  assert.match(launch, /data-dx-token-scope="validation\/zod"/);

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized /launch route metadata");
  assert.ok(launchRoute.forgePackages.includes("validation/zod"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-style-surface"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-token-scope"));
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/validation-schemas.source-guard-runbook.json",
    ),
    "generated /launch metadata must point at the Validation & Schemas source-guard fixture",
  );

  const fixture = manifest.sourceGuardRunbookFixtures.find(
    (entry) => entry.packageId === "validation/zod",
  );
  assert.ok(fixture, "expected Validation & Schemas source-guard fixture metadata");
  assert.equal(fixture.officialPackageName, "Validation & Schemas");
  assert.equal(fixture.fixture, "docs/packages/validation-schemas.source-guard-runbook.json");
  assert.equal(fixture.guardId, "validation-schemas-generated-starter-materialization");
  assert.equal(fixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(fixture.runtimeProof, false);
  assert.equal(fixture.zedVisibility, "validation-schemas:receipt-hash-refresh");

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("validation/zod"),
    "generated dx-check panel package scope must include Validation & Schemas",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-helper"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-zed"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-dx-style-status"),
  );
  assert.ok(checkPanel.stateMarkers.includes("data-dx-style-surface"));
  assert.ok(checkPanel.stateMarkers.includes("data-dx-token-scope"));

  const validationSurface = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-settings-validation",
  );
  assert.ok(validationSurface, "expected generated settings validation edit surface");
  assert.equal(validationSurface.sourceFile, "pages/index.html");
  assert.ok(
    validationSurface.packageIds.includes("validation/zod"),
    "settings validation surface must stay scoped to Validation & Schemas",
  );
  assert.ok(
    validationSurface.stateMarkers.includes("data-dx-style-surface"),
    "settings validation surface must expose dx-style surface marker",
  );
  assert.ok(
    validationSurface.stateMarkers.includes("data-dx-token-scope"),
    "settings validation surface must expose package token scope marker",
  );
  assert.ok(
    validationSurface.stateMarkers.includes("data-dx-check-package-lane-dx-style-status"),
    "settings validation surface must expose package-lane dx-style status marker",
  );
});
