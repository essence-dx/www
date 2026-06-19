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
  "examples/template/type-safe-api-receipt-hashes.ts",
);
const helperRelativePath =
  "examples/template/type-safe-api-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json";
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath =
  "examples/template/forge-package-status-read-model.ts";
const sourceGuardRunbookFixturePath =
  "docs/packages/api-trpc.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const templateReadinessReceiptPath =
  "examples/template/.dx/forge/template-readiness/database-api.json";
const lockBackedSourcePaths = [
  "examples/template/app/api/trpc/health/route.ts",
  "examples/template/lib/trpc/metadata.ts",
  "examples/template/lib/trpc/README.md",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

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

function sha256FixtureFile(fixtureRoot, relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(fixtureRoot, relativePath)))
    .digest("hex");
}

function renderFixtureReadModel(selectedFiles, fileHashes) {
  return [
    "const typeSafeApiFileHashes = {",
    ...selectedFiles.flatMap((selectedFile) => [
      `  ${JSON.stringify(selectedFile)}:`,
      `    ${JSON.stringify(fileHashes[selectedFile])},`,
    ]),
    "} as const;",
    "",
    "export const typeSafeApiPackageVisibility = {",
    "  receiptHashRefresh: {",
    '    schema: "dx.forge.package.receipt_hash_refresh",',
    '    status: "stale",',
    `    helperPath: ${JSON.stringify(helperRelativePath)},`,
    `    checkCommand: ${JSON.stringify(`node ${helperRelativePath} --check`)},`,
    `    writeCommand: ${JSON.stringify(`node ${helperRelativePath} --write`)},`,
    `    jsonCheckCommand: ${JSON.stringify(
      `node ${helperRelativePath} --check --json`,
    )},`,
    `    receiptPath: ${JSON.stringify(receiptPath)},`,
    '    hashAlgorithm: "sha256",',
    `    sourceGuardRunbookFixture: ${JSON.stringify(
      sourceGuardRunbookFixturePath,
    )},`,
    `    previewManifestMaterializer: ${JSON.stringify(
      previewManifestMaterializerPath,
    )},`,
    `    trackedFileCount: ${selectedFiles.length},`,
    "    staleFileCount: 0,",
    "    missingFileCount: 0,",
    "    trackedFiles: [",
    ...selectedFiles.map((selectedFile) => `      ${JSON.stringify(selectedFile)},`),
    "    ],",
    "    currentFiles: [],",
    "    staleFiles: [],",
    "    missingFiles: [],",
    "    staleMirrorFiles: [],",
    "    missingMirrorFiles: [],",
    "    mirrorProblemCount: 0,",
    "    runtimeExecution: false,",
    "    secretAccess: false,",
    '    zedVisibility: "type-safe-api:receipt-hash-refresh",',
    "    runtimeLimitations: [],",
    "  },",
    "  statusVocabulary: [",
    '    "present",',
    '    "stale",',
    '    "missing-receipt",',
    '    "blocked",',
    '    "unsupported-surface",',
    "  ],",
    "} as const satisfies LaunchForgePackageLaneVisibility;",
    "",
  ].join("\n");
}

