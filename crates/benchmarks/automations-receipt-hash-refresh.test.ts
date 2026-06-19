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
  "examples/template/automation-connectors-receipt-hashes.ts",
);
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";
const sourceGuardRunbookPath =
  "docs/packages/automation-connectors.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";
const lowerDxCheckSourcePath =
  "core/src/ecosystem/project_check/automation_connectors_dx_check.rs";
const checkPanelSourcePath = "core/src/ecosystem/dx_check_receipt.rs";
const legacyGiantCliModSourcePath = "dx-www/src/cli/mod.rs";

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

function writeFile(rootDir, relativePath, contents) {
  const filePath = path.join(rootDir, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, contents);
}

test("Automation Connectors receipt hash helper refreshes stale selected-file hashes", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "Automation Connectors hash helper is missing",
  );

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-automation-connectors-hashes-"),
  );
  try {
    const selectedFile = "examples/template/automations-status.tsx";
    const selectedFiles = [
      selectedFile,
      sourceGuardRunbookPath,
      previewManifestMaterializerPath,
      studioManifestSourcePath,
      lowerDxCheckSourcePath,
      checkPanelSourcePath,
    ];
    writeFile(
      fixtureRoot,
      selectedFile,
      "export const automationStatus = 'fresh connector readiness';\n",
    );
    writeFile(
      fixtureRoot,
      sourceGuardRunbookPath,
      '{"schema":"dx.forge.package.source_guard_runbook_fixture"}\n',
    );
    writeFile(
      fixtureRoot,
      previewManifestMaterializerPath,
      "const AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE = true;\n",
    );
    writeFile(
      fixtureRoot,
      studioManifestSourcePath,
      'const AUTOMATION_CONNECTORS_STUDIO_FIXTURE: &str = "automation-connectors-package-lane-panel";\n',
    );
    writeFile(
      fixtureRoot,
      lowerDxCheckSourcePath,
      'fn forge_automation_connectors_package_metrics() { let _ = "automation_connectors_receipt_hash_refresh_current"; }\n',
    );
    writeFile(
      fixtureRoot,
      checkPanelSourcePath,
      'fn automation_connectors_package_lane_row() { let _ = "check_panel.view_model.package_lane_rows"; }\n',
    );

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "automations/n8n",
      official_package_name: "Automation Connectors",
      upstream_package: "n8n-nodes-base",
      upstream_version: "2.22.0",
      hash_algorithm: "sha256",
      file_hashes: {
        [selectedFile]: "stale",
        [sourceGuardRunbookPath]: "stale",
      },
      dx_check_visibility: {
        monitored_surfaces: [
          {
            id: "automation-launch-dashboard-workflow",
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedFile]: "stale",
            },
          },
          {
            id: "automation-connectors-source-guard-runbook",
            hash_algorithm: "sha256",
            file_hashes: {
              [sourceGuardRunbookPath]: "stale",
            },
          },
          {
            id: "automation-connectors-preview-manifest-materializer",
            hash_algorithm: "sha256",
            file_hashes: {
              [previewManifestMaterializerPath]: "stale",
            },
          },
          {
            id: "automation-connectors-studio-manifest-source",
            hash_algorithm: "sha256",
            file_hashes: {
              [studioManifestSourcePath]: "stale",
            },
          },
          {
            id: "automation-connectors-lower-dx-check-source",
            hash_algorithm: "sha256",
            file_hashes: {
              [lowerDxCheckSourcePath]: "stale",
            },
          },
          {
            id: "automation-connectors-check-panel-source",
            hash_algorithm: "sha256",
            file_hashes: {
              [checkPanelSourcePath]: "stale",
            },
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Automation Connectors",
          package_id: "automations/n8n",
          package_receipt_path: receiptPath,
          selected_surfaces: selectedFiles.map((selectedPath) => ({
            surface_id:
              selectedPath === previewManifestMaterializerPath
                ? "automation-connectors-preview-manifest-materializer"
                : selectedPath === studioManifestSourcePath
                  ? "automation-connectors-studio-manifest-source"
                  : selectedPath === lowerDxCheckSourcePath
                    ? "automation-connectors-lower-dx-check-source"
                    : selectedPath === checkPanelSourcePath
                      ? "automation-connectors-check-panel-source"
                  : selectedPath === sourceGuardRunbookPath
                    ? "automation-connectors-source-guard-runbook"
                    : "automation-launch-dashboard-workflow",
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedPath]: "stale",
            },
          })),
        },
      ],
      zed_receipt_surfaces: [],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    writeFile(
      fixtureRoot,
      readModelPath,
      [
        "export const automationConnectorsPackageVisibility = {",
        '  packageId: "automations/n8n",',
        "  selectedSurfaces: [",
        ...selectedFiles.flatMap((selectedPath) => [
          "    {",
          "      fileHashes: {",
          `        "${selectedPath}": "stale",`,
          "      },",
          "    },",
        ]),
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
      ].join("\n"),
    );

    const staleJson = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(staleJson.status, 0, staleJson.stdout + staleJson.stderr);
    const staleReport = JSON.parse(staleJson.stdout);
    assert.equal(
      staleReport.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.equal(staleReport.studio_manifest_source, studioManifestSourcePath);
    assert.equal(staleReport.lower_dx_check_source, lowerDxCheckSourcePath);
    assert.equal(staleReport.check_panel_source, checkPanelSourcePath);
    assert.ok(staleReport.tracked_files.includes(previewManifestMaterializerPath));
    assert.ok(staleReport.tracked_files.includes(studioManifestSourcePath));
    assert.ok(staleReport.tracked_files.includes(lowerDxCheckSourcePath));
    assert.ok(staleReport.tracked_files.includes(checkPanelSourcePath));
    assert.ok(staleReport.stale_files.includes(previewManifestMaterializerPath));
    assert.ok(staleReport.stale_files.includes(studioManifestSourcePath));
    assert.ok(staleReport.stale_files.includes(lowerDxCheckSourcePath));
    assert.ok(staleReport.stale_files.includes(checkPanelSourcePath));

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /automations-status\.tsx/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.deepEqual(freshReport.stale_files, []);
    assert.deepEqual(freshReport.missing_files, []);
    assert.deepEqual(freshReport.stale_mirror_files, []);
    assert.deepEqual(freshReport.missing_mirror_files, []);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    for (const selectedPath of selectedFiles) {
      assert.match(
        refreshedReceipt.file_hashes[selectedPath],
        /^[a-f0-9]{64}$/,
      );
    }
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Automation Connectors docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/automations-n8n.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/automation-connectors-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(
    packageDoc,
    /does not run n8n workflows or read provider secrets/i,
  );
});

