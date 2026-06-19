const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json";
const templateReadinessReceiptPath =
  "examples/template/.dx/forge/template-readiness/database-api.json";
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

function readHashRefreshReport() {
  const helperPath = path.join(
    root,
    "examples/template/type-safe-api-receipt-hashes.ts",
  );
  const result = spawnSync(
    process.execPath,
    [helperPath, "--check", "--json"],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return JSON.parse(result.stdout);
}

test("Type-Safe API receipt visibility is wired into the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const templateReadinessReceipt = readJson(templateReadinessReceiptPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/api-trpc.md");

  const typeSafeApiVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "api/trpc",
  );

  assert.ok(typeSafeApiVisibility, "Type-Safe API visibility row is missing");
  assert.equal(typeSafeApiVisibility.official_package_name, "Type-Safe API");
  assert.equal(typeSafeApiVisibility.upstream_package, "@trpc/server");
  assert.equal(typeSafeApiVisibility.upstream_version, "11.17.0");
  assert.equal(typeSafeApiVisibility.source_mirror, "G:/WWW/inspirations/trpc");
  assert.equal(typeSafeApiVisibility.status, "present");
  assert.equal(typeSafeApiVisibility.receipt_status, "present");
  assert.equal(typeSafeApiVisibility.package_receipt_path, receiptPath);
  const hashRefreshReport = readHashRefreshReport();
  assert.deepEqual(typeSafeApiVisibility.receipt_hash_refresh, {
    schema: hashRefreshReport.schema,
    status: hashRefreshReport.status,
    helper_path: hashRefreshReport.helper_path,
    check_command: hashRefreshReport.check_command,
    write_command: hashRefreshReport.write_command,
    json_check_command: hashRefreshReport.json_check_command,
    receipt_path: hashRefreshReport.receipt_path,
    hash_algorithm: hashRefreshReport.hash_algorithm,
    source_guard_runbook_fixture: hashRefreshReport.source_guard_runbook_fixture,
    preview_manifest_materializer: hashRefreshReport.preview_manifest_materializer,
    tracked_file_count: hashRefreshReport.tracked_file_count,
    stale_file_count: hashRefreshReport.stale_file_count,
    missing_file_count: hashRefreshReport.missing_file_count,
    tracked_files: hashRefreshReport.tracked_files,
    current_files: hashRefreshReport.current_files,
    stale_files: hashRefreshReport.stale_files,
    missing_files: hashRefreshReport.missing_files,
    stale_mirror_files: hashRefreshReport.stale_mirror_files,
    missing_mirror_files: hashRefreshReport.missing_mirror_files,
    mirror_problem_count: hashRefreshReport.mirror_problem_count,
    runtime_execution: hashRefreshReport.runtime_execution,
    secret_access: hashRefreshReport.secret_access,
    zed_visibility: hashRefreshReport.zed_visibility,
    runtime_limitations: hashRefreshReport.runtime_limitations,
  });
  assert.equal(typeSafeApiVisibility.source_hashes.algorithm, "sha256");
  assert.deepEqual(typeSafeApiVisibility.source_hashes.files, receipt.file_hashes);
  assert.deepEqual(typeSafeApiVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.equal(templateReadinessReceipt.schema, "dx.forge.template_readiness.package");
  assert.equal(templateReadinessReceipt.lane_number, 4);
  assert.equal(templateReadinessReceipt.lane_name, "Database + API");
  assert.equal(templateReadinessReceipt.runtime_proof, false);
  assert.equal(templateReadinessReceipt.network_calls, false);
  assert.equal(templateReadinessReceipt.hosted_credentials, false);
  assert.equal(
    templateReadinessReceipt.cache_evidence.manifest_source,
    "package-status-current-manifests",
  );
  assert.equal(
    templateReadinessReceipt.cache_evidence.manifest_caveat_id,
    "physical-cache-matches-current-manifests",
  );
  assert.equal(
    templateReadinessReceipt.cache_evidence.current_manifest_count,
    status.cache.current_manifest_count,
  );
  assert.equal(
    templateReadinessReceipt.cache_evidence.physical_manifest_count,
    status.cache.physical_manifest_count,
  );
  assert.equal(
    templateReadinessReceipt.cache_evidence.stale_physical_manifest_count,
    status.cache.stale_physical_manifest_count,
  );
  assert.ok(
    receipt.file_hashes[templateReadinessReceiptPath],
    "Type-Safe API receipt is missing the Database + API template-readiness receipt hash",
  );
  for (const [filePath, expectedHash] of Object.entries(receipt.file_hashes)) {
    assert.equal(
      expectedHash,
      sha256(filePath),
      `${filePath} hash is stale in Type-Safe API receipt`,
    );
  }

  assert.ok(
    typeSafeApiVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "type-safe-api-template-readiness-receipt" &&
        surface.files.includes(templateReadinessReceiptPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes[templateReadinessReceiptPath] ===
          receipt.file_hashes[templateReadinessReceiptPath] &&
        surface.source_markers.includes("dx.forge.template_readiness.package") &&
        surface.source_markers.includes("physical-cache-matches-current-manifests"),
    ),
    "Type-Safe API template-readiness receipt surface is missing",
  );

  for (const surface of receipt.dx_check_visibility.monitored_surfaces) {
    assert.ok(
      typeSafeApiVisibility.selected_surfaces.some(
        (entry) =>
          entry.surface_id === surface.id &&
          entry.status === surface.status &&
          entry.receipt_path === surface.receipt_path &&
          entry.files.includes(surface.materialized_file) &&
          entry.hash_algorithm === "sha256" &&
          Object.keys(entry.file_hashes).length > 0,
      ),
      `${surface.id} missing from Type-Safe API package-status visibility`,
    );
  }

  for (const surface of typeSafeApiVisibility.selected_surfaces) {
    assert.equal(surface.hash_algorithm, "sha256");
    for (const [filePath, expectedHash] of Object.entries(surface.file_hashes)) {
      assert.equal(
        expectedHash,
        receipt.file_hashes[filePath],
        `${filePath} hash missing from Type-Safe API receipt`,
      );
    }
  }

  for (const marker of [
    'data-dx-package="api/trpc"',
    'data-dx-component="launch-trpc-api-dashboard-workflow"',
    'data-dx-component="dashboard-trpc-workflow"',
    'data-dx-trpc-action="check-health"',
    'data-trpc-interaction="local-launch-event-mutation"',
    "fetchRequestHandler",
  ]) {
    assert.ok(
      typeSafeApiVisibility.selected_surfaces
        .flatMap((surface) => surface.source_markers)
        .includes(marker),
      `${marker} missing from Type-Safe API visibility markers`,
    );
  }

  for (const metric of receipt.dx_check_visibility.dx_check_metrics) {
    assert.ok(
      typeSafeApiVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Type-Safe API visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  for (const zedSurface of [
    "type-safe-api:trpc-launch-dashboard-workflow",
    "type-safe-api:trpc-starter-dashboard-workflow",
    "type-safe-api:trpc-route-handler",
  ]) {
    assert.ok(
      status.zed_receipt_surfaces.includes(zedSurface),
      `${zedSurface} missing from Zed receipt surfaces`,
    );
    assert.match(readModel, new RegExp(zedSurface));
  }

  assert.match(readModel, /export const typeSafeApiPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh:\s*{/);
  assert.match(
    readModel,
    /helperPath:\s*"examples\/template\/type-safe-api-receipt-hashes\.ts"/,
  );
  assert.match(readModel, /zedVisibility:\s*"type-safe-api:receipt-hash-refresh"/);
  assert.match(readModel, /sourceGuardRunbookFixture:\s*"docs\/packages\/api-trpc\.source-guard-runbook\.json"/);
  assert.match(readModel, /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/);
  assert.match(readModel, /trackedFiles:\s*\[/);
  assert.match(readModel, /currentFiles:\s*\[/);
  assert.match(readModel, /staleFiles:\s*\[\s*\]/);
  assert.match(readModel, /missingFiles:\s*\[\s*\]/);
  assert.match(readModel, /staleMirrorFiles:\s*\[\s*\]/);
  assert.match(readModel, /missingMirrorFiles:\s*\[\s*\]/);
  assert.match(readModel, /mirrorProblemCount:\s*0/);
  assert.match(readModel, /runtimeExecution:\s*false/);
  assert.match(readModel, /secretAccess:\s*false/);
  assert.match(
    readModel,
    /packageLaneVisibility:\s*\[[\s\S]*typeSafeApiPackageVisibility[\s\S]*\]/,
  );
  assert.match(statusSource, /typeSafeApiPackageVisibility/);
  assert.match(statusSource, /typeSafeApiVisibility: typeSafeApiPackageVisibility/);
  assert.match(packageDoc, /shared launch package-status read model/i);
  assert.match(packageDoc, /file_hashes/);
  assert.match(packageDoc, /type_safe_api_hash_manifest_present/);
  assert.equal(receipt.official_dx_package_name, "Type-Safe API");
});
