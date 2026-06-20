const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";
const runbookFixturePath = "docs/packages/next-intl.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const expectedHashFiles = [
  "examples/template/next-intl-dashboard-locale-contract.ts",
  "examples/template/next-intl-dashboard-locale.tsx",
  "core/src/ecosystem/forge_next_intl.rs",
  runbookFixturePath,
  previewManifestMaterializerPath,
];
const workflowHashFiles = expectedHashFiles.filter(
  (filePath) =>
    filePath !== runbookFixturePath &&
    filePath !== previewManifestMaterializerPath,
);

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Internationalization receipt exposes hash-backed dx-check freshness", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const provider = readMirror(
    "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
  );
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const middleware = readMirror("packages/next-intl/src/middleware/middleware.tsx");
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const rustHelper = read(
    "core/src/ecosystem/project_check/internationalization_dx_check.rs",
  );
  const packageDoc = read("docs/packages/next-intl.md");

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);
  assert.match(middleware, /export default function createMiddleware/);

  assert.equal(receipt.package_id, "i18n/next-intl");
  assert.equal(receipt.package_name, "Internationalization");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.deepEqual(receipt.files, expectedHashFiles);
  assert.ok(receipt.file_hashes, "receipt file_hashes manifest is missing");

  for (const filePath of expectedHashFiles) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in the Internationalization receipt`,
    );
  }

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "i18n/next-intl",
  );
  assert.ok(visibility, "Internationalization package-status row is missing");

  for (const surfaceId of [
    "next-intl-dashboard-locale-workflow",
    "next-intl-dashboard-message-contract",
  ]) {
    const surface = visibility.selected_surfaces.find(
      (candidate) => candidate.surface_id === surfaceId,
    );
    assert.ok(surface, `${surfaceId} is missing from package-status`);
    assert.equal(surface.hash_algorithm, "sha256");
    for (const filePath of workflowHashFiles) {
      assert.equal(surface.file_hashes[filePath], receipt.file_hashes[filePath]);
    }
  }

  const runbookSurface = visibility.selected_surfaces.find(
    (candidate) =>
      candidate.surface_id === "internationalization-source-guard-runbook",
  );
  assert.ok(
    runbookSurface,
    "Internationalization source-guard runbook surface is missing",
  );
  assert.equal(runbookSurface.hash_algorithm, "sha256");
  assert.deepEqual(runbookSurface.files, [runbookFixturePath]);
  assert.equal(
    runbookSurface.file_hashes[runbookFixturePath],
    receipt.file_hashes[runbookFixturePath],
  );
  const materializerSurface = visibility.selected_surfaces.find(
    (candidate) =>
      candidate.surface_id ===
      "internationalization-preview-manifest-materializer",
  );
  assert.ok(
    materializerSurface,
    "Internationalization preview-manifest materializer surface is missing",
  );
  assert.equal(materializerSurface.hash_algorithm, "sha256");
  assert.deepEqual(materializerSurface.files, [
    previewManifestMaterializerPath,
    "public/preview-.dx/build-cache/manifest.json",
  ]);
  assert.equal(
    materializerSurface.file_hashes[previewManifestMaterializerPath],
    receipt.file_hashes[previewManifestMaterializerPath],
  );
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.deepEqual(visibility.receipt_hash_refresh.tracked_files, expectedHashFiles);
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 5);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/next-intl\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
  );

  for (const metric of [
    "internationalization_hash_manifest_present",
    "internationalization_hash_mismatch",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Internationalization package-status row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(rustHelper, new RegExp(metric));
    assert.match(packageDoc, new RegExp(metric));
  }

  for (const marker of [
    "count_sha256_file_hash_mismatches(root, surface)",
    "internationalization-hash-mismatch",
    "hash_manifest_present",
    "hash_mismatches",
  ]) {
    assert.match(
      rustHelper,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /file_hashes/);
  assert.match(packageDoc, /preview_manifest_materializer/);
  assert.match(packageDoc, /without claiming live locale routing proof/);
});