test("Automation Connectors receipt helper tracks materializer and Studio manifest handoffs", () => {
  const receipt = JSON.parse(
    fs.readFileSync(path.join(root, receiptPath), "utf8"),
  );

  const report = runHelper(["--check", "--json"]);
  assert.equal(report.status, 0, report.stdout + report.stderr);
  const helperReport = JSON.parse(report.stdout);

  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(helperReport.studio_manifest_source, studioManifestSourcePath);
  assert.equal(
    helperReport.tracked_file_count,
    Object.keys(receipt.file_hashes).length,
  );
  assert.equal(helperReport.tracked_file_count, 14);
  assert.ok(helperReport.current_files.includes(previewManifestMaterializerPath));
  assert.ok(helperReport.current_files.includes(studioManifestSourcePath));
  assert.equal(helperReport.lower_dx_check_source, lowerDxCheckSourcePath);
  assert.ok(helperReport.current_files.includes(lowerDxCheckSourcePath));
  assert.equal(helperReport.check_panel_source, checkPanelSourcePath);
  assert.ok(helperReport.current_files.includes(checkPanelSourcePath));
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.ok(!helperReport.tracked_files.includes(legacyGiantCliModSourcePath));
  assert.ok(!Object.hasOwn(receipt.file_hashes, legacyGiantCliModSourcePath));

  assert.match(
    receipt.file_hashes[previewManifestMaterializerPath],
    /^[a-f0-9]{64}$/,
  );
  assert.match(receipt.file_hashes[studioManifestSourcePath], /^[a-f0-9]{64}$/);
  assert.match(receipt.file_hashes[lowerDxCheckSourcePath], /^[a-f0-9]{64}$/);
  assert.match(receipt.file_hashes[checkPanelSourcePath], /^[a-f0-9]{64}$/);
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id ===
          "automation-connectors-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath],
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-studio-manifest-source" &&
        surface.file_hashes?.[studioManifestSourcePath],
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-lower-dx-check-source" &&
        surface.file_hashes?.[lowerDxCheckSourcePath],
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-check-panel-source" &&
        surface.file_hashes?.[checkPanelSourcePath],
    ),
  );
});

