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
  "examples/template/validation-schemas-receipt-hashes.ts",
);
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const selectedSettingsValidationPaths = [
  "examples/template/zod-dashboard-settings.tsx",
  "examples/dashboard/src/lib/forge/validation/zod/dashboard-settings.ts",
  "docs/packages/validation-schemas.source-guard-runbook.json",
];
const trackedHashPaths = [
  ...selectedSettingsValidationPaths,
  previewManifestMaterializerPath,
];

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

function copySourceIntoFixture(fixtureRoot, relativePath) {
  const source = path.join(root, relativePath);
  const target = path.join(fixtureRoot, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(source, target);
  return crypto.createHash("sha256").update(fs.readFileSync(source)).digest("hex");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("Validation & Schemas receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "Validation & Schemas hash helper is missing",
  );

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-validation-schemas-hashes-"),
  );
  try {
    const selectedFiles = [
      "examples/template/package-catalog.ts",
      "tools/launch/runtime-template/pages/index.html",
      "tools/launch/materialize-www-template.ts",
      "docs/packages/validation-schemas.source-guard-runbook.json",
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const validationSchemasFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "validation/zod",
      package_name: "Validation & Schemas",
      official_dx_package_name: "Validation & Schemas",
      upstream_package: "zod",
      upstream_version: "4.4.3",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Validation & Schemas",
          package_id: "validation/zod",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "launch-package-catalog-validation",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[0]]: "stale",
              },
            },
            {
              surface_id: "dashboard-settings-validation",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[1]]: "stale",
              },
            },
            {
              surface_id: "generated-starter-materialization",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[2]]: "stale",
              },
            },
            {
              surface_id: "validation-schemas-source-guard-runbook",
              surface_type: "source_guard_runbook_fixture",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[3]]: "stale",
              },
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
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "const validationSchemasFileHashes = {",
        `  "${selectedFiles[0]}":`,
        '    "stale",',
        `  "${selectedFiles[1]}":`,
        '    "stale",',
        `  "${selectedFiles[2]}":`,
        '    "stale",',
        `  "${selectedFiles[3]}":`,
        '    "stale",',
        "} as const;",
        "",
        "export const validationSchemasPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    status: "stale",',
        "  },",
        "  statusVocabulary: [],",
        "  selectedSurfaces: [],",
        "} as const satisfies LaunchForgePackageLaneVisibility;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /package-catalog\.ts/);
    assert.match(stale.stdout + stale.stderr, /tools\\launch\\runtime-template\\pages\\/index\.html/);
    assert.match(stale.stdout + stale.stderr, /materialize-www-template\.ts/);
    assert.match(
      stale.stdout + stale.stderr,
      /validation-schemas\.source-guard-runbook\.json/,
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.official_package_name, "Validation & Schemas");
    assert.equal(report.package_id, "validation/zod");
    assert.equal(report.status, "current");
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.equal(report.zed_visibility, "validation-schemas:receipt-hash-refresh");
    assert.equal(
      report.source_guard_runbook_fixture,
      "docs/packages/validation-schemas.source-guard-runbook.json",
    );
    assert.equal(report.tracked_file_count, 4);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.equal(
        refreshedStatus.package_lane_visibility[0].source_hashes.files[
          selectedFile
        ],
        refreshedHash,
      );
      assert.match(readModelText, new RegExp(refreshedHash));
    }
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[0]
        .file_hashes[selectedFiles[0]],
      refreshedReceipt.file_hashes[selectedFiles[0]],
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[1]
        .file_hashes[selectedFiles[1]],
      refreshedReceipt.file_hashes[selectedFiles[1]],
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[2]
        .file_hashes[selectedFiles[2]],
      refreshedReceipt.file_hashes[selectedFiles[2]],
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[3]
        .file_hashes[selectedFiles[3]],
      refreshedReceipt.file_hashes[selectedFiles[3]],
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh
        .source_guard_runbook_fixture,
      selectedFiles[3],
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Validation & Schemas helper reports materializer-only stale paths without blaming settings validation sources", () => {
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-validation-schemas-materializer-stale-"),
  );
  try {
    const currentHashes = new Map(
      trackedHashPaths.map((relativePath) => [
        relativePath,
        copySourceIntoFixture(fixtureRoot, relativePath),
      ]),
    );
    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json";
    const receiptHashes = Object.fromEntries(
      trackedHashPaths.map((relativePath) => [
        relativePath,
        relativePath === previewManifestMaterializerPath
          ? "0".repeat(64)
          : currentHashes.get(relativePath),
      ]),
    );

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "validation/zod",
      package_name: "Validation & Schemas",
      official_dx_package_name: "Validation & Schemas",
      upstream_package: "zod",
      upstream_version: "4.4.3",
      hash_algorithm: "sha256",
      file_hashes: receiptHashes,
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    const currentFileHashes = Object.fromEntries(currentHashes);
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Validation & Schemas",
          package_id: "validation/zod",
          package_receipt_path: receiptPath,
          receipt_hash_refresh: {
            schema: "dx.forge.package.receipt_hash_refresh",
            status: "current",
          },
          selected_surfaces: [
            {
              surface_id: "validation-schemas-materializer-stale-fixture",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: currentFileHashes,
            },
          ],
          source_hashes: {
            algorithm: "sha256",
            files: currentFileHashes,
          },
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
        "const validationSchemasFileHashes = {",
        ...trackedHashPaths.flatMap((relativePath) => [
          `  "${relativePath}":`,
          `    "${currentHashes.get(relativePath)}",`,
        ]),
        "} as const;",
        "",
        "export const validationSchemasPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    status: "current",',
        "  },",
        "  statusVocabulary: [],",
        "  selectedSurfaces: [],",
        "} as const satisfies LaunchForgePackageLaneVisibility;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.preview_manifest_materializer, previewManifestMaterializerPath);
    assert.equal(staleReport.tracked_file_count, 4);
    assert.equal(staleReport.stale_file_count, 1);
    assert.equal(staleReport.missing_file_count, 0);
    assert.deepEqual(staleReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(staleReport.missing_files, []);
    assert.deepEqual(
      staleReport.current_files.sort(),
      selectedSettingsValidationPaths.slice().sort(),
    );

    const plain = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(plain.status, 0, plain.stdout + plain.stderr);
    const plainOutput = plain.stdout + plain.stderr;
    assert.match(
      plainOutput,
      new RegExp(escapeRegExp(previewManifestMaterializerPath)),
    );
    for (const selectedSourcePath of selectedSettingsValidationPaths) {
      assert.doesNotMatch(
        plainOutput,
        new RegExp(escapeRegExp(selectedSourcePath)),
      );
    }
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Validation & Schemas docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/validation-zod.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/validation-schemas-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run browser runtime proof/i);
});
