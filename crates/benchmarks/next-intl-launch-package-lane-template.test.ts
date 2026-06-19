const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Internationalization exposes a static www-template package-lane template", () => {
  const runbookFixturePath = "docs/packages/next-intl.source-guard-runbook.json";
  assert.ok(
    fs.existsSync(path.join(root, runbookFixturePath)),
    "Internationalization source-guard runbook fixture should exist",
  );

  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const provider = readMirror(
    "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
  );
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/next-intl.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const runbookFixture = readJson(runbookFixturePath);
  const intlTemplate = launchShell.slice(
    launchShell.indexOf('package_id: "i18n/next-intl"'),
    launchShell.indexOf('package_id: "supabase/client"'),
  );

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);

  assert.match(launchShell, /dxCheckPackageLaneTemplates/);
  assert.match(launchShell, /package_id: "i18n\/next-intl"/);
  assert.match(launchShell, /official_package_name: "Internationalization"/);
  assert.match(launchShell, /upstream_package: "next-intl"/);
  assert.match(launchShell, /upstream_version: "4\.12\.0"/);
  assert.match(launchShell, /source_mirror: "G:\/WWW\/inspirations\/next-intl"/);
  assert.match(
    launchShell,
    /package_receipt_path:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"/,
  );
  assert.match(launchShell, /dx_style_status: "present"/);
  assert.match(
    launchShell,
    /data-dx-check-package-lane-template=\{packageLane\.package_id\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-row=\{packageLane\.package_id\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-dx-style-status=\{packageLane\.dx_style_status\}/,
  );
  assert.match(launchShell, /data-dx-style-surface=\{packageLane\.dx_style_surface\}/);
  assert.match(launchShell, /data-dx-token-scope=\{packageLane\.token_scope\}/);
  assert.match(intlTemplate, /hash_refresh_status: "current"/);
  assert.match(
    intlTemplate,
    /hash_refresh_helper:\s*"examples\/template\/internationalization-receipt-hashes\.ts"/,
  );
  assert.match(
    intlTemplate,
    /hash_refresh_json_command:\s*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/internationalization-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    intlTemplate,
    /hash_refresh_zed: "internationalization:receipt-hash-refresh"/,
  );
  assert.match(intlTemplate, /hash_refresh_tracked_files: 4/);
  assert.match(intlTemplate, /hash_refresh_stale_files: 0/);
  assert.match(intlTemplate, /hash_refresh_missing_files: 0/);
  assert.match(
    intlTemplate,
    /hash_refresh_metric_current:\s*"internationalization_receipt_hash_refresh_current"/,
  );
  assert.match(
    intlTemplate,
    /hash_refresh_metric_stale:\s*"internationalization_receipt_hash_refresh_stale"/,
  );
  assert.match(
    intlTemplate,
    /hash_refresh_metric_missing:\s*"internationalization_receipt_hash_refresh_missing"/,
  );

  for (const source of [runtimeLaunch, editContract, materializer]) {
    assert.match(source, /data-dx-check-package-lane-template/);
    assert.match(source, /i18n\/next-intl/);
    assert.match(source, /data-dx-check-package-lane-dx-style-status/);
  }

  assert.match(runtimeLaunch, /Internationalization/);
  assert.match(runtimeLaunch, /next-intl/);
  assert.match(runtimeLaunch, /G:\/WWW\/inspirations\/next-intl/);
  assert.match(
    runtimeLaunch,
    /examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json/,
  );
  assert.match(runtimeLaunch, /data-dx-check-package-lane-status="missing"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-receipt-status="missing-receipt"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-dx-style-status="present"/);
  assert.match(runtimeLaunch, /data-dx-style-surface="internationalization"/);
  assert.match(runtimeLaunch, /data-dx-token-scope="i18n\/next-intl"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-hash-refresh-status="current"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/internationalization-receipt-hashes\.ts"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/internationalization-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-zed="internationalization:receipt-hash-refresh"/,
  );
  assert.match(runtimeLaunch, /data-dx-check-package-lane-hash-refresh-tracked-files="4"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-hash-refresh-stale-files="0"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-hash-refresh-missing-files="0"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-current-metric="internationalization_receipt_hash_refresh_current"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-stale-metric="internationalization_receipt_hash_refresh_stale"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-missing-metric="internationalization_receipt_hash_refresh_missing"/,
  );

  assert.match(editContract, /id: "dx-check-health-panel"[\s\S]*"i18n\/next-intl"/);
  assert.match(materializer, /"launch-runtime-dx-check-panel"[\s\S]*"i18n\/next-intl"/);

  assert.match(studioManifest, /studio_source_guard\(\s*"internationalization-launch-package-lane-template"/);
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"benchmarks\/next-intl-launch-package-lane-template\.test\.ts"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"dx run --test \.\\\\benchmarks\\\\next-intl-launch-package-lane-template\.test\.ts"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"data-dx-check-package-lane-template"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"data-dx-style-surface=\\\"internationalization\\\""/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"examples\/template\/internationalization-receipt-hashes\.ts"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/internationalization-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"internationalization:receipt-hash-refresh"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"internationalization_receipt_hash_refresh_current"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"internationalization_receipt_hash_refresh_stale"/,
  );
  assert.match(
    studioManifest,
    /"internationalization-launch-package-lane-template"[\s\S]*"internationalization_receipt_hash_refresh_missing"/,
  );
  assert.match(studioManifest, /docs\/packages\/next-intl\.source-guard-runbook\.json/);
  assert.match(
    studioManifest,
    /source_guard_command\(\s*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/internationalization-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-template"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-dx-style-status"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-helper"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-json-command"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-zed"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-current-metric"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-stale-metric"/,
  );
  assert.match(
    studioManifest,
    /"package": "i18n\/next-intl"[\s\S]*"data-dx-check-package-lane-hash-refresh-missing-metric"/,
  );
  assert.match(studioManifest, /"\/" => guards\.extend\(\[[\s\S]*"internationalization-launch-package-lane-template"/);
  assert.match(
    frameworkDoc,
    /Lane 7 uses the official front-facing package name `Internationalization`[\s\S]*docs\/packages\/next-intl\.source-guard-runbook\.json/,
  );
  assert.match(packageDoc, /docs\/packages\/next-intl\.source-guard-runbook\.json/);

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(
    runbookFixture.package.official_package_name,
    "Internationalization",
  );
  assert.equal(runbookFixture.package.package_id, "i18n/next-intl");
  assert.equal(runbookFixture.package.upstream_package, "next-intl");
  assert.equal(runbookFixture.package.upstream_version, "4.12.0");
  assert.equal(
    runbookFixture.package.source_mirror,
    "G:/WWW/inspirations/next-intl",
  );
  assert.deepEqual(runbookFixture.selected_surfaces, [
    "launch-package-lane-template",
    "dashboard-locale-workflow",
    "receipt-hash-refresh",
    "internationalization-source-guard-runbook",
  ]);
  assert.equal(
    runbookFixture.guard.id,
    "internationalization-launch-package-lane-template",
  );
  assert.deepEqual(runbookFixture.guard.routes, ["/"]);
  assert.equal(
    runbookFixture.guard.command,
    "dx run --test .\\benchmarks\\next-intl-launch-package-lane-template.test.ts",
  );
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  assert.equal(runbookFixture.guard.starts_server, false);
  assert.equal(runbookFixture.guard.runs_package_install, false);
  assert.equal(runbookFixture.guard.runs_full_build, false);
  assert.equal(runbookFixture.guard.node_modules_required, false);
  assert.ok(
    runbookFixture.guard.proves.includes(
      "docs/packages/next-intl.source-guard-runbook.json",
    ),
  );
  assert.equal(
    runbookFixture.runbook.command.command,
    runbookFixture.guard.command,
  );
  assert.equal(
    runbookFixture.runbook.helper_command.command,
    "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json",
  );
  assert.equal(runbookFixture.runbook.command.starts_server, false);
  assert.equal(runbookFixture.runbook.helper_command.runs_package_install, false);
  assert.deepEqual(runbookFixture.preview_manifest, {
    generated_file: "public/preview-manifest.json",
    materializer: "tools/launch/materialize-www-template.ts",
    field: "sourceGuardRunbookFixtures",
    route_field: "routes[].sourceGuardRunbookFixtures",
    fixture_path: runbookFixturePath,
    route: "/",
    runtime_proof: false,
    zed_visibility: "internationalization:receipt-hash-refresh",
  });
  assert.ok(runbookFixture.upstream_public_apis.includes("useTranslations"));
  assert.ok(runbookFixture.upstream_public_apis.includes("createMiddleware"));
  assert.ok(
    runbookFixture.inspected_upstream_files.includes(
      "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
    ),
  );
  assert.ok(
    runbookFixture.dx_check_metrics.includes(
      "internationalization_receipt_hash_refresh_current",
    ),
  );
  assert.ok(
    runbookFixture.zed_dx_studio_markers.includes(
      "data-dx-check-package-lane-hash-refresh-helper",
    ),
  );
  assert.equal(
    runbookFixture.receipt.zed_visibility,
    "internationalization:receipt-hash-refresh",
  );
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("SOURCE-ONLY"),
    ),
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Internationalization www-template package-lane template/);
    assert.match(source, /Internationalization static `\/launch` helper freshness markers/);
    assert.match(source, /data-dx-check-package-lane-template="i18n\/next-intl"/);
    assert.match(source, /data-dx-style-surface="internationalization"/);
    assert.match(source, /internationalization-receipt-hashes\.ts/);
    assert.match(source, /internationalization:receipt-hash-refresh/);
    assert.match(source, /internationalization_receipt_hash_refresh_current/);
    assert.match(source, /internationalization_receipt_hash_refresh_stale/);
    assert.match(source, /internationalization_receipt_hash_refresh_missing/);
    assert.match(source, /without claiming live locale routing proof/);
  }
});

