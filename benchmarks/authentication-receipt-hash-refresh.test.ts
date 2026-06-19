const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/authentication-receipt-hashes.ts",
);
const runbookFixturePath = "docs/packages/authentication.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";
const receiptPath =
  "examples/template/.dx/forge/receipts/auth-better-auth.json";
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath =
  "examples/template/forge-package-status-read-model.ts";
const appRouteHandlerSource =
  "examples/template/app/api/auth/[...all]/route.ts";
const readinessRouteHandlerSource =
  "examples/template/app/api/auth/readiness/route.ts";
const appServerBoundarySource =
  "examples/template/server/auth/better-auth.ts";
const templateReadinessReceipt =
  "examples/template/.dx/forge/template-readiness/authentication.json";
const selectedAuthenticationReceiptFiles = [
  "examples/template/template-shell.tsx",
  "examples/template/auth-session-status.tsx",
  "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
  "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
  runbookFixturePath,
  previewManifestMaterializerPath,
  studioManifestSourcePath,
  appRouteHandlerSource,
  readinessRouteHandlerSource,
  appServerBoundarySource,
  templateReadinessReceipt,
];
const selectedAuthenticationSourceFiles = selectedAuthenticationReceiptFiles.filter(
  (relativePath) => relativePath !== previewManifestMaterializerPath,
);

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

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function copyFixtureFile(tempRoot, relativePath) {
  const target = path.join(tempRoot, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(root, relativePath), target);
}

function materializerDriftFixture() {
  const tempRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-authentication-receipt-drift-"),
  );
  const receipt = readJson(receiptPath);
  for (const relativePath of Object.keys(receipt.file_hashes)) {
    copyFixtureFile(tempRoot, relativePath);
  }
  copyFixtureFile(tempRoot, receiptPath);
  copyFixtureFile(tempRoot, packageStatusPath);
  copyFixtureFile(tempRoot, readModelPath);

  const refresh = runHelper(["--root", tempRoot, "--write"]);
  if (refresh.status !== 0) {
    throw new Error(refresh.stdout + refresh.stderr);
  }

  fs.appendFileSync(
    path.join(tempRoot, previewManifestMaterializerPath),
    "\n// Authentication helper drift fixture.\n",
  );

  return tempRoot;
}