test("Type-Safe API hash helper tracks runbook fixture and preview-manifest materializer freshness", () => {
  assert.ok(fs.existsSync(helperPath), "Type-Safe API hash helper is missing");

  const helperSource = read(helperRelativePath);
  const receipt = readJson(receiptPath);
  const status = readJson(packageStatusPath);
  const readModel = read(readModelPath);
  const packageDoc = read("docs/packages/api-trpc.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");

  assert.match(helperSource, /SOURCE_GUARD_RUNBOOK_FIXTURE/);
  assert.match(helperSource, /PREVIEW_MANIFEST_MATERIALIZER/);

  assert.equal(receipt.package_id, "api/trpc");
  assert.equal(receipt.official_dx_package_name, "Type-Safe API");
  assert.equal(receipt.upstream_package, "@trpc/server");
  assert.equal(receipt.upstream_version, "11.17.0");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/trpc");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.file_hashes, "Type-Safe API receipt is missing file_hashes");

  const trackedFiles = Object.keys(receipt.file_hashes);
  const expectedTrackedFiles = [
    "core/src/ecosystem/forge_trpc.rs",
    sourceGuardRunbookFixturePath,
    "examples/dashboard/src/components/TrpcDashboardWorkflow.tsx",
    "examples/dashboard/src/lib/trpcDashboardWorkflow.ts",
    templateReadinessReceiptPath,
    ...lockBackedSourcePaths,
    "examples/template/trpc-launch-contract.ts",
    "examples/template/trpc-launch-health.tsx",
    previewManifestMaterializerPath,
  ];
  assert.deepEqual(trackedFiles.sort(), expectedTrackedFiles.slice().sort());

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "api/trpc",
  );
  assert.ok(visibility, "Type-Safe API package-status row is missing");
  assert.equal(visibility.official_package_name, "Type-Safe API");
  assert.equal(visibility.upstream_package, "@trpc/server");
  assert.equal(visibility.source_hashes.algorithm, "sha256");
  assert.deepEqual(Object.keys(visibility.source_hashes.files).sort(), trackedFiles.sort());

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Type-Safe API receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperRelativePath);
  assert.equal(hashRefresh.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
  assert.equal(hashRefresh.preview_manifest_materializer, previewManifestMaterializerPath);
  assert.equal(hashRefresh.tracked_file_count, trackedFiles.length);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "type-safe-api:receipt-hash-refresh");

  const surfaceIds = visibility.selected_surfaces.map((surface) => surface.surface_id);
  assert.ok(surfaceIds.includes("type-safe-api-source-guard-runbook"));
  assert.ok(surfaceIds.includes("type-safe-api-preview-manifest-materializer"));
  assert.ok(surfaceIds.includes("type-safe-api-lock-backed-source"));

  assert.match(readModel, /sourceGuardRunbookFixture/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(readModel, /type-safe-api-source-guard-runbook/);
  assert.match(readModel, /type-safe-api-preview-manifest-materializer/);
  assert.match(readModel, /type-safe-api-lock-backed-source/);
  assert.match(readModel, /app\/api\/trpc\/health\/route\.ts/);
  assert.match(readModel, /lib\/trpc\/metadata\.ts/);
  assert.match(readModel, /lib\/trpc\/README\.md/);
  assert.match(readModel, /docs\/packages\/api-trpc\.source-guard-runbook\.json/);
  assert.match(readModel, /tools\/launch\/materialize-www-template\.ts/);
  assert.match(packageDoc, /source_guard_runbook_fixture/);
  assert.match(packageDoc, /preview_manifest_materializer/);
  assert.match(packageDoc, /stale_files/);
  assert.match(packageDoc, /eleven selected source-owned files/i);
  assert.match(frameworkDoc, /receipt hash helper tracks the fixture, preview-manifest materializer/);

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.package_id, "api/trpc");
  assert.equal(helperReport.official_package_name, "Type-Safe API");
  assert.equal(helperReport.status, "current");
  assert.equal(helperReport.tracked_file_count, trackedFiles.length);
  assert.deepEqual(helperReport.tracked_files.sort(), trackedFiles.sort());
  assert.deepEqual(helperReport.current_files.sort(), trackedFiles.sort());
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.deepEqual(helperReport.stale_mirror_files, []);
  assert.deepEqual(helperReport.missing_mirror_files, []);
  assert.equal(helperReport.mirror_problem_count, 0);
  assert.equal(helperReport.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
  assert.equal(helperReport.preview_manifest_materializer, previewManifestMaterializerPath);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
});

