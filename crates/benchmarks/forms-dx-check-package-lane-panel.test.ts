const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/react-hook-form");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

function sourceSection(source, marker, label) {
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `missing ${label}`);
  const rest = source.slice(start);
  const end = ["\n  },", "\n    ),"]
    .map((candidate) => rest.indexOf(candidate))
    .filter((index) => index >= 0)
    .sort((a, b) => a - b)[0];
  assert.notEqual(end, -1, `missing end of ${label}`);
  return rest.slice(0, end);
}

test("Forms package-lane row exposes hash-backed DX check-panel visibility", () => {
  const upstreamPackage = JSON.parse(readUpstream("package.json"));
  const upstreamIndex = readUpstream("src/index.ts");
  const upstreamUseForm = readUpstream("src/useForm.ts");
  const upstreamContext = readUpstream("src/useFormContext.tsx");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/forms-react-hook-form.md");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const formsRow = reader.slice(
    reader.indexOf("fn forms_package_lane_row"),
    reader.indexOf("fn forms_missing_receipt_row"),
  );

  assert.equal(upstreamPackage.name, "react-hook-form");
  assert.equal(upstreamPackage.version, "7.75.0");
  assert.match(upstreamIndex, /export \* from '\.\/useForm'/);
  assert.match(upstreamUseForm, /export function useForm/);
  assert.match(upstreamContext, /export const FormProvider/);

  assert.match(reader, /FORMS_PACKAGE_ID: &str = "forms\/react-hook-form"/);
  assert.match(reader, /FORMS_OFFICIAL_NAME: &str = "Forms"/);
  assert.match(reader, /FORMS_UPSTREAM_PACKAGE: &str = "react-hook-form"/);
  assert.match(reader, /FORMS_UPSTREAM_VERSION: &str = "7\.75\.0"/);
  assert.match(
    reader,
    /FORMS_PACKAGE_STATUS_PATH: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    reader,
    /FORMS_PACKAGE_RECEIPT_PATH: &str =\s*"\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );
  assert.match(reader, /rows\.extend\(forms_package_lane_row\(root, package_status\)\)/);
  assert.match(reader, /forms_package_lane_row\(\s*root: &Path,\s*package_status: Option<&ForgePackageStatusReadModel>,/);
  assert.match(reader, /forms_hash_manifest_present/);
  assert.match(reader, /forms_hash_mismatch/);
  assert.match(reader, /const FORMS_METRICS: \[&str; 11\]/);
  assert.match(reader, /forms_receipt_hash_refresh_current/);
  assert.match(reader, /forms_receipt_hash_refresh_stale/);
  assert.match(reader, /forms_receipt_hash_refresh_missing/);
  assert.match(
    reader,
    /let stale_path_count = json_string_array\(refresh, &\["stale_files"\]\)\.len\(\) as u64\s*\+\s*json_string_array\(refresh, &\["stale_mirror_files"\]\)\.len\(\) as u64;/,
  );
  assert.match(
    reader,
    /let missing_path_count = json_string_array\(refresh, &\["missing_files"\]\)\.len\(\) as u64\s*\+\s*json_string_array\(refresh, &\["missing_mirror_files"\]\)\.len\(\) as u64;/,
  );
  assert.match(
    reader,
    /status == "stale" \|\| stale_file_count > 0 \|\| stale_path_count > 0/,
  );
  assert.match(
    reader,
    /status == "missing" \|\| missing_file_count > 0 \|\| missing_path_count > 0/,
  );
  assert.match(formsRow, /let \(refresh_current, refresh_stale, refresh_missing\) = receipt_hash_refresh_counts\(package\)/);
  assert.match(
    formsRow,
    /let \(refresh_current, refresh_stale, refresh_missing\) = receipt_hash_refresh_counts\(package\);\s*let stale_receipt = u64::from\(\s*matches!\(visibility_status, "stale"\)\s*\|\| matches!\(receipt_status, "stale"\)\s*\|\| hash_mismatches > 0\s*\|\| refresh_stale > 0,/,
  );
  assert.match(reader, /count_sha256_file_hash_mismatches\(root, package\)/);
  assert.match(reader, /forms_next_action\(status, refresh_stale, refresh_missing\)/);
  assert.match(
    reader,
    /fn forms_next_action\(\s*status: &str,\s*refresh_stale: u64,\s*refresh_missing: u64\)/,
  );
  assert.match(reader, /dx_check_latest_panel_exposes_forms_package_lane_hash_row/);
  assert.match(reader, /stale_helper_package_status\["package_lane_visibility"\]\[0\]\["receipt_hash_refresh"\]\["stale_file_count"\]/);
  assert.match(
    reader,
    /stale_helper_package_status\["package_lane_visibility"\]\[0\]\["receipt_hash_refresh"\]\["stale_files"\]\s*=\s*serde_json::json!\(\["docs\/packages\/forms-react-hook-form\.md"\]\)/,
  );
  assert.match(
    reader,
    /helper_stale_forms\["receipt_hash_refresh"\]\["stale_files"\]\[0\],\s*"docs\/packages\/forms-react-hook-form\.md"/,
  );
  assert.match(reader, /helper_stale_metric_value\("forms_receipt_hash_refresh_stale"\)/);
  assert.match(reader, /helper_stale_metric_value\("forms_hash_mismatch"\),\s*0/);

  assert.match(
    launchShell,
    /package_id: "forms\/react-hook-form"[\s\S]*official_package_name: "Forms"[\s\S]*hash_refresh_helper:\s*"examples\/template\/forms-receipt-hashes\.ts"[\s\S]*hash_refresh_metric_current: "forms_receipt_hash_refresh_current"[\s\S]*hash_refresh_metric_stale: "forms_receipt_hash_refresh_stale"[\s\S]*hash_refresh_metric_missing: "forms_receipt_hash_refresh_missing"/,
  );
  const formsTemplate = sourceSection(
    launchShell,
    'package_id: "forms/react-hook-form"',
    "Forms static package-lane template",
  );
  assert.match(formsTemplate, /hash_refresh_stale_file_list:\s*""/);
  assert.match(formsTemplate, /hash_refresh_missing_file_list:\s*""/);

  const formsRuntimeStart = runtimePage.indexOf(
    'data-dx-check-package-lane-template="forms/react-hook-form"',
  );
  assert.notEqual(formsRuntimeStart, -1, "missing Forms static runtime row");
  const formsRuntimeEnd = runtimePage.indexOf(
    'data-dx-check-package-lane-template="i18n/next-intl"',
    formsRuntimeStart,
  );
  assert.notEqual(formsRuntimeEnd, -1, "missing end of Forms static runtime row");
  const formsRuntimeRow = runtimePage.slice(formsRuntimeStart, formsRuntimeEnd);
  assert.match(
    formsRuntimeRow,
    /data-dx-check-package-lane-hash-refresh-stale-file-list=""/,
  );
  assert.match(
    formsRuntimeRow,
    /data-dx-check-package-lane-hash-refresh-missing-file-list=""/,
  );
  assert.match(
    runtimePage,
    /data-dx-check-package-lane-template="forms\/react-hook-form"[\s\S]*data-dx-check-package-lane-row="forms\/react-hook-form"[\s\S]*data-dx-check-package-lane-name="Forms"[\s\S]*data-dx-check-package-lane-hash-refresh-helper="examples\/template\/forms-receipt-hashes\.ts"[\s\S]*data-dx-check-package-lane-hash-refresh-current-metric="forms_receipt_hash_refresh_current"[\s\S]*data-dx-check-package-lane-hash-refresh-stale-metric="forms_receipt_hash_refresh_stale"[\s\S]*data-dx-check-package-lane-hash-refresh-missing-metric="forms_receipt_hash_refresh_missing"/,
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Forms package row/);
    assert.match(source, /forms_hash_manifest_present/);
    assert.match(source, /forms_hash_mismatch/);
    assert.match(source, /stale-helper-only|stale helper only/);
    assert.match(source, /static .*\/launch|static launch/);
    assert.match(source, /receiptHashRefresh|receipt_hash_refresh/);
    assert.match(source, /without claiming browser submission proof/);
  }
});

test("Forms package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forms-package-lane-"));
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

  try {
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
    const sourceContract = read("examples/template/dx-studio-edit-contract.ts");
    const materializerSource = read("tools/launch/materialize-www-template.ts");
    const studioManifest = read("dx-www/src/cli/studio_manifest.rs");

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

    for (const marker of [
      'data-dx-check-package-lane-template="forms/react-hook-form"',
      'data-dx-check-package-lane-row="forms/react-hook-form"',
      'data-dx-check-package-lane-name="Forms"',
      'data-dx-check-package-lane-upstream-package="react-hook-form"',
      'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/react-hook-form"',
      'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json"',
      'data-dx-check-package-lane-hash-refresh-helper="examples/template/forms-receipt-hashes.ts"',
      'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check --json"',
      'data-dx-check-package-lane-hash-refresh-zed="forms:receipt-hash-refresh"',
      'data-dx-check-package-lane-hash-refresh-tracked-files="7"',
      'data-dx-check-package-lane-hash-refresh-stale-files="0"',
      'data-dx-check-package-lane-hash-refresh-missing-files="0"',
      'data-dx-check-package-lane-hash-refresh-stale-file-list=""',
      'data-dx-check-package-lane-hash-refresh-missing-file-list=""',
      'data-dx-check-package-lane-hash-refresh-current-metric="forms_receipt_hash_refresh_current"',
      'data-dx-check-package-lane-hash-refresh-stale-metric="forms_receipt_hash_refresh_stale"',
      'data-dx-check-package-lane-hash-refresh-missing-metric="forms_receipt_hash_refresh_missing"',
      'data-dx-style-surface="forms"',
      'data-dx-token-scope="forms/react-hook-form"',
      'data-dx-package="forms/react-hook-form"',
    ]) {
      assert.match(launch, escaped(marker), `missing generated Forms marker ${marker}`);
    }

    for (const routePath of ["/login", "/dashboard", "/"]) {
      const route = manifest.routes.find((entry) => entry.route === routePath);
      assert.ok(route, `expected generated ${routePath} route metadata`);
      assert.ok(
        route.forgePackages.includes("forms/react-hook-form"),
        `generated ${routePath} route package scope must include Forms`,
      );
    }

    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const formsRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "forms/react-hook-form",
    );
    assert.ok(
      formsRunbookFixture,
      "generated preview manifest must expose Forms source-guard runbook fixture",
    );
    assert.equal(formsRunbookFixture.officialPackageName, "Forms");
    assert.equal(formsRunbookFixture.upstreamPackage, "react-hook-form");
    assert.equal(formsRunbookFixture.upstreamVersion, "7.75.0");
    assert.equal(
      formsRunbookFixture.sourceMirror,
      "G:/WWW/inspirations/react-hook-form",
    );
    assert.equal(
      formsRunbookFixture.fixture,
      "docs/packages/forms.source-guard-runbook.json",
    );
    assert.equal(
      formsRunbookFixture.guardId,
      "forms-generated-starter-materialization",
    );
    assert.equal(formsRunbookFixture.route, "/");
    assert.equal(formsRunbookFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(formsRunbookFixture.runtimeProof, false);
    assert.equal(
      formsRunbookFixture.zedVisibility,
      "forms:receipt-hash-refresh",
    );
    const launchRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(
        "docs/packages/forms.source-guard-runbook.json",
      ),
      "generated /launch route must link the Forms source-guard runbook fixture",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected generated dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("forms/react-hook-form"),
      "generated dx-check panel package scope must include Forms",
    );

    for (const marker of [
      "data-dx-check-package-lane-template",
      "data-dx-check-package-lane-row",
      "data-dx-check-package-lane-hash-refresh-helper",
      "data-dx-check-package-lane-hash-refresh-json-command",
      "data-dx-check-package-lane-hash-refresh-zed",
      "data-dx-check-package-lane-hash-refresh-stale-file-list",
      "data-dx-check-package-lane-hash-refresh-missing-file-list",
      "data-dx-check-package-lane-hash-refresh-current-metric",
      "data-dx-check-package-lane-hash-refresh-stale-metric",
      "data-dx-check-package-lane-hash-refresh-missing-metric",
      "data-dx-style-surface",
      "data-dx-token-scope",
    ]) {
      assert.ok(
        checkPanel.stateMarkers.includes(marker),
        `generated dx-check panel must expose ${marker}`,
      );
    }

    const sourceDxCheckPanel = sourceSection(
      sourceContract,
      'id: "dx-check-health-panel"',
      "source Forms dx-check panel package scope",
    );
    assert.match(sourceDxCheckPanel, /"forms\/react-hook-form"/);
    assert.match(
      sourceContract,
      /"data-dx-check-package-lane-hash-refresh-missing-file-list"/,
    );

    const materializedDxCheckPanel = sourceSection(
      materializerSource,
      '"launch-runtime-dx-check-panel"',
      "materialized Forms dx-check panel package scope",
    );
    assert.match(materializedDxCheckPanel, /"forms\/react-hook-form"/);
    assert.match(
      materializerSource,
      /"data-dx-check-package-lane-hash-refresh-missing-file-list"/,
    );

    assert.match(
      studioManifest,
      /fn studio_dx_check_edit_surface\(\)[\s\S]*"forms\/react-hook-form"/,
      "Rust Studio manifest dx-check panel package scope must include Forms",
    );

    for (const source of [
      read("docs/packages/forms-react-hook-form.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(source, /Forms generated-starter materialization guard/);
      assert.match(source, /without claiming browser submission proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("Forms Studio source guard runbook exposes generated-starter materialization command", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/forms-react-hook-form.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"forms-generated-starter-materialization",\s*&\["\/"\],\s*"benchmarks\/forms-dx-check-package-lane-panel\.test\.ts",\s*"dx run --test \.\\\\benchmarks\\\\forms-dx-check-package-lane-panel\.test\.ts"[\s\S]*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /"forms-generated-starter-materialization"[\s\S]*"Forms generated-starter materialization guard"[\s\S]*"data-dx-check-package-lane-row=\\\"forms\/react-hook-form\\\""[\s\S]*"data-dx-token-scope=\\\"forms\/react-hook-form\\\""[\s\S]*"forms:receipt-hash-refresh"[\s\S]*"forms\/react-hook-form source-only Studio discovery"/,
  );
  assert.match(
    studioManifest,
    /"forms-generated-starter-materialization"[\s\S]*"The generated starter preserves the Forms package-lane row, helper freshness markers, and package-scoped dx-check panel without browser submission proof\."/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"forms-package-metrics-helper-freshness-path-arrays",\s*&\["\/"\],\s*"core\/src\/ecosystem\/project_check\/forms_dx_check\.rs",\s*"cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib"[\s\S]*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /"forms-package-metrics-helper-freshness-path-arrays"[\s\S]*"Forms lower dx-check helper freshness fixture"[\s\S]*"forms_receipt_hash_refresh_current"[\s\S]*"forms_receipt_hash_refresh_stale"[\s\S]*"forms_receipt_hash_refresh_missing"[\s\S]*"forms_hash_mismatch stays byte-derived"[\s\S]*"forms\/react-hook-form source-only Studio discovery"/,
  );
  assert.match(
    studioManifest,
    /"dx run --test \.\\\\benchmarks\\\\forms-dx-check-package-lane-panel\.test\.ts"[\s\S]*"Validate the source-only Forms package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without browser submission proof\."/,
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"forms-generated-starter-materialization"[\s\S]*"forms-package-metrics-helper-freshness-path-arrays"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "forms-generated-starter-materialization"[\s\S]*"package_id": "forms\/react-hook-form"[\s\S]*"fixture_path": "docs\/packages\/forms\.source-guard-runbook\.json"[\s\S]*"schema": "dx\.forge\.package\.source_guard_runbook_fixture"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "forms-package-metrics-helper-freshness-path-arrays"[\s\S]*"package_id": "forms\/react-hook-form"[\s\S]*"fixture_path": "docs\/packages\/forms\.source-guard-runbook\.json"[\s\S]*"schema": "dx\.forge\.package\.source_guard_runbook_fixture"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"forms-generated-starter-materialization",\s*"The generated starter preserves the Forms package-lane row, helper freshness markers, and package-scoped dx-check panel without browser submission proof\.",\s*"benchmarks\/forms-dx-check-package-lane-panel\.test\.ts",\s*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"forms-package-metrics-helper-freshness-path-arrays",\s*"The lower-level Forms dx-check producer reports helper freshness path arrays while keeping forms_hash_mismatch byte-derived\.",\s*"cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib",\s*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"dx run --test \.\\\\benchmarks\\\\forms-dx-check-package-lane-panel\.test\.ts",\s*"Validate the source-only Forms package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without browser submission proof\.",\s*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib",\s*"Validate the source-only Forms lower dx-check helper freshness metrics and path-array attribution without browser submission proof\.",\s*"docs\/packages\/forms\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(frameworkDoc, /Lane 6 uses the official front-facing package name `Forms`/);
  assert.match(frameworkDoc, /forms-generated-starter-materialization/);
  assert.match(frameworkDoc, /forms-package-metrics-helper-freshness-path-arrays/);
  assert.match(frameworkDoc, /source_guard_index/);
  assert.match(frameworkDoc, /source_guard_runbook_index/);
  assert.match(frameworkDoc, /structured `fixture_path` metadata/);
  assert.match(frameworkDoc, /without claiming browser submission proof/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Forms Studio source-guard\/runbook entry/);
    assert.match(source, /forms-generated-starter-materialization/);
    assert.match(source, /forms-package-metrics-helper-freshness-path-arrays/);
    assert.match(
      source,
      /dx run --test \.\\benchmarks\\forms-dx-check-package-lane-panel\.test\.ts/,
    );
    assert.match(
      source,
      /cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib/,
    );
    assert.match(source, /without claiming browser submission proof/);
  }
});

test("Forms source-guard runbook fixture mirrors the Studio manifest", () => {
  const fixture = readJson("docs/packages/forms.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/forms-react-hook-form.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Forms");
  assert.equal(fixture.package.package_id, "forms/react-hook-form");
  assert.equal(fixture.package.upstream_package, "react-hook-form");
  assert.equal(fixture.package.upstream_version, "7.75.0");
  assert.equal(
    fixture.package.source_mirror,
    "G:/WWW/inspirations/react-hook-form",
  );

  assert.equal(fixture.guard.id, "forms-generated-starter-materialization");
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/forms-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\forms-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "Forms generated-starter materialization guard",
    'data-dx-check-package-lane-row="forms/react-hook-form"',
    'data-dx-token-scope="forms/react-hook-form"',
    "forms:receipt-hash-refresh",
    "docs/packages/forms.source-guard-runbook.json",
    "without claiming browser submission proof",
    "forms/react-hook-form source-only Studio discovery",
  ]) {
    assert.ok(fixture.guard.proves.includes(proof), `missing proof ${proof}`);
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll('"', '\\"')),
      `Studio manifest missing ${proof}`,
    );
  }

  assert.equal(
    fixture.runbook.contract.evidence_field,
    "benchmarks/forms-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only Forms package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without browser submission proof.",
  );

  for (const marker of [
    "data-dx-check-package-lane-row",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-check-package-lane-hash-refresh-stale-file-list",
    "data-dx-check-package-lane-hash-refresh-missing-file-list",
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(
    fixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  );
  assert.equal(
    fixture.receipt.hash_helper,
    "examples/template/forms-receipt-hashes.ts",
  );
  assert.equal(fixture.receipt.zed_visibility, "forms:receipt-hash-refresh");
  assert.equal(
    fixture.receipt.source_guard_runbook_fixture,
    "docs/packages/forms.source-guard-runbook.json",
  );
  assert.equal(fixture.receipt.tracked_by_receipt_hash_helper, true);
  assert.equal(fixture.receipt.tracked_file_count, 7);
  assert.equal(
    fixture.preview_manifest.generated_file,
    "public/preview-.dx/build-cache/manifest.json",
  );
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(
    fixture.preview_manifest.fixture,
    "docs/packages/forms.source-guard-runbook.json",
  );
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.equal(
    fixture.studio_manifest.source_guard_entry,
    "source_guard_index[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.runbook_fixture_paths,
    "source_guard_runbook_index[].fixture_paths[]",
  );
  assert.equal(
    fixture.studio_manifest.contract_entry,
    "source_guard_runbook_index[].contracts[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.command_entry,
    "source_guard_runbook_index[].commands[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.fixture_path,
    "docs/packages/forms.source-guard-runbook.json",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /browser submission proof/);

  for (const source of [
    studioManifest,
    packageDoc,
    frameworkDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /forms\.source-guard-runbook\.json/);
    assert.match(source, /forms-generated-starter-materialization/);
    assert.match(source, /without claiming browser submission proof/);
  }
});