test("Internationalization package-lane template survives generated starter materialization", () => {
  const runbookFixturePath = "docs/packages/next-intl.source-guard-runbook.json";
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-intl-package-lane-"));
  const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  const intlFixture = manifest.sourceGuardRunbookFixtures.find(
    (fixture) => fixture.packageId === "i18n/next-intl",
  );
  assert.ok(
    intlFixture,
    "generated preview manifest must include the Internationalization runbook fixture",
  );
  assert.equal(intlFixture.officialPackageName, "Internationalization");
  assert.equal(intlFixture.upstreamPackage, "next-intl");
  assert.equal(intlFixture.upstreamVersion, "4.12.0");
  assert.equal(intlFixture.sourceMirror, "G:/WWW/inspirations/next-intl");
  assert.equal(intlFixture.fixture, runbookFixturePath);
  assert.equal(intlFixture.guardId, "internationalization-launch-package-lane-template");
  assert.equal(intlFixture.schema, "dx.forge.package.source_guard_runbook_fixture");
  assert.equal(intlFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(intlFixture.runtimeProof, false);
  assert.equal(intlFixture.zedVisibility, "internationalization:receipt-hash-refresh");

  assert.match(launch, /data-dx-check-package-lane-template="i18n\/next-intl"/);
  assert.match(launch, /data-dx-check-package-lane-row="i18n\/next-intl"/);
  assert.match(launch, /data-dx-check-package-lane-name="Internationalization"/);
  assert.match(launch, /data-dx-check-package-lane-upstream-package="next-intl"/);
  assert.match(launch, /data-dx-check-package-lane-dx-style-status="present"/);
  assert.match(launch, /data-dx-style-surface="internationalization"/);
  assert.match(launch, /data-dx-token-scope="i18n\/next-intl"/);
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-status="current"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/internationalization-receipt-hashes\.ts"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/internationalization-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-zed="internationalization:receipt-hash-refresh"/,
  );
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-tracked-files="4"/);
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-stale-files="0"/);
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-missing-files="0"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-current-metric="internationalization_receipt_hash_refresh_current"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-stale-metric="internationalization_receipt_hash_refresh_stale"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-missing-metric="internationalization_receipt_hash_refresh_missing"/,
  );

  const rootRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(rootRoute, "expected generated root route metadata");
  assert.ok(
    rootRoute.forgePackages.includes("3d/launch-scene"),
    "generated root route should stay scoped to the 3D landing scene package",
  );
  assert.ok(
    rootRoute.forgePackages.includes("i18n/next-intl"),
    "generated root route must include the Internationalization package lane",
  );

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected generated / route metadata");
  assert.ok(
    launchRoute.forgePackages.includes("i18n/next-intl"),
    "generated / route must include the Internationalization package lane",
  );
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(runbookFixturePath),
    "generated /launch route must expose the Internationalization runbook fixture",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-template"),
    "generated /launch manifest must expose package-lane template marker",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-intl-dashboard-workflow"),
    "generated /launch manifest must expose Internationalization dashboard marker",
  );

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("i18n/next-intl"),
    "generated dx-check panel package scope must include Internationalization",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-template"),
    "generated dx-check panel must keep the package-lane template marker",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-dx-style-status"),
    "generated dx-check panel must keep the dx-style status marker",
  );
});
