const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/documentation-system-receipt-hashes.ts",
);
const runbookFixturePath =
  "docs/packages/content-fumadocs-next.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";

function runHelper(args) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd: root,
    encoding: "utf8",
  });
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function documentationSystemSelectedFiles() {
  return [
    "core/src/ecosystem/forge_fumadocs.rs",
    "examples/template/package-catalog.ts",
    "examples/template/docs-status.tsx",
    "examples/dashboard/src/lib/fumadocsDocsWorkflow.ts",
    "examples/dashboard/src/components/FumadocsDocsWorkflow.tsx",
    "docs/packages/content-fumadocs-next.md",
    runbookFixturePath,
    previewManifestMaterializerPath,
  ];
}

test("Documentation System receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Documentation System hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-documentation-system-hashes-"),
  );
  try {
    const selectedFiles = documentationSystemSelectedFiles();
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const documentationSystemFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "content/fumadocs-next",
      package_name: "Documentation System",
      official_dx_package_name: "Documentation System",
      upstream_package: "fumadocs",
      upstream_version: "16.8.12",
      source_mirror: "G:/WWW/inspirations/fumadocs",
      hash_algorithm: "sha256",
      source_hashes: {
        algorithm: "sha256",
        files: selectedFiles.map((selectedFile) => ({
          path: selectedFile,
          sha256: "stale",
        })),
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Documentation System",
          package_id: "content/fumadocs-next",
          upstream_package: "fumadocs",
          upstream_version: "16.8.12",
          source_mirror: "G:/WWW/inspirations/fumadocs",
          status: "present",
          receipt_status: "present",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "dashboard-help-workflow",
              status: "present",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
          source_hashes: {
            algorithm: "sha256",
            files: Object.fromEntries(
              selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
        },
      ],
      zed_receipt_surfaces: ["documentation-system:docs-help-changelog"],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const stateManagementPackageVisibility = {",
        '  packageId: "state/zustand",',
        "  selectedSurfaces: [{",
        "    fileHashes: {",
        '      "examples/template/package-catalog.ts": "adjacent-stale-hash",',
        "    },",
        "  }],",
        "} as const;",
        "",
        "export const documentationSystemPackageVisibility = {",
        "  selectedSurfaces: [{",
        "    fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `      "${selectedFile}":`,
          '        "stale",',
        ]),
        "    },",
        "  }],",
        "  sourceHashes: {",
        '    algorithm: "sha256",',
        "    files: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `      "${selectedFile}":`,
          '        "stale",',
        ]),
        "    },",
        "  },",
        "} as const;",
        "",
        "export const authenticationPackageVisibility = {",
        '  packageId: "auth/better-auth",',
        '  status: "present",',
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "content/fumadocs-next");
    assert.equal(staleReport.official_package_name, "Documentation System");
    assert.equal(staleReport.upstream_package, "fumadocs");
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.zed_visibility, "documentation-system:receipt-hash-refresh");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Documentation System receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.equal(freshReport.source_guard_runbook_fixture, runbookFixturePath);
    assert.equal(
      freshReport.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.deepEqual(freshReport.tracked_files, selectedFiles);
    assert.equal(freshReport.stale_file_count, 0);
    assert.equal(freshReport.missing_file_count, 0);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");

    for (const selectedFile of selectedFiles) {
      const sourceEntry = refreshedReceipt.source_hashes.files.find(
        (entry) => entry.path === selectedFile,
      );
      assert.match(sourceEntry.sha256, /^[a-f0-9]{64}$/);
      assert.equal(
        refreshedStatus.package_lane_visibility[0].source_hashes.files[selectedFile],
        sourceEntry.sha256,
      );
      assert.equal(
        refreshedStatus.package_lane_visibility[0].selected_surfaces[0].file_hashes[
          selectedFile
        ],
        sourceEntry.sha256,
      );
      assert.match(readModelText, new RegExp(sourceEntry.sha256));
    }

    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/documentation-system-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        preview_manifest_materializer: previewManifestMaterializerPath,
        receipt_path: receiptPath,
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        tracked_files: selectedFiles,
        current_files: selectedFiles,
        stale_files: [],
        missing_files: [],
        stale_mirror_files: [],
        missing_mirror_files: [],
        mirror_problem_count: 0,
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "documentation-system:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Documentation System receipt hash freshness only.",
          "ADAPTER-BOUNDARY: live Fumadocs rendering, hosted search indexing, OpenAPI proxy execution, dependency installation, and governed browser QA stay app-owned.",
        ],
      },
    );
    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes(
        "documentation-system:receipt-hash-refresh",
      ),
    );
    assert.match(readModelText, /receiptHashRefresh/);
    assert.match(readModelText, /documentation-system:receipt-hash-refresh/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture:\s*"docs\/packages\/content-fumadocs-next\.source-guard-runbook\.json"/,
    );
    assert.match(
      readModelText,
      /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
    );
    assert.match(readModelText, /trackedFiles:\s*\[/);
    assert.match(readModelText, /currentFiles:\s*\[/);
    assert.match(readModelText, /staleFiles:\s*\[\s*\]/);
    assert.match(readModelText, /missingFiles:\s*\[\s*\]/);
    assert.match(readModelText, /staleMirrorFiles:\s*\[\s*\]/);
    assert.match(readModelText, /missingMirrorFiles:\s*\[\s*\]/);
    assert.match(readModelText, /mirrorProblemCount:\s*0/);
    assert.match(
      readModelText,
      /authenticationPackageVisibility/,
      "Documentation System helper must not rewrite adjacent package lanes",
    );
    assert.match(
      readModelText,
      /adjacent-stale-hash/,
      "Documentation System helper must ignore same-path hashes in adjacent package lanes",
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Documentation System receipt hash helper attributes materializer-only stale drift", () => {
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-documentation-system-materializer-stale-"),
  );
  try {
    const selectedFiles = documentationSystemSelectedFiles();
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const documentationSystemFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "content/fumadocs-next",
      package_name: "Documentation System",
      official_dx_package_name: "Documentation System",
      upstream_package: "fumadocs",
      upstream_version: "16.8.12",
      source_mirror: "G:/WWW/inspirations/fumadocs",
      hash_algorithm: "sha256",
      source_hashes: {
        algorithm: "sha256",
        files: selectedFiles.map((selectedFile) => ({
          path: selectedFile,
          sha256: "stale",
        })),
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Documentation System",
          package_id: "content/fumadocs-next",
          upstream_package: "fumadocs",
          upstream_version: "16.8.12",
          source_mirror: "G:/WWW/inspirations/fumadocs",
          status: "present",
          receipt_status: "present",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "dashboard-help-workflow",
              status: "present",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
          source_hashes: {
            algorithm: "sha256",
            files: Object.fromEntries(
              selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
        },
      ],
      zed_receipt_surfaces: ["documentation-system:docs-help-changelog"],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const documentationSystemPackageVisibility = {",
        "  selectedSurfaces: [{",
        "    fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `      "${selectedFile}":`,
          '        "stale",',
        ]),
        "    },",
        "  }],",
        "  sourceHashes: {",
        '    algorithm: "sha256",',
        "    files: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `      "${selectedFile}":`,
          '        "stale",',
        ]),
        "    },",
        "  },",
        "} as const;",
        "",
      ].join("\n"),
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);

    fs.appendFileSync(
      path.join(fixtureRoot, previewManifestMaterializerPath),
      "export const changedMaterializerOnly = true;\n",
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);

    const expectedCurrentFiles = selectedFiles.filter(
      (selectedFile) => selectedFile !== previewManifestMaterializerPath,
    );
    assert.equal(staleReport.status, "stale");
    assert.deepEqual(staleReport.current_files, expectedCurrentFiles);
    assert.deepEqual(staleReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(staleReport.missing_files, []);
    assert.deepEqual(staleReport.stale_mirror_files, []);
    assert.deepEqual(staleReport.missing_mirror_files, []);
    assert.equal(staleReport.mirror_problem_count, 0);
    assert.equal(staleReport.stale_file_count, 1);
    assert.equal(staleReport.missing_file_count, 0);
    assert.ok(
      !staleReport.stale_files.includes(
        "examples/dashboard/src/lib/fumadocsDocsWorkflow.ts",
      ),
      "materializer drift must not mark dashboard workflow source stale",
    );
    assert.ok(
      !staleReport.stale_files.includes("core/src/ecosystem/forge_fumadocs.rs"),
      "materializer drift must not mark the Forge source stale",
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Documentation System current receipt tracks the runbook fixture and preview-manifest materializer", () => {
  const receiptPath =
    "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json";
  const receipt = JSON.parse(fs.readFileSync(path.join(root, receiptPath), "utf8"));
  const packageStatus = JSON.parse(
    fs.readFileSync(
      path.join(root, "examples/template/.dx/forge/package-status.json"),
      "utf8",
    ),
  );
  const readModelText = fs.readFileSync(
    path.join(root, "examples/template/forge-package-status-read-model.ts"),
    "utf8",
  );
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/content-fumadocs-next.md"),
    "utf8",
  );
  const helperReportResult = runHelper(["--check", "--json"]);

  assert.equal(helperReportResult.status, 0, helperReportResult.stdout + helperReportResult.stderr);
  const helperReport = JSON.parse(helperReportResult.stdout);
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    helperReport.tracked_files.includes(previewManifestMaterializerPath),
    "Documentation System helper report must list the preview-manifest materializer as a tracked file",
  );
  assert.ok(
    receipt.source_hashes.files.some((entry) => entry.path === runbookFixturePath),
    "Documentation System receipt must hash the package-owned runbook fixture",
  );
  assert.ok(
    receipt.source_hashes.files.some(
      (entry) => entry.path === previewManifestMaterializerPath,
    ),
    "Documentation System receipt must hash the preview-manifest materializer",
  );
  assert.equal(helperReport.tracked_file_count, receipt.source_hashes.files.length);

  const visibility = packageStatus.package_lane_visibility.find(
    (row) => row.package_id === "content/fumadocs-next",
  );
  assert.ok(visibility, "Documentation System package-status row missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(
      previewManifestMaterializerPath,
    ),
    "Documentation System package-status should expose tracked_files for the preview-manifest materializer",
  );
  const runbookMirrors = visibility.selected_surfaces.filter(
    (surface) =>
      surface.hash_algorithm === "sha256" &&
      surface.file_hashes &&
      Object.prototype.hasOwnProperty.call(surface.file_hashes, runbookFixturePath),
  );
  assert.ok(
    runbookMirrors.length > 0,
    "Documentation System package-status should expose a selected surface for the runbook fixture hash",
  );
  for (const surface of runbookMirrors) {
    assert.equal(
      surface.file_hashes[runbookFixturePath],
      visibility.source_hashes.files[runbookFixturePath],
      `${surface.surface_id} should mirror the runbook fixture hash`,
    );
  }
  const materializerMirrors = visibility.selected_surfaces.filter(
    (surface) =>
      surface.hash_algorithm === "sha256" &&
      surface.file_hashes &&
      Object.prototype.hasOwnProperty.call(
        surface.file_hashes,
        previewManifestMaterializerPath,
      ),
  );
  assert.ok(
    materializerMirrors.some(
      (surface) =>
        surface.surface_id ===
        "documentation-system-preview-manifest-materializer",
    ),
    "Documentation System package-status should expose a selected surface for the preview-manifest materializer hash",
  );
  for (const surface of materializerMirrors) {
    assert.equal(
      surface.file_hashes[previewManifestMaterializerPath],
      visibility.source_hashes.files[previewManifestMaterializerPath],
      `${surface.surface_id} should mirror the preview-manifest materializer hash`,
    );
  }

  assert.match(
    readModelText,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/content-fumadocs-next\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModelText,
    /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(
    readModelText,
    /"tools\/launch\/materialize-www-template\.ts":\s*(?:\r?\n\s*)?"[a-f0-9]{64}"/,
  );
  assert.match(
    packageDoc,
    /source-guard runbook fixture and preview-manifest materializer are hash-tracked/,
  );
  assert.match(
    packageDoc,
    /documentation-system-preview-manifest-materializer/,
  );
});

test("Documentation System docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/content-fumadocs-next.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/documentation-system-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run live Fumadocs rendering/i);
});
