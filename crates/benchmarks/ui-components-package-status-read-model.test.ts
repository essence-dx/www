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
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

function assertHashMapMatchesFiles(fileHashes) {
  assert.ok(fileHashes, "missing file hash map");

  for (const [relativePath, expectedHash] of Object.entries(fileHashes)) {
    assert.equal(
      expectedHash,
      sha256(relativePath),
      `${relativePath} hash does not match current source`,
    );
  }
}

test("UI Components exposes receipt-backed package-status visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/ui-components.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const uiVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "shadcn/ui/button",
  );

  assert.ok(uiVisibility, "UI Components visibility row is missing");
  assert.equal(uiVisibility.official_package_name, "UI Components");
  assert.equal(uiVisibility.upstream_package, "shadcn-ui");
  assert.equal(uiVisibility.upstream_version, "0.0.1");
  assert.equal(
    uiVisibility.source_mirror,
    "G:/WWW/inspirations/shadcn-ui; G:/WWW/inspirations/radix-primitives",
  );
  assert.equal(uiVisibility.status, "present");
  assert.equal(uiVisibility.receipt_status, "present");
  assert.equal(
    uiVisibility.package_receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
  );
  assert.deepEqual(uiVisibility.status_vocabulary, statusVocabulary);

  for (const surfaceId of [
    "ui-components-source-primitives",
    "ui-components-dashboard-controls",
    "ui-components-runtime-controls",
  ]) {
    assert.ok(
      uiVisibility.selected_surfaces.some(
        (surface) => surface.surface_id === surfaceId,
      ),
      `${surfaceId} missing from UI Components visibility row`,
    );
  }

  const sourceMarkers = uiVisibility.selected_surfaces.flatMap(
    (surface) => surface.source_markers,
  );

  for (const marker of [
    'data-dx-package="shadcn/ui/button"',
    'data-dx-component="shadcn-dashboard-controls"',
    'data-dx-component="shadcn-dashboard-controls-runtime"',
    'data-dx-dashboard-workflow="operator-controls"',
    'data-dx-shadcn-dashboard-action="set-density"',
    'data-dx-shadcn-dashboard-action="select-queue"',
    'data-slot="button"',
    'data-slot="item"',
    'data-slot="field"',
  ]) {
    assert.ok(sourceMarkers.includes(marker), `${marker} missing from markers`);
  }

  for (const metric of [
    "ui_components_receipt_present",
    "ui_components_receipt_stale",
    "ui_components_missing_receipt",
    "ui_components_blocked_surface",
    "ui_components_unsupported_surface",
    "ui_components_hash_manifest_present",
    "ui_components_hash_mismatch",
  ]) {
    assert.ok(
      uiVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from UI Components visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const uiComponentsPackageVisibility/);
  assert.match(statusSource, /uiComponentsPackageVisibility/);
  assert.match(statusSource, /uiComponentsVisibility/);
  assert.match(packageDoc, /package-status read model/);
  assert.match(packageDoc, /ui_components_receipt_present/);
  assert.equal(receipt.official_package_name, "UI Components");
  assert.equal(receipt.package_id, "shadcn/ui/button");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(Array.isArray(receipt.file_hashes), "receipt file_hashes must be an array");
  assert.ok(receipt.file_hashes.length >= 4, "receipt should hash selected UI surfaces");
  for (const entry of receipt.file_hashes) {
    assert.equal(entry.sha256, sha256(entry.path), `${entry.path} receipt hash mismatch`);
  }
  assert.equal(uiVisibility.source_hashes.algorithm, "sha256");
  assertHashMapMatchesFiles(uiVisibility.source_hashes.files);

  for (const surface of uiVisibility.selected_surfaces.filter((entry) =>
    ["ui-components-source-primitives", "ui-components-dashboard-controls"].includes(
      entry.surface_id,
    ),
  )) {
    assert.equal(surface.hash_algorithm, "sha256", `${surface.surface_id} missing hash algorithm`);
    assertHashMapMatchesFiles(surface.file_hashes);
  }
  assert.match(readModel, /hashAlgorithm: "sha256"/);
  assert.match(readModel, /fileHashes:/);
  assert.match(packageDoc, /hash-backed stale detection/);

  const hashRefresh = uiVisibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "UI Components hash refresh status is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(
    hashRefresh.helper_path,
    "examples/template/ui-components-receipt-hashes.ts",
  );
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ui-components-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, uiVisibility.package_receipt_path);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, receipt.file_hashes.length);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.deepEqual(hashRefresh.missing_files, []);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "ui-components:receipt-hash-refresh");

  const helper = spawnSync(
    process.execPath,
    [
      "examples/template/ui-components-receipt-hashes.ts",
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
  assert.equal(helperReport.package_id, "shadcn/ui/button");
  assert.equal(helperReport.official_package_name, "UI Components");
  assert.equal(helperReport.status, hashRefresh.status);
  assert.equal(helperReport.tracked_file_count, hashRefresh.tracked_file_count);
  assert.equal(helperReport.stale_file_count, hashRefresh.stale_file_count);
  assert.deepEqual(helperReport.stale_files, hashRefresh.stale_files);
  assert.equal(helperReport.missing_file_count, hashRefresh.missing_file_count);
  assert.deepEqual(helperReport.missing_files, hashRefresh.missing_files);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);

  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /ui-components:receipt-hash-refresh/);
  assert.ok(
    status.zed_receipt_surfaces.includes("ui-components:receipt-hash-refresh"),
    "UI Components receipt-hash refresh Zed surface is missing",
  );
  assert.match(packageDoc, /receipt_hash_refresh/);
});
