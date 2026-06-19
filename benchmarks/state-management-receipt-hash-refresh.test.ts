const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "examples/template/state-management-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json";
const runbookFixturePath = "docs/packages/state-zustand.source-guard-runbook.json";
const previewManifestMaterializerPath = "tools/launch/materialize-www-template.ts";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function runHelper(args) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd: root,
    encoding: "utf8",
  });
}

function copyFixtureFile(tempRoot, relativePath) {
  const target = path.join(tempRoot, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(root, relativePath), target);
}

function materializerDriftFixture() {
  const tempRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-state-management-receipt-drift-"),
  );
  const receipt = readJson(receiptPath);
  for (const relativePath of Object.keys(receipt.file_hashes)) {
    copyFixtureFile(tempRoot, relativePath);
  }
  copyFixtureFile(tempRoot, receiptPath);
  copyFixtureFile(tempRoot, "examples/template/.dx/forge/package-status.json");
  copyFixtureFile(tempRoot, "examples/template/forge-package-status-read-model.ts");

  fs.appendFileSync(
    path.join(tempRoot, previewManifestMaterializerPath),
    "\n// State Management helper drift fixture.\n",
  );

  return tempRoot;
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("State Management exposes receipt hash freshness through package-status and read model", () => {
  const helperSource = read(helperPath);
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const packageDoc = read("docs/packages/state-zustand.md");

  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "State Management"/);
  assert.match(helperSource, /PACKAGE_ID = "state\/zustand"/);
  assert.match(helperSource, /UPSTREAM_PACKAGE = "zustand"/);
  assert.match(helperSource, /dx\.forge\.package\.receipt_hash_refresh/);
  assert.doesNotMatch(helperSource, /fetch\(|localStorage|sessionStorage/);

  assert.equal(receipt.packageId, "state/zustand");
  assert.equal(receipt.officialPackageName, "State Management");
  assert.equal(receipt.upstreamPackage, "zustand");
  assert.equal(receipt.upstreamVersion, "5.0.13");
  assert.equal(receipt.sourceMirror, "G:/WWW/inspirations/zustand");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.file_hashes, "State Management receipt is missing file_hashes");

  const trackedFiles = Object.keys(receipt.file_hashes);
  assert.deepEqual(trackedFiles.sort(), [
    "docs/packages/state-zustand.md",
    runbookFixturePath,
    studioManifestSourcePath,
    "examples/template/template-shell.tsx",
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    "tools/launch/runtime-template/pages/index.html",
    "examples/template/state-zustand-dashboard.tsx",
    previewManifestMaterializerPath,
  ]);
  assert.ok(
    receipt.sourceFiles.includes(previewManifestMaterializerPath),
    "State Management receipt must list the preview-manifest materializer as source",
  );
  assert.ok(
    receipt.sourceFiles.includes(studioManifestSourcePath),
    "State Management receipt must list the Studio manifest source as source",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "state-management-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath],
    ),
    "State Management receipt must monitor the preview-manifest materializer hash",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "state-management-studio-manifest-source" &&
        surface.file_hashes?.[studioManifestSourcePath],
    ),
    "State Management receipt must monitor the Studio manifest source hash",
  );

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "state/zustand",
  );
  assert.ok(visibility, "State Management package-status row is missing");
  assert.equal(visibility.official_package_name, "State Management");
  assert.equal(visibility.upstream_package, "zustand");

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "State Management receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperPath);
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(hashRefresh.studio_manifest_source, studioManifestSourcePath);
  assert.ok(
    hashRefresh.tracked_files.includes(previewManifestMaterializerPath),
    "State Management receipt_hash_refresh must list the materializer in tracked_files",
  );
  assert.ok(
    hashRefresh.tracked_files.includes(studioManifestSourcePath),
    "State Management receipt_hash_refresh must list the Studio manifest source in tracked_files",
  );
  assert.ok(
    hashRefresh.current_files.includes("examples/template/state-zustand-dashboard.tsx"),
    "State Management receipt_hash_refresh must expose current dashboard state files",
  );
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.deepEqual(hashRefresh.missing_files, []);
  assert.deepEqual(hashRefresh.stale_mirror_files, []);
  assert.deepEqual(hashRefresh.missing_mirror_files, []);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, trackedFiles.length);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "state-management:receipt-hash-refresh");

  for (const surface of visibility.selected_surfaces) {
    assert.equal(surface.hash_algorithm, "sha256");
    assert.ok(
      surface.file_hashes,
      `${surface.surface_id} is missing State Management file hashes`,
    );
  }

  assert.ok(
    status.zed_receipt_surfaces.includes(
      "state-management:receipt-hash-refresh",
    ),
    "State Management helper is missing from Zed receipt surfaces",
  );

  assert.match(readModel, /export const stateManagementPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /currentFiles/);
  assert.match(readModel, /staleMirrorFiles/);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/state-zustand\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /studioManifestSource:\s*"dx-www\/src\/cli\/studio_manifest\.rs"/,
  );
  assert.match(readModel, /state-management:receipt-hash-refresh/);
  assert.match(readModel, /stateManagementPackageVisibility/);
  const stateReadModelStart = readModel.indexOf(
    "export const stateManagementPackageVisibility = {",
  );
  const stateReadModelEnd = readModel.indexOf(
    "} as const satisfies LaunchForgePackageLaneVisibility;",
    stateReadModelStart,
  );
  assert.notEqual(stateReadModelStart, -1, "State Management read model is missing");
  assert.notEqual(stateReadModelEnd, -1, "State Management read model is not closed");
  const stateReadModelBlock = readModel.slice(stateReadModelStart, stateReadModelEnd);
  for (const relativePath of trackedFiles) {
    const mirrors = [
      ...stateReadModelBlock.matchAll(
        new RegExp(
          `"${escapeRegex(relativePath)}"\\s*:\\s*(?:\\r?\\n\\s*)?"([^"]+)"`,
          "g",
        ),
      ),
    ].map((match) => match[1]);
    assert.ok(mirrors.length > 0, `${relativePath} missing from State Management read model`);
    assert.ok(
      mirrors.every((hash) => hash === receipt.file_hashes[relativePath]),
      `${relativePath} has stale State Management read model mirrors: ${mirrors.join(", ")}`,
    );
  }
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /state-management-receipt-hashes\.ts --check/);
  assert.match(packageDoc, /materialize-www-template\.ts/);
  assert.match(packageDoc, /studio_manifest\.rs/);

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "state/zustand");
  assert.equal(helperReport.official_package_name, "State Management");
  assert.equal(helperReport.upstream_package, "zustand");
  assert.equal(helperReport.upstream_version, "5.0.13");
  assert.equal(helperReport.source_mirror, "G:/WWW/inspirations/zustand");
  assert.equal(helperReport.status, "current");
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(helperReport.studio_manifest_source, studioManifestSourcePath);
  assert.ok(helperReport.tracked_files.includes(previewManifestMaterializerPath));
  assert.ok(helperReport.tracked_files.includes(studioManifestSourcePath));
  assert.ok(
    helperReport.current_files.includes("examples/template/state-zustand-dashboard.tsx"),
  );
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.deepEqual(helperReport.stale_mirror_files, []);
  assert.deepEqual(helperReport.missing_mirror_files, []);
  assert.equal(helperReport.tracked_file_count, trackedFiles.length);
  assert.equal(helperReport.stale_file_count, 0);
  assert.equal(helperReport.missing_file_count, 0);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
  assert.equal(helperReport.zed_visibility, "state-management:receipt-hash-refresh");
});

test("State Management helper attributes materializer drift while dashboard sources stay current", () => {
  const tempRoot = materializerDriftFixture();
  try {
    const helper = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.equal(helper.status, 1, helper.stdout + helper.stderr);
    const helperReport = JSON.parse(helper.stdout);

    assert.equal(helperReport.status, "stale");
    assert.deepEqual(helperReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(helperReport.missing_files, []);
    assert.ok(
      helperReport.stale_mirror_files.includes(previewManifestMaterializerPath),
      "materializer drift must be attributed as stale mirror evidence",
    );

    const selectedDashboardSources = [
      "examples/template/state-zustand-dashboard.tsx",
      "examples/template/template-shell.tsx",
      "tools/launch/runtime-template/assets/launch-runtime.ts",
      "tools/launch/runtime-template/pages/index.html",
    ];
    for (const relativePath of selectedDashboardSources) {
      assert.ok(
        helperReport.current_files.includes(relativePath),
        `${relativePath} should remain current when only the materializer drifts`,
      );
      assert.ok(
        !helperReport.stale_files.includes(relativePath),
        `${relativePath} should not be reported stale for a materializer-only drift`,
      );
    }
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});