test("Type-Safe API hash helper isolates runbook and materializer drift from route and dashboard files", () => {
  assert.ok(fs.existsSync(helperPath), "Type-Safe API hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-type-safe-api-helper-only-drift-"),
  );
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_trpc.rs",
      "examples/template/trpc-launch-health.tsx",
      "examples/template/trpc-launch-contract.ts",
      "examples/dashboard/src/components/TrpcDashboardWorkflow.tsx",
      "examples/dashboard/src/lib/trpcDashboardWorkflow.ts",
      sourceGuardRunbookFixturePath,
      previewManifestMaterializerPath,
      templateReadinessReceiptPath,
      ...lockBackedSourcePaths,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const typeSafeApiFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const currentHashes = Object.fromEntries(
      selectedFiles.map((selectedFile) => [
        selectedFile,
        sha256FixtureFile(fixtureRoot, selectedFile),
      ]),
    );
    const staleRunbookHash = "0".repeat(64);
    const staleMaterializerHash = "1".repeat(64);
    const seededHashes = {
      ...currentHashes,
      [sourceGuardRunbookFixturePath]: staleRunbookHash,
      [previewManifestMaterializerPath]: staleMaterializerHash,
    };

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "api/trpc",
      package_name: "Type-Safe API",
      official_dx_package_name: "Type-Safe API",
      upstream_package: "@trpc/server",
      upstream_version: "11.17.0",
      hash_algorithm: "sha256",
      file_hashes: seededHashes,
    });

    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Type-Safe API",
          package_id: "api/trpc",
          package_receipt_path: receiptPath,
          receipt_hash_refresh: {
            schema: "dx.forge.package.receipt_hash_refresh",
            status: "stale",
            helper_path: helperRelativePath,
            source_guard_runbook_fixture: sourceGuardRunbookFixturePath,
            preview_manifest_materializer: previewManifestMaterializerPath,
            tracked_file_count: selectedFiles.length,
            stale_file_count: 2,
            missing_file_count: 0,
            runtime_execution: false,
            secret_access: false,
            zed_visibility: "type-safe-api:receipt-hash-refresh",
          },
          selected_surfaces: [
            {
              surface_id: "trpc-route-handler",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[0]]: currentHashes[selectedFiles[0]],
              },
            },
            {
              surface_id: "trpc-launch-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[1]]: currentHashes[selectedFiles[1]],
                [selectedFiles[2]]: currentHashes[selectedFiles[2]],
              },
            },
            {
              surface_id: "trpc-starter-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[3]]: currentHashes[selectedFiles[3]],
                [selectedFiles[4]]: currentHashes[selectedFiles[4]],
              },
            },
            {
              surface_id: "type-safe-api-source-guard-runbook",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[5]]: staleRunbookHash,
              },
            },
            {
              surface_id: "type-safe-api-preview-manifest-materializer",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[6]]: staleMaterializerHash,
              },
            },
            {
              surface_id: "type-safe-api-template-readiness-receipt",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [templateReadinessReceiptPath]:
                  currentHashes[templateReadinessReceiptPath],
              },
            },
            {
              surface_id: "type-safe-api-lock-backed-source",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                lockBackedSourcePaths.map((selectedFile) => [
                  selectedFile,
                  currentHashes[selectedFile],
                ]),
              ),
            },
          ],
          source_hashes: {
            algorithm: "sha256",
            files: seededHashes,
          },
        },
      ],
    });

    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      renderFixtureReadModel(selectedFiles, seededHashes),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.status, "stale");
    assert.deepEqual(staleReport.tracked_files.sort(), selectedFiles.slice().sort());
    assert.deepEqual(
      staleReport.stale_files.sort(),
      [sourceGuardRunbookFixturePath, previewManifestMaterializerPath].sort(),
    );
    assert.deepEqual(staleReport.missing_files, []);

    const staleText = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(staleText.status, 0, staleText.stdout + staleText.stderr);
    assert.match(staleText.stderr, /api-trpc\.source-guard-runbook\.json/);
    assert.match(staleText.stderr, /materialize-www-template\.ts/);
    assert.doesNotMatch(staleText.stderr, /trpc-launch-health\.tsx/);
    assert.doesNotMatch(staleText.stderr, /trpc-launch-contract\.ts/);
    assert.doesNotMatch(staleText.stderr, /TrpcDashboardWorkflow\.tsx/);
    assert.doesNotMatch(staleText.stderr, /trpcDashboardWorkflow\.ts/);
    assert.doesNotMatch(staleText.stderr, /forge_trpc\.rs/);
    assert.doesNotMatch(staleText.stderr, /app\/api\/trpc\/health\/route\.ts/);
    assert.doesNotMatch(staleText.stderr, /lib\/trpc\/metadata\.ts/);
    assert.doesNotMatch(staleText.stderr, /lib\/trpc\/README\.md/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const refreshedReadModel = fs.readFileSync(absoluteReadModelPath, "utf8");

    for (const selectedFile of selectedFiles) {
      assert.equal(refreshedReceipt.file_hashes[selectedFile], currentHashes[selectedFile]);
      assert.equal(
        refreshedStatus.package_lane_visibility[0].source_hashes.files[
          selectedFile
        ],
        currentHashes[selectedFile],
      );
      assert.match(refreshedReadModel, new RegExp(currentHashes[selectedFile]));
    }

    const routeAndDashboardFiles = selectedFiles.slice(0, 5);
    for (const selectedFile of routeAndDashboardFiles) {
      assert.equal(refreshedReceipt.file_hashes[selectedFile], seededHashes[selectedFile]);
    }

    assert.notEqual(
      refreshedReceipt.file_hashes[sourceGuardRunbookFixturePath],
      staleRunbookHash,
    );
    assert.notEqual(
      refreshedReceipt.file_hashes[previewManifestMaterializerPath],
      staleMaterializerHash,
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Type-Safe API receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Type-Safe API hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-type-safe-api-hashes-"),
  );
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_trpc.rs",
      "examples/template/trpc-launch-health.tsx",
      "examples/template/trpc-launch-contract.ts",
      "examples/dashboard/src/components/TrpcDashboardWorkflow.tsx",
      "examples/dashboard/src/lib/trpcDashboardWorkflow.ts",
      sourceGuardRunbookFixturePath,
      previewManifestMaterializerPath,
      templateReadinessReceiptPath,
      ...lockBackedSourcePaths,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const typeSafeApiFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "api/trpc",
      package_name: "Type-Safe API",
      official_dx_package_name: "Type-Safe API",
      upstream_package: "@trpc/server",
      upstream_version: "11.17.0",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
    });

    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Type-Safe API",
          package_id: "api/trpc",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "trpc-route-handler",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[0]]: "stale",
              },
            },
            {
              surface_id: "trpc-launch-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[1]]: "stale",
                [selectedFiles[2]]: "stale",
              },
            },
            {
              surface_id: "trpc-starter-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[3]]: "stale",
                [selectedFiles[4]]: "stale",
              },
            },
            {
              surface_id: "type-safe-api-source-guard-runbook",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[5]]: "stale",
              },
            },
            {
              surface_id: "type-safe-api-preview-manifest-materializer",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[6]]: "stale",
              },
            },
            {
              surface_id: "type-safe-api-template-readiness-receipt",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [templateReadinessReceiptPath]: "stale",
              },
            },
            {
              surface_id: "type-safe-api-lock-backed-source",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                lockBackedSourcePaths.map((selectedFile) => [selectedFile, "stale"]),
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
    });

    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    const staleHashes = Object.fromEntries(
      selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
    );
    fs.writeFileSync(
      absoluteReadModelPath,
      renderFixtureReadModel(selectedFiles, staleHashes),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /trpc-launch-health\.tsx/);
    assert.match(stale.stdout + stale.stderr, /TrpcDashboardWorkflow\.tsx/);
    assert.match(stale.stdout + stale.stderr, /api-trpc\.source-guard-runbook\.json/);
    assert.match(stale.stdout + stale.stderr, /materialize-www-template\.ts/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.official_package_name, "Type-Safe API");
    assert.equal(report.package_id, "api/trpc");
    assert.equal(report.status, "current");
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.equal(report.zed_visibility, "type-safe-api:receipt-hash-refresh");
    assert.equal(report.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
    assert.equal(report.preview_manifest_materializer, previewManifestMaterializerPath);

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
    const refreshedSurfaces =
      refreshedStatus.package_lane_visibility[0].selected_surfaces;
    for (const selectedFile of selectedFiles) {
      assert.ok(
        refreshedSurfaces.some(
          (surface) =>
            surface.file_hashes &&
            surface.file_hashes[selectedFile] === refreshedReceipt.file_hashes[selectedFile],
        ),
        `${selectedFile} is missing from refreshed selected-surface mirrors`,
      );
    }
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Type-Safe API docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/api-trpc.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/type-safe-api-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run live tRPC\s+route execution/i);
});
