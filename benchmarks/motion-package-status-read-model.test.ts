const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
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

test("Motion & Animation exposes hash-backed package-status dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/animation-motion.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const motionVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "animation/motion",
  );

  assert.ok(motionVisibility, "Motion & Animation visibility row is missing");
  assert.equal(motionVisibility.official_package_name, "Motion & Animation");
  assert.equal(motionVisibility.upstream_package, "motion");
  assert.equal(motionVisibility.upstream_version, "12.38.0");
  assert.equal(motionVisibility.source_mirror, "G:/WWW/inspirations/motion");
  assert.equal(motionVisibility.status, "present");
  assert.equal(motionVisibility.receipt_status, "present");
  assert.equal(
    motionVisibility.package_receipt_path,
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
  );
  assert.deepEqual(motionVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "motion-animation:dashboard-workflow",
    ),
    "Motion & Animation Zed receipt surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "motion-animation:receipt-hash-refresh",
    ),
    "Motion & Animation receipt-hash refresh Zed surface is missing",
  );
  assert.deepEqual(motionVisibility.receipt_hash_refresh, {
    schema: "dx.forge.package.receipt_hash_refresh",
    status: "current",
    helper_path: "examples/template/motion-receipt-hashes.ts",
    check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check",
    write_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --write",
    json_check_command:
      "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json",
    source_guard_runbook_fixture:
      "docs/packages/motion-animation.source-guard-runbook.json",
    source_guard_runbook_fixture_paths: [
      {
        source_guard_id: "motion-animation-generated-starter-materialization",
        package_id: "animation/motion",
        fixture_path: "docs/packages/motion-animation.source-guard-runbook.json",
        schema: "dx.forge.package.source_guard_runbook_fixture",
      },
    ],
    receipt_path:
      "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    hash_algorithm: "sha256",
    tracked_file_count: 22,
    stale_file_count: 0,
    missing_file_count: 0,
    runtime_execution: false,
    secret_access: false,
    zed_visibility: "motion-animation:receipt-hash-refresh",
    runtime_limitations: [
      "SOURCE-ONLY: this helper checks local Motion & Animation receipt hash freshness only.",
      "ADAPTER-BOUNDARY: route choreography, reduced-motion policy, accessibility QA, animation budgets, and browser runtime proof stay app-owned.",
    ],
  });

  const surfaceIds = motionVisibility.selected_surfaces.map(
    (surface) => surface.surface_id,
  );
  assert.deepEqual(surfaceIds, [
    "motion-dashboard-workflow",
    "motion-interaction-proof",
    "motion-source-guard-runbook",
    "motion-source-owned-template-helpers",
  ]);

  for (const surface of motionVisibility.selected_surfaces) {
    assert.equal(surface.status, "present");
    assert.equal(
      surface.receipt_path,
      "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
    );
    assert.equal(surface.hash_algorithm, "sha256");
    for (const [filePath, hash] of Object.entries(surface.file_hashes)) {
      assert.equal(
        hash,
        receipt.file_hashes[filePath],
        `${surface.surface_id} hash for ${filePath} must mirror the Motion & Animation receipt`,
      );
    }
  }

  assert.equal(receipt.hash_algorithm, "sha256");
  assert.deepEqual(receipt.dx_check_visibility.statuses, statusVocabulary);

  for (const filePath of receipt.files) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in Motion & Animation receipt`,
    );
  }

  const sourceMarkers = motionVisibility.selected_surfaces.flatMap(
    (surface) => surface.source_markers,
  );
  for (const marker of [
    'data-dx-package="animation/motion"',
    'data-dx-component="launch-motion-dashboard-workflow"',
    'data-dx-component="motion-interaction-proof"',
    'data-dx-dashboard-workflow="motion-panel-orchestration"',
    'data-dx-motion-interaction="advance-stage"',
    'data-dx-motion-interaction="move-stage-next"',
    "data-dx-motion-preference-storage",
    "data-dx-motion-keyboard-reorder",
    "docs/packages/motion-animation.source-guard-runbook.json",
  ]) {
    assert.ok(
      sourceMarkers.includes(marker),
      `${marker} missing from Motion & Animation visibility markers`,
    );
  }

  for (const metric of [
    "motion_animation_receipt_present",
    "motion_animation_receipt_stale",
    "motion_animation_missing_receipt",
    "motion_animation_blocked_surface",
    "motion_animation_unsupported_surface",
    "motion_animation_hash_manifest_present",
    "motion_animation_hash_mismatch",
  ]) {
    assert.ok(
      motionVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Motion & Animation visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const motionAnimationPackageVisibility/);
  assert.match(readModel, /motionAnimationPackageVisibility,/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /motion-animation:receipt-hash-refresh/);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture: "docs\/packages\/motion-animation\.source-guard-runbook\.json"/,
  );
  assert.match(readModel, /sourceGuardRunbookFixturePaths/);
  assert.match(readModel, /sourceGuardId: "motion-animation-generated-starter-materialization"/);
  assert.match(readModel, /helperPath: "examples\/template\/motion-receipt-hashes\.ts"/);
  assert.match(statusSource, /motionAnimationPackageVisibility/);
  assert.match(statusSource, /motionAnimationVisibility/);
  assert.match(packageDoc, /Receipt Hash Refresh/);
  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/motion-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /package-status read model/);
  assert.match(packageDoc, /source_guard_runbook_fixture/);
  assert.match(packageDoc, /motion_animation_hash_mismatch/);
});
