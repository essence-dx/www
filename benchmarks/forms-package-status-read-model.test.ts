const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];
const formMetrics = [
  "forms_receipt_present",
  "forms_receipt_stale",
  "forms_missing_receipt",
  "forms_blocked_surface",
  "forms_unsupported_surface",
  "forms_hash_manifest_present",
  "forms_hash_mismatch",
];
const studioManifestPath = "dx-www/src/cli/studio_manifest.rs";

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

function assertHashManifest(surface, expectedPaths) {
  assert.equal(surface.hash_algorithm, "sha256");
  assert.equal(typeof surface.file_hashes, "object");
  for (const relativePath of expectedPaths) {
    assert.equal(
      surface.file_hashes[relativePath],
      sha256(relativePath),
      `${surface.surface_id} has a stale hash for ${relativePath}`,
    );
  }
}

function runFormsHashRefresh() {
  const result = spawnSync(
    process.execPath,
    [
      "tools/launch/run-template-receipt-helper.js",
      "examples/template/forms-receipt-hashes.ts",
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return JSON.parse(result.stdout);
}

function readModelExport(source, exportName) {
  const start = source.indexOf(`export const ${exportName} = {`);
  assert.notEqual(start, -1, `${exportName} export is missing`);
  const nextExport = source.indexOf("\n\nexport const ", start + 1);
  return source.slice(start, nextExport === -1 ? undefined : nextExport);
}

test("Forms receipt visibility is consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageLock = read("examples/template/forge-package-lock.ts");
  const packageDoc = read("docs/packages/forms-react-hook-form.md");
  const hashRefreshReport = runFormsHashRefresh();

  const formsVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "forms/react-hook-form",
  );
  const formsReadModel = readModelExport(readModel, "formsPackageVisibility");

  assert.ok(formsVisibility, "Forms visibility row is missing");
  assert.equal(formsVisibility.official_package_name, "Forms");
  assert.equal(formsVisibility.upstream_package, "react-hook-form");
  assert.equal(formsVisibility.upstream_version, "7.75.0");
  assert.equal(formsVisibility.status, "present");
  assert.equal(formsVisibility.receipt_status, "present");
  assert.equal(
    formsVisibility.package_receipt_path,
    ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  );
  assert.deepEqual(formsVisibility.receipt_hash_refresh, {
    schema: "dx.forge.package.receipt_hash_refresh",
    status: hashRefreshReport.status,
    helper_path: hashRefreshReport.helper_path,
    check_command: hashRefreshReport.check_command,
    write_command: hashRefreshReport.write_command,
    json_check_command: hashRefreshReport.json_check_command,
    source_guard_runbook_fixture:
      hashRefreshReport.source_guard_runbook_fixture,
    receipt_path: hashRefreshReport.receipt_path,
    hash_algorithm: hashRefreshReport.hash_algorithm,
    tracked_file_count: hashRefreshReport.tracked_file_count,
    tracked_files: hashRefreshReport.tracked_files,
    current_files: hashRefreshReport.current_files,
    stale_files: hashRefreshReport.stale_files,
    missing_files: hashRefreshReport.missing_files,
    stale_mirror_files: hashRefreshReport.stale_mirror_files,
    missing_mirror_files: hashRefreshReport.missing_mirror_files,
    stale_file_count: hashRefreshReport.stale_file_count,
    missing_file_count: hashRefreshReport.missing_file_count,
    runtime_execution: hashRefreshReport.runtime_execution,
    secret_access: hashRefreshReport.secret_access,
    zed_visibility: hashRefreshReport.zed_visibility,
    runtime_limitations: hashRefreshReport.runtime_limitations,
  });
  assert.match(formsReadModel, /receiptHashRefresh: \{/);
  assert.match(formsReadModel, /schema: "dx\.forge\.package\.receipt_hash_refresh"/);
  assert.match(formsReadModel, /status: "current"/);
  assert.match(formsReadModel, /helperPath: "examples\/template\/forms-receipt-hashes\.ts"/);
  assert.match(
    formsReadModel,
    /jsonCheckCommand:\s*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/forms-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    formsReadModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/forms\.source-guard-runbook\.json"/,
  );
  assert.match(formsReadModel, /zedVisibility: "forms:receipt-hash-refresh"/);
  assert.match(formsReadModel, /trackedFiles: \[[\s\S]*"dx-www\/src\/cli\/studio_manifest\.rs"/);
  assert.match(formsReadModel, /currentFiles: \[[\s\S]*"dx-www\/src\/cli\/studio_manifest\.rs"/);
  assert.match(formsReadModel, /staleFiles: \[\]/);
  assert.match(formsReadModel, /missingFiles: \[\]/);
  assert.match(formsReadModel, /staleMirrorFiles: \[\]/);
  assert.match(formsReadModel, /missingMirrorFiles: \[\]/);
  assert.deepEqual(hashRefreshReport.stale_files, []);
  assert.deepEqual(hashRefreshReport.missing_files, []);
  assert.deepEqual(hashRefreshReport.stale_mirror_files, []);
  assert.deepEqual(hashRefreshReport.missing_mirror_files, []);
  assert.ok(
    hashRefreshReport.current_files.includes(studioManifestPath),
    "Forms helper report should attribute Studio manifest as current",
  );
  assert.deepEqual(formsVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.map((statusName) =>
      statusName.replaceAll(" ", "-"),
    ),
    statusVocabulary,
  );
  assert.equal(receipt.hash_algorithm, "sha256");
  for (const entry of receipt.file_hashes) {
    assert.equal(entry.sha256.toLowerCase(), sha256(entry.path));
  }

  const launchLeadSurface = formsVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "template-lead-form",
  );
  assert.ok(launchLeadSurface, "Forms launch lead form surface is missing");
  assert.equal(
    launchLeadSurface.receipt_path,
    ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
  );
  assert.ok(
    launchLeadSurface.files.includes("components/template-app/template-lead-form.tsx"),
    "Forms launch lead form file is missing",
  );
  assert.ok(
    launchLeadSurface.source_markers.includes(
      'data-dx-component="template-lead-form"',
    ) &&
      launchLeadSurface.source_markers.includes(
        'data-dx-edit-id="launch.lead-form"',
      ),
    "Forms launch lead form surface is missing",
  );
  assertHashManifest(launchLeadSurface, [
    "examples/template/template-lead-form.tsx",
    "examples/template/template-route-contract.ts",
  ]);

  const providerFieldsSurface = formsVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "forms-provider-fields",
  );
  assert.ok(providerFieldsSurface, "Forms provider/fields surface is missing");
  assert.ok(
      providerFieldsSurface.files.includes("lib/forms/react-hook-form/form.tsx") &&
      providerFieldsSurface.files.includes("lib/forms/react-hook-form/fields.tsx") &&
      providerFieldsSurface.files.includes("lib/forms/react-hook-form/dry-run-receipt.ts") &&
      providerFieldsSurface.files.includes("lib/forms/react-hook-form/resolver.ts") &&
      providerFieldsSurface.source_markers.includes("DxHookForm") &&
      providerFieldsSurface.source_markers.includes("DxSelectField") &&
      providerFieldsSurface.source_markers.includes("createDxFormDryRunReceipt") &&
      providerFieldsSurface.source_markers.includes("createDxZodResolver"),
    "Forms provider/fields surface is missing",
  );
  assertHashManifest(providerFieldsSurface, [
    "docs/packages/forms-react-hook-form.md",
    "core/src/ecosystem/forge_react_hook_form.rs",
  ]);

  const sourceGuardRunbookSurface = formsVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "forms-source-guard-runbook",
  );
  assert.ok(
    sourceGuardRunbookSurface,
    "Forms source-guard runbook surface is missing",
  );
  assert.ok(
    sourceGuardRunbookSurface.files.includes(
      "docs/packages/forms.source-guard-runbook.json",
    ) &&
      sourceGuardRunbookSurface.source_markers.includes(
        "forms-generated-starter-materialization",
      ) &&
      sourceGuardRunbookSurface.source_markers.includes(
        "source_guard_runbook_index",
      ),
    "Forms source-guard runbook surface is incomplete",
  );
  assertHashManifest(sourceGuardRunbookSurface, [
    "docs/packages/forms.source-guard-runbook.json",
  ]);

  const studioManifestSurface = formsVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "forms-studio-manifest",
  );
  assert.ok(
    studioManifestSurface,
    "Forms Studio manifest selected surface is missing",
  );
  assert.ok(
    studioManifestSurface.files.includes(studioManifestPath) &&
      studioManifestSurface.source_markers.includes(
        "studio_source_guard_with_fixture",
      ) &&
      studioManifestSurface.source_markers.includes(
        "source_guard_fixture_paths_for_route",
      ) &&
      studioManifestSurface.source_markers.includes(
        "forms-generated-starter-materialization",
      ),
    "Forms Studio manifest surface is incomplete",
  );
  assertHashManifest(studioManifestSurface, [studioManifestPath]);
  assert.match(formsReadModel, /surfaceId: "forms-studio-manifest"/);
  assert.match(formsReadModel, /"dx-www\/src\/cli\/studio_manifest\.rs"/);

  for (const metric of formMetrics) {
    assert.ok(
      formsVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Forms visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const formsPackageVisibility/);
  assert.match(readModel, /packageLaneVisibility: \[[\s\S]*formsPackageVisibility/);
  assert.match(statusSource, /formsPackageVisibility/);
  assert.match(statusSource, /formsVisibility: formsPackageVisibility/);
  assert.match(packageLock, /checkFormsPackageVisibility/);
  assert.match(packageLock, /forms_receipt_present/);
  assert.match(packageDoc, /shared dx-check\/Zed package-status read model/);
  assert.ok(status.zed_receipt_surfaces.includes("forms:template-lead-form"));
  assert.ok(status.zed_receipt_surfaces.includes("forms:provider-fields"));
  assert.ok(status.zed_receipt_surfaces.includes("forms:source-guard-runbook"));
  assert.ok(status.zed_receipt_surfaces.includes("forms:studio-manifest"));
});