test("Automation Connectors receipt helper rejects stale mirrors for any hashed receipt file", () => {
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-automation-connectors-mirror-stale-"),
  );
  try {
    const selectedFile = "examples/template/automations-status.tsx";
    const selectedFiles = [
      selectedFile,
      sourceGuardRunbookPath,
      previewManifestMaterializerPath,
      studioManifestSourcePath,
      lowerDxCheckSourcePath,
      checkPanelSourcePath,
    ];
    const fileContents = new Map(
      selectedFiles.map((selectedPath) => [
        selectedPath,
        `// ${selectedPath}\nexport const receiptMirror = ${JSON.stringify(selectedPath)};\n`,
      ]),
    );
    for (const [selectedPath, contents] of fileContents) {
      writeFile(fixtureRoot, selectedPath, contents);
    }
    const hashes = Object.fromEntries(
      [...fileContents].map(([selectedPath, contents]) => [
        selectedPath,
        sha256(contents),
      ]),
    );

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "automations/n8n",
      official_package_name: "Automation Connectors",
      upstream_package: "n8n-nodes-base",
      upstream_version: "2.22.0",
      hash_algorithm: "sha256",
      file_hashes: hashes,
      dx_check_visibility: {
        monitored_surfaces: selectedFiles.map((selectedPath) => ({
          id:
            selectedPath === previewManifestMaterializerPath
              ? "automation-connectors-preview-manifest-materializer"
              : selectedPath === studioManifestSourcePath
                ? "automation-connectors-studio-manifest-source"
                : selectedPath === lowerDxCheckSourcePath
                  ? "automation-connectors-lower-dx-check-source"
                  : selectedPath === checkPanelSourcePath
                    ? "automation-connectors-check-panel-source"
                    : selectedPath === sourceGuardRunbookPath
                      ? "automation-connectors-source-guard-runbook"
                      : "automation-launch-dashboard-workflow",
          hash_algorithm: "sha256",
          file_hashes: {
            [selectedPath]: selectedPath === selectedFile ? "stale" : hashes[selectedPath],
          },
        })),
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          package_id: "automations/n8n",
          selected_surfaces: selectedFiles.map((selectedPath) => ({
            surface_id: "automation-launch-dashboard-workflow",
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedPath]: selectedPath === selectedFile ? "stale" : hashes[selectedPath],
            },
          })),
        },
      ],
      zed_receipt_surfaces: [],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    writeFile(
      fixtureRoot,
      readModelPath,
      [
        "export const automationConnectorsPackageVisibility = {",
        '  packageId: "automations/n8n",',
        "  selectedSurfaces: [",
        ...selectedFiles.flatMap((selectedPath) => [
          "    {",
          "      fileHashes: {",
          `        "${selectedPath}": "${selectedPath === selectedFile ? "stale" : hashes[selectedPath]}",`,
          "      },",
          "    },",
        ]),
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
      ].join("\n"),
    );

    const result = runHelper(["--root", fixtureRoot, "--check", "--json"]);

    assert.notEqual(result.status, 0, result.stdout + result.stderr);
    const report = JSON.parse(result.stdout);
    assert.deepEqual(report.stale_files, []);
    assert.ok(report.stale_mirror_files.includes(selectedFile));
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

function sha256(contents) {
  return crypto.createHash("sha256").update(contents).digest("hex");
}
