const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/forms-receipt-hashes.ts",
);
const runbookFixturePath = "docs/packages/forms.source-guard-runbook.json";
const studioManifestPath = "dx-www/src/cli/studio_manifest.rs";
const lowerDxCheckPath = "core/src/ecosystem/project_check/forms_dx_check.rs";

function runHelper(args, cwd = root) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd,
    encoding: "utf8",
  });
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function readModelExport(source, exportName) {
  const start = source.indexOf(`export const ${exportName} = {`);
  assert.notEqual(start, -1, `${exportName} export is missing`);
  const nextExport = source.indexOf("\n\nexport const ", start + 1);
  return source.slice(start, nextExport === -1 ? undefined : nextExport);
}

function sha256(relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(root, relativePath)))
    .digest("hex");
}

test("Forms receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Forms hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forms-hashes-"));
  try {
    const selectedFiles = [
      "examples/template/template-lead-form.tsx",
      "docs/packages/forms-react-hook-form.md",
      studioManifestPath,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const formsHashFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.receipt",
      package_id: "forms/react-hook-form",
      package_name: "Forms",
      upstream_package: "react-hook-form",
      upstream_version: "7.75.0",
      hash_algorithm: "sha256",
      file_hashes: selectedFiles.map((selectedFile) => ({
        path: selectedFile,
        sha256: "stale",
      })),
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Forms",
          package_id: "forms/react-hook-form",
          package_receipt_path: ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
          selected_surfaces: [
            {
              surface_id: "template-lead-form",
              receipt_path:
                ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[0]]: "stale",
              },
            },
            {
              surface_id: "forms-provider-fields",
              receipt_path:
                ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[1]]: "stale",
              },
            },
            {
              surface_id: "forms-studio-manifest",
              receipt_path:
                ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[2]]: "stale",
              },
            },
          ],
        },
      ],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const unrelatedPackageVisibility = {",
        "  fileHashes: {",
        `    "${selectedFiles[0]}":`,
        '      "stale-unrelated-copy",',
        "  },",
        "};",
        "",
        "export const formsPackageVisibility = {",
        '  packageId: "forms/react-hook-form",',
        '  packageReceiptPath: ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",',
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        `        "${selectedFiles[0]}":`,
        '          "stale",',
        "      },",
        "    },",
        "    {",
        "      fileHashes: {",
        `        "${selectedFiles[1]}":`,
        '          "stale",',
        "      },",
        "    },",
        "    {",
        "      fileHashes: {",
        `        "${selectedFiles[2]}":`,
        '          "stale",',
        "      },",
        "    },",
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
      ].join("\n"),
    );

    const staleJson = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(staleJson.status, 0, staleJson.stdout + staleJson.stderr);
    const staleReport = JSON.parse(staleJson.stdout);
    assert.equal(staleReport.status, "missing");
    assert.deepEqual(staleReport.tracked_files, selectedFiles);
    assert.deepEqual(staleReport.current_files, []);
    assert.deepEqual(staleReport.stale_files, selectedFiles);
    assert.deepEqual(staleReport.missing_files, []);
    assert.deepEqual(staleReport.stale_mirror_files, selectedFiles);
    assert.ok(
      staleReport.missing_mirror_files.includes(packageStatusPath),
      "missing package-status receipt_hash_refresh should name package-status path",
    );
    assert.ok(
      staleReport.missing_mirror_files.includes(readModelPath),
      "missing read-model receiptHashRefresh should name read-model path",
    );

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /template-lead-form\.tsx/);
    assert.match(stale.stdout + stale.stderr, /forms-react-hook-form\.md/);
    assert.match(stale.stdout + stale.stderr, /studio_manifest\.rs/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.schema, "dx.forge.package.receipt_hash_refresh");
    assert.equal(report.official_package_name, "Forms");
    assert.equal(report.package_id, "forms/react-hook-form");
    assert.equal(report.status, "current");
    assert.deepEqual(report.tracked_files, selectedFiles);
    assert.deepEqual(report.current_files, selectedFiles);
    assert.deepEqual(report.stale_files, []);
    assert.deepEqual(report.missing_files, []);
    assert.deepEqual(report.stale_mirror_files, []);
    assert.deepEqual(report.missing_mirror_files, []);
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.equal(report.zed_visibility, "forms:receipt-hash-refresh");

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");
    const formsReadModel = readModelExport(readModelText, "formsPackageVisibility");

    for (const selectedFile of selectedFiles) {
      const refreshedEntry = refreshedReceipt.file_hashes.find(
        (entry) => entry.path === selectedFile,
      );
      assert.ok(refreshedEntry, `${selectedFile} missing from receipt`);
      assert.match(refreshedEntry.sha256, /^[a-f0-9]{64}$/);
      assert.match(formsReadModel, new RegExp(refreshedEntry.sha256));
    }
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[0]
        .file_hashes[selectedFiles[0]],
      refreshedReceipt.file_hashes[0].sha256,
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[1]
        .file_hashes[selectedFiles[1]],
      refreshedReceipt.file_hashes[1].sha256,
    );
    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path: "examples/template/forms-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/forms-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        receipt_path:
          "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
        hash_algorithm: "sha256",
        tracked_file_count: 3,
        tracked_files: selectedFiles,
        current_files: selectedFiles,
        stale_files: [],
        missing_files: [],
        stale_mirror_files: [],
        missing_mirror_files: [],
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "forms:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Forms receipt hash freshness only.",
          "ADAPTER-BOUNDARY: browser submission proof, dependency installation, submit handlers, persistence, authorization, spam protection, and accessibility review stay app-owned.",
        ],
      },
    );
    assert.match(readModelText, /receiptHashRefresh: \{/);
    assert.match(readModelText, /trackedFiles: \[/);
    assert.match(readModelText, /currentFiles: \[/);
    assert.match(readModelText, /staleFiles: \[\]/);
    assert.match(readModelText, /missingFiles: \[\]/);
    assert.match(readModelText, /staleMirrorFiles: \[\]/);
    assert.match(readModelText, /missingMirrorFiles: \[\]/);
    assert.match(readModelText, /zedVisibility: "forms:receipt-hash-refresh"/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture:\s*"docs\/packages\/forms\.source-guard-runbook\.json"/,
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Forms docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/forms-react-hook-form.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/forms-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run browser submission proof/i);
});

test("Forms receipt helper tracks the source-guard runbook and Studio manifest surfaces", () => {
  const receipt = JSON.parse(
    fs.readFileSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
      ),
      "utf8",
    ),
  );
  const packageStatus = JSON.parse(
    fs.readFileSync(
      path.join(root, "examples/template/.dx/forge/package-status.json"),
      "utf8",
    ),
  );
  const readModel = fs.readFileSync(
    path.join(root, "examples/template/forge-package-status-read-model.ts"),
    "utf8",
  );
  const runbookFixture = JSON.parse(
    fs.readFileSync(path.join(root, runbookFixturePath), "utf8"),
  );
  const launchShell = fs.readFileSync(
    path.join(root, "examples/template/template-shell.tsx"),
    "utf8",
  );
  const runtimePage = fs.readFileSync(
    path.join(root, "tools/launch/runtime-template/pages/index.html"),
    "utf8",
  );

  const report = runHelper(["--check", "--json"]);
  assert.equal(report.status, 0, report.stdout + report.stderr);
  const helperReport = JSON.parse(report.stdout);

  const receiptHashPaths = receipt.file_hashes.map((entry) => entry.path);
  assert.ok(
    receiptHashPaths.includes(runbookFixturePath),
    "receipt must hash the Forms source-guard runbook fixture",
  );
  assert.ok(
    receiptHashPaths.includes(studioManifestPath),
    "receipt must hash the Forms Studio manifest source guard handoff",
  );
  assert.ok(
    receiptHashPaths.includes(lowerDxCheckPath),
    "receipt must hash the Forms lower dx-check helper freshness module",
  );
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(helperReport.tracked_file_count, receipt.file_hashes.length);
  assert.equal(helperReport.tracked_file_count, 7);
  assert.deepEqual(runbookFixture.receipt.attribution_fields, [
    "tracked_files",
    "current_files",
    "stale_files",
    "missing_files",
    "stale_mirror_files",
    "missing_mirror_files",
  ]);
  assert.ok(
    helperReport.current_files.includes(studioManifestPath),
    "hash helper should attribute the current Studio manifest file",
  );
  assert.ok(
    helperReport.current_files.includes(lowerDxCheckPath),
    "hash helper should attribute the current lower dx-check module",
  );
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.deepEqual(helperReport.stale_mirror_files, []);
  assert.deepEqual(helperReport.missing_mirror_files, []);

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "forms/react-hook-form",
  );
  assert.ok(visibility, "Forms package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 7);
  assert.ok(
    visibility.receipt_hash_refresh.current_files.includes(studioManifestPath),
    "package-status helper payload should attribute the current Studio manifest file",
  );
  assert.ok(
    visibility.receipt_hash_refresh.current_files.includes(lowerDxCheckPath),
    "package-status helper payload should attribute the current lower dx-check module",
  );
  assert.deepEqual(visibility.receipt_hash_refresh.stale_files, []);
  assert.deepEqual(visibility.receipt_hash_refresh.missing_files, []);
  assert.deepEqual(visibility.receipt_hash_refresh.stale_mirror_files, []);
  assert.deepEqual(visibility.receipt_hash_refresh.missing_mirror_files, []);
  const studioManifestSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "forms-studio-manifest",
  );
  assert.ok(studioManifestSurface, "Forms Studio manifest surface is missing");
  assert.deepEqual(studioManifestSurface.files, [studioManifestPath]);
  assert.equal(
    studioManifestSurface.file_hashes[studioManifestPath],
    sha256(studioManifestPath),
  );
  const lowerDxCheckSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "forms-lower-dx-check",
  );
  assert.ok(lowerDxCheckSurface, "Forms lower dx-check surface is missing");
  assert.deepEqual(lowerDxCheckSurface.files, [lowerDxCheckPath]);
  assert.equal(
    lowerDxCheckSurface.file_hashes[lowerDxCheckPath],
    sha256(lowerDxCheckPath),
  );
  assert.ok(
    lowerDxCheckSurface.source_markers.includes(
      "forms_package_metrics_reports_helper_freshness_from_path_arrays",
    ),
    "Forms lower dx-check surface should name the helper freshness fixture",
  );
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/forms\.source-guard-runbook\.json"/,
  );
  assert.match(readModel, /trackedFileCount: 7/);
  assert.match(
    readModel,
    /currentFiles: \[[\s\S]*"dx-www\/src\/cli\/studio_manifest\.rs"/,
  );
  assert.match(
    readModel,
    /currentFiles: \[[\s\S]*"core\/src\/ecosystem\/project_check\/forms_dx_check\.rs"/,
  );
  assert.match(readModel, /staleFiles: \[\]/);
  assert.match(readModel, /missingFiles: \[\]/);
  assert.match(readModel, /surfaceId: "forms-studio-manifest"/);
  assert.match(readModel, /"dx-www\/src\/cli\/studio_manifest\.rs"/);
  assert.match(readModel, /surfaceId: "forms-lower-dx-check"/);
  assert.match(
    readModel,
    /"core\/src\/ecosystem\/project_check\/forms_dx_check\.rs"/,
  );
  assert.match(
    launchShell,
    /package_id: "forms\/react-hook-form"[\s\S]*hash_refresh_tracked_files: 7/,
  );
  assert.match(
    runtimePage,
    /data-dx-check-package-lane-template="forms\/react-hook-form"[\s\S]*data-dx-check-package-lane-hash-refresh-tracked-files="7"/,
  );
});