test("Authentication receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Authentication hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-authentication-hashes-"),
  );
  try {
    const selectedFiles = [...selectedAuthenticationReceiptFiles];
    const accountWorkflowFiles = [
      "examples/template/template-shell.tsx",
      "examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
      "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
    ];
    const boundarySurfaces = [
      {
        surfaceId: "authentication-app-route-handler",
        file: appRouteHandlerSource,
      },
      {
        surfaceId: "authentication-readiness-route-handler",
        file: readinessRouteHandlerSource,
      },
      {
        surfaceId: "authentication-app-server-boundary",
        file: appServerBoundarySource,
      },
      {
        surfaceId: "authentication-template-readiness-receipt",
        file: templateReadinessReceipt,
      },
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const authenticationFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/auth-better-auth.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.receipt",
      package_id: "auth/better-auth",
      package_name: "Authentication",
      upstream_package: "better-auth",
      upstream_version: "1.6.11",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        monitored_surfaces: [
          {
            id: "authentication-account-workflow",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              accountWorkflowFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
          {
            id: "authentication-session-status",
            hash_algorithm: "sha256",
            file_hashes: {
              "examples/template/auth-session-status.tsx": "stale",
            },
          },
          {
            id: "authentication-source-guard-runbook",
            hash_algorithm: "sha256",
            file_hashes: {
              [runbookFixturePath]: "stale",
            },
          },
          {
            id: "authentication-preview-manifest-materializer",
            hash_algorithm: "sha256",
            file_hashes: {
              [previewManifestMaterializerPath]: "stale",
            },
          },
          {
            id: "authentication-studio-manifest-source",
            hash_algorithm: "sha256",
            file_hashes: {
              [studioManifestSourcePath]: "stale",
            },
          },
          ...boundarySurfaces.map((surface) => ({
            id: surface.surfaceId,
            hash_algorithm: "sha256",
            file_hashes: {
              [surface.file]: "stale",
            },
          })),
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Authentication",
          package_id: "auth/better-auth",
          package_receipt_path: ".dx/forge/receipts/auth-better-auth.json",
          selected_surfaces: [
            {
              surface_id: "authentication-account-workflow",
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                accountWorkflowFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
            {
              surface_id: "authentication-session-status",
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: {
                "examples/template/auth-session-status.tsx": "stale",
              },
            },
            {
              surface_id: "authentication-source-guard-runbook",
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [runbookFixturePath]: "stale",
              },
            },
            {
              surface_id: "authentication-preview-manifest-materializer",
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [previewManifestMaterializerPath]: "stale",
              },
            },
            {
              surface_id: "authentication-studio-manifest-source",
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [studioManifestSourcePath]: "stale",
              },
            },
            ...boundarySurfaces.map((surface) => ({
              surface_id: surface.surfaceId,
              receipt_path: ".dx/forge/receipts/auth-better-auth.json",
              hash_algorithm: "sha256",
              file_hashes: {
                [surface.file]: "stale",
              },
            })),
          ],
          receipt_hash_refresh: {
            schema: "dx.forge.package.receipt_hash_refresh",
            status: "stale",
            helper_path:
              "examples/template/authentication-receipt-hashes.ts",
            check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check",
            write_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --write",
            json_check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json",
            source_guard_runbook_fixture: runbookFixturePath,
            studio_manifest_source: studioManifestSourcePath,
            preview_manifest_materializer: previewManifestMaterializerPath,
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            tracked_file_count: selectedFiles.length,
            stale_file_count: selectedFiles.length,
            missing_file_count: 0,
            runtime_execution: false,
            secret_access: false,
            zed_visibility: "authentication:receipt-hash-refresh",
            runtime_limitations: [],
          },
        },
      ],
      zed_receipt_surfaces: ["authentication-package-visibility"],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const stateManagementPackageVisibility = {",
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        '        "examples/template/template-shell.tsx":',
        '          "state-management-stale",',
        "      },",
        "    },",
        "  ],",
        "} as const;",
        "",
        "export const authenticationPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    status: "stale",',
        '    helperPath: "examples/template/authentication-receipt-hashes.ts",',
        "    trackedFileCount: 0,",
        "    staleFileCount: 4,",
        "    missingFileCount: 0,",
        "  },",
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "  ],",
        "} as const satisfies LaunchForgePackageLaneVisibility;",
        "",
        "export const internationalizationPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    helperPath: "examples/template/next-intl-receipt-hashes.ts",',
        '    zedVisibility: "internationalization:receipt-hash-refresh",',
        "  },",
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "auth/better-auth");
    assert.equal(staleReport.official_package_name, "Authentication");
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(staleReport.source_guard_runbook_fixture, runbookFixturePath);
    assert.equal(staleReport.studio_manifest_source, studioManifestSourcePath);
    assert.equal(
      staleReport.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.equal(
      staleReport.zed_visibility,
      "authentication:receipt-hash-refresh",
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Authentication receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.deepEqual(freshReport.tracked_files, selectedFiles);
    assert.deepEqual(freshReport.current_files, selectedFiles);
    assert.deepEqual(freshReport.stale_files, []);
    assert.deepEqual(freshReport.missing_files, []);
    assert.deepEqual(freshReport.stale_mirror_files, []);
    assert.deepEqual(freshReport.missing_mirror_files, []);
    assert.equal(freshReport.mirror_problem_count, 0);
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
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.match(readModelText, new RegExp(refreshedHash));
    }

    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/authentication-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/authentication-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        studio_manifest_source: studioManifestSourcePath,
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
        zed_visibility: "authentication:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Authentication receipt hash freshness only.",
          "ADAPTER-BOUNDARY: Better Auth credentials, provider callbacks, cookies, database adapters, email delivery, and hosted sessions stay app-owned.",
        ],
      },
    );
    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes(
        "authentication:receipt-hash-refresh",
      ),
    );
    assert.match(readModelText, /receiptHashRefresh/);
    assert.match(readModelText, /trackedFiles/);
    assert.match(readModelText, /currentFiles/);
    assert.match(readModelText, /staleFiles/);
    assert.match(readModelText, /missingMirrorFiles/);
    assert.match(readModelText, /mirrorProblemCount:\s*0/);
    assert.match(readModelText, /authentication:receipt-hash-refresh/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture:\s*"docs\/packages\/authentication\.source-guard-runbook\.json"/,
    );
    assert.match(
      readModelText,
      /studioManifestSource:\s*"dx-www\/src\/cli\/studio_manifest\.rs"/,
    );
    assert.match(
      readModelText,
      /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
    );
    assert.match(
      readModelText,
      /next-intl-receipt-hashes\.ts/,
      "Authentication helper must not rewrite the next package lane's hash refresh block",
    );
    assert.match(
      readModelText,
      /state-management-stale/,
      "Authentication helper must not rewrite a previous package lane's shared template-shell hash",
    );
    assert.match(readModelText, /status: "current"/);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Authentication helper attributes materializer drift while account/session/runbook files stay current", () => {
  const tempRoot = materializerDriftFixture();
  try {
    const helper = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.equal(helper.status, 1, helper.stdout + helper.stderr);
    const report = JSON.parse(helper.stdout);

    assert.equal(report.status, "stale");
    assert.equal(report.mirror_problem_count, 3);
    assert.deepEqual(report.tracked_files, selectedAuthenticationReceiptFiles);
    assert.deepEqual(report.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(report.missing_files, []);
    assert.deepEqual(report.stale_mirror_files, [previewManifestMaterializerPath]);
    assert.deepEqual(report.missing_mirror_files, []);

    for (const relativePath of selectedAuthenticationSourceFiles) {
      assert.ok(
        report.current_files.includes(relativePath),
        `${relativePath} should stay current when only the materializer drifts`,
      );
      assert.ok(
        !report.stale_files.includes(relativePath),
        `${relativePath} should not be stale for a materializer-only drift`,
      );
    }
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("Authentication docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/authentication.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/authentication-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(
    packageDoc,
    /does not run Better Auth, read secrets, open a browser, or prove live OAuth/i,
  );
  assert.match(packageDoc, /preview_manifest_materializer/);
  assert.match(packageDoc, /studio_manifest_source/);
  assert.match(packageDoc, /studio_manifest\.rs/);
  assert.match(packageDoc, /stale_files/);
  assert.match(packageDoc, /current_files/);
  assert.match(packageDoc, /materialize-www-template\.ts/);
});
