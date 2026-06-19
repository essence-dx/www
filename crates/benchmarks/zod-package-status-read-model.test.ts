const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(root, relativePath)))
    .digest("hex");
}

test("Validation & Schemas exposes package-lane dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/validation-zod.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "validation/zod",
  );

  assert.ok(visibility, "Validation & Schemas visibility row is missing");
  assert.equal(visibility.official_package_name, "Validation & Schemas");
  assert.equal(visibility.upstream_package, "zod");
  assert.equal(visibility.upstream_version, "4.4.3");
  assert.equal(visibility.source_mirror, "G:/WWW/inspirations/zod");
  assert.equal(visibility.status, "present");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(
    visibility.package_receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json",
  );
  assert.deepEqual(visibility.status_vocabulary, statusVocabulary);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "validation-schemas:dashboard-settings-validation",
    ),
    "Validation & Schemas Zed receipt surface is missing",
  );

  for (const surfaceId of [
    "dashboard-settings-validation",
    "starter-dashboard-settings-validator",
    "launch-package-catalog-validation",
  ]) {
    assert.ok(
      visibility.selected_surfaces.some(
        (surface) => surface.surface_id === surfaceId,
      ),
      `${surfaceId} missing from Validation & Schemas visibility row`,
    );
  }

  const dashboardSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "dashboard-settings-validation",
  );
  assert.ok(dashboardSurface, "dashboard settings surface is missing");
  assert.equal(dashboardSurface.hash_algorithm, "sha256");
  assert.ok(
    dashboardSurface.file_hashes,
    "dashboard settings surface should carry file_hashes",
  );

  for (const marker of [
    'data-dx-package="validation/zod"',
    'data-dx-component="launch-settings-validation-summary"',
    'data-dx-zod-dashboard-fieldset="editable-settings"',
    'data-dx-zod-dashboard-receipt-api="createDxDashboardSettingsReceipt"',
    'data-dx-zod-field-errors-api="z.flattenError"',
  ]) {
    assert.ok(
      dashboardSurface.source_markers.includes(marker),
      `${marker} missing from Validation & Schemas dashboard markers`,
    );
  }

  for (const metric of [
    "validation_schemas_receipt_present",
    "validation_schemas_receipt_stale",
    "validation_schemas_missing_receipt",
    "validation_schemas_blocked_surface",
    "validation_schemas_unsupported_surface",
    "validation_schemas_hash_manifest_present",
    "validation_schemas_hash_mismatch",
    "validation_schemas_dx_style_compatibility_present",
    "validation_schemas_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Validation & Schemas visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const validationSchemasPackageVisibility/);
  assert.match(statusSource, /validationSchemasPackageVisibility/);
  assert.match(statusSource, /validationSchemasVisibility/);
  assert.match(packageDoc, /package-status read model/);
  assert.match(packageDoc, /validation_schemas_receipt_present/);
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.file_hashes, "receipt should expose file_hashes");
  assert.ok(receipt.files, "receipt should expose tracked files");
  assert.ok(
    receipt.file_hashes["tools/launch/materialize-www-template.ts"],
    "materializer must be hash-backed so generated-starter metadata drift is receipt-visible",
  );
  assert.deepEqual(
    Object.keys(receipt.file_hashes).sort(),
    [...receipt.files].sort(),
    "receipt files and file_hashes should describe the same tracked files",
  );
  for (const [filePath, expectedHash] of Object.entries(receipt.file_hashes)) {
    assert.equal(expectedHash, sha256(filePath), `${filePath} hash is stale`);
  }
  assert.equal(visibility.source_hashes.algorithm, "sha256");
  assert.equal(
    visibility.source_hashes.stale_receipt_policy,
    "dx-check marks Validation & Schemas stale when any selected surface file hash differs from this receipt manifest.",
  );
  assert.deepEqual(visibility.source_hashes.files, receipt.file_hashes);
  assert.match(readModel, /sourceHashes: \{/);
  assert.match(readModel, /hashAlgorithm: "sha256"/);
  assert.match(readModel, /fileHashes: validationSchemasFileHashes/);
  assert.match(packageDoc, /validation_schemas_hash_manifest_present/);
  assert.match(packageDoc, /validation_schemas_hash_mismatch/);
  assert.equal(
    visibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(visibility.dx_style_compatibility.status, "present");
  assert.equal(
    visibility.dx_style_compatibility.token_source,
    "styles/globals.css",
  );
  assert.ok(
    visibility.dx_style_compatibility.visible_surfaces.includes(
      "zod-dashboard-settings-form",
    ),
  );
  assert.match(readModel, /dxStyleCompatibility/);
  assert.match(readModel, /data-dx-style-surface="validation-schemas"/);
  assert.match(packageDoc, /dx\.forge\.package\.dx_style_compatibility/);

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Validation & Schemas hash refresh status is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(
    hashRefresh.helper_path,
    "examples/template/validation-schemas-receipt-hashes.ts",
  );
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/validation-schemas-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/validation-schemas-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.preview_manifest_materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(hashRefresh.receipt_path, visibility.package_receipt_path);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(
    hashRefresh.tracked_file_count,
    Object.keys(receipt.file_hashes).length,
  );
  assert.deepEqual(hashRefresh.current_files, hashRefresh.tracked_files);
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.deepEqual(hashRefresh.missing_files, []);
  assert.deepEqual(hashRefresh.stale_mirror_files, []);
  assert.deepEqual(hashRefresh.missing_mirror_files, []);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "validation-schemas:receipt-hash-refresh");

  const helper = spawnSync(
    process.execPath,
    [
      "tools/launch/run-template-receipt-helper.js",
      "examples/template/validation-schemas-receipt-hashes.ts",
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "validation/zod");
  assert.equal(helperReport.status, hashRefresh.status);
  assert.equal(helperReport.tracked_file_count, hashRefresh.tracked_file_count);
  assert.deepEqual(helperReport.current_files, hashRefresh.current_files);
  assert.deepEqual(helperReport.stale_files, hashRefresh.stale_files);
  assert.deepEqual(helperReport.missing_files, hashRefresh.missing_files);
  assert.deepEqual(helperReport.stale_mirror_files, hashRefresh.stale_mirror_files);
  assert.deepEqual(
    helperReport.missing_mirror_files,
    hashRefresh.missing_mirror_files,
  );
  assert.equal(helperReport.stale_file_count, hashRefresh.stale_file_count);
  assert.equal(helperReport.missing_file_count, hashRefresh.missing_file_count);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);

  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(readModel, /currentFiles/);
  assert.match(readModel, /staleMirrorFiles/);
  assert.match(readModel, /validation-schemas:receipt-hash-refresh/);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "validation-schemas:receipt-hash-refresh",
    ),
    "Validation & Schemas receipt-hash refresh Zed surface is missing",
  );
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.includes(
      "dashboard-settings-validation",
    ),
  );
});

test("Validation & Schemas package visibility is consumed by Rust dx-check", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageCheck = read(
    "core/src/ecosystem/project_check/validation_schemas_dx_check.rs",
  );
  const packageDoc = read("docs/packages/validation-zod.md");

  assert.match(projectCheck, /mod validation_schemas_dx_check;/);
  assert.match(
    projectCheck,
    /use validation_schemas_dx_check::forge_validation_schemas_package_metrics;/,
  );
  assert.match(
    projectCheck,
    /forge_validation_schemas_package_metrics\(root, &manifest\)/,
  );

  for (const expectedSource of [
    'const VALIDATION_SCHEMAS_PACKAGE_ID: &str = "validation/zod";',
    'const VALIDATION_SCHEMAS_OFFICIAL_NAME: &str = "Validation & Schemas";',
    'const VALIDATION_SCHEMAS_PACKAGE_STATUS: &str = ".dx/forge/package-status.json";',
    'const VALIDATION_SCHEMAS_DASHBOARD_RECEIPT: &str =',
    'pub(super) fn forge_validation_schemas_package_metrics',
    '"validation_schemas_package_present"',
    '"validation_schemas_receipt_present"',
    '"validation_schemas_receipt_stale"',
    '"validation_schemas_missing_receipt"',
    '"validation_schemas_blocked_surface"',
    '"validation_schemas_unsupported_surface"',
    '"validation_schemas_hash_manifest_present"',
    '"validation_schemas_hash_mismatch"',
    '"validation_schemas_dx_style_compatibility_present"',
    '"validation_schemas_dx_style_compatibility_missing"',
    '"validation-schemas-missing-package-status"',
    '"validation-schemas-missing-receipt"',
    '"validation-schemas-stale-receipt"',
    '"validation-schemas-blocked-surface"',
    '"validation-schemas-unsupported-surface"',
    '"validation-schemas-hash-mismatch"',
    '"validation-schemas-missing-dx-style-compatibility"',
    "fn dx_style_compatibility_is_present",
    'json_text(surface, &["hash_algorithm"]) == Some("sha256")',
    '.get("file_hashes")',
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "count_sha256_file_hash_mismatches(root, surface)",
    "validation_schemas_hash_mismatch_flips_when_selected_file_changes",
    "validation_schemas_dx_style_missing_metric_and_finding_flip",
  ]) {
    assert.match(
      packageCheck,
      new RegExp(expectedSource.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${expectedSource} missing from Validation & Schemas dx-check module`,
    );
  }

  assert.doesNotMatch(packageCheck, /fn count_hash_mismatches/);
  assert.doesNotMatch(packageCheck, /fn sha256_project_file/);

  assert.match(
    packageDoc,
    /Rust `dx check` consumes `validation_schemas_package_present`/,
  );
  assert.match(packageDoc, /validation-schemas-stale-receipt/);
});
