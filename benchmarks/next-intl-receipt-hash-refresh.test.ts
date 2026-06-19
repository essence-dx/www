const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "examples/template/internationalization-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";
const runbookFixturePath = "docs/packages/next-intl.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const catalogValidationFiles = [
  "examples/template/app/i18n/page.tsx",
  "examples/template/components/template-app/i18n-page.tsx",
  "examples/template/i18n/catalog-validation.ts",
  "examples/template/i18n/messages/bn.json",
  "examples/template/i18n/messages/en.json",
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

test("Internationalization exposes a package-owned receipt hash refresh helper", () => {
  const helperSource = read(helperPath);
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const packageDoc = read("docs/packages/next-intl.md");

  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "Internationalization"/);
  assert.match(helperSource, /PACKAGE_ID = "i18n\/next-intl"/);
  assert.match(helperSource, /UPSTREAM_PACKAGE = "next-intl"/);
  assert.match(helperSource, /SOURCE_GUARD_RUNBOOK_FIXTURE/);
  assert.match(helperSource, /PREVIEW_MANIFEST_MATERIALIZER/);
  assert.doesNotMatch(helperSource, /fetch\(|localStorage|sessionStorage/);

  assert.equal(receipt.package_id, "i18n/next-intl");
  assert.equal(receipt.official_package_name, "Internationalization");
  assert.equal(receipt.upstream_package, "next-intl");
  assert.equal(receipt.upstream_version, "4.12.0");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/next-intl");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.file_hashes, "Internationalization receipt is missing file_hashes");

  const trackedFiles = Object.keys(receipt.file_hashes);
  assert.deepEqual(trackedFiles.sort(), [
    "core/src/ecosystem/forge_next_intl.rs",
    runbookFixturePath,
    ...catalogValidationFiles,
    "examples/template/next-intl-dashboard-locale-contract.ts",
    "examples/template/next-intl-dashboard-locale.tsx",
    previewManifestMaterializerPath,
  ].sort());
  assert.ok(
    receipt.source_files.includes(previewManifestMaterializerPath),
    "Internationalization receipt must list the preview-manifest materializer as source",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "internationalization-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath],
    ),
    "Internationalization receipt must monitor the preview-manifest materializer hash",
  );

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "i18n/next-intl",
  );
  assert.ok(visibility, "Internationalization package-status row is missing");
  assert.equal(visibility.official_package_name, "Internationalization");
  assert.equal(visibility.upstream_package, "next-intl");

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Internationalization receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperPath);
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    hashRefresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    hashRefresh.tracked_files.includes(previewManifestMaterializerPath),
    "Internationalization receipt_hash_refresh must list the preview-manifest materializer",
  );
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, trackedFiles.length);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "internationalization:receipt-hash-refresh");

  assert.ok(
    status.zed_receipt_surfaces.includes(
      "internationalization:receipt-hash-refresh",
    ),
    "Internationalization helper is missing from Zed receipt surfaces",
  );
  assert.match(readModel, /export const internationalizationPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/next-intl\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(readModel, /internationalization:receipt-hash-refresh/);
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "internationalization-source-guard-runbook" &&
        surface.file_hashes?.[runbookFixturePath] ===
          receipt.file_hashes[runbookFixturePath],
    ),
    "Internationalization package-status must mirror the source-guard runbook fixture hash",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "internationalization-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath] ===
          receipt.file_hashes[previewManifestMaterializerPath],
    ),
    "Internationalization package-status must mirror the preview-manifest materializer hash",
  );
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /source_guard_runbook_fixture/);
  assert.match(
    packageDoc,
    /internationalization-receipt-hashes\.ts --check/,
  );

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "i18n/next-intl");
  assert.equal(helperReport.official_package_name, "Internationalization");
  assert.equal(helperReport.upstream_package, "next-intl");
  assert.equal(helperReport.upstream_version, "4.12.0");
  assert.equal(helperReport.source_mirror, "G:/WWW/inspirations/next-intl");
  assert.equal(helperReport.status, "current");
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(helperReport.tracked_file_count, trackedFiles.length);
  assert.ok(
    helperReport.tracked_files.includes(previewManifestMaterializerPath),
    "Internationalization helper report must list the preview-manifest materializer",
  );
  assert.ok(
    helperReport.files.some((entry) => entry.path === runbookFixturePath),
    "Internationalization helper report must include the runbook fixture path",
  );
  assert.ok(
    helperReport.files.some(
      (entry) => entry.path === previewManifestMaterializerPath,
    ),
    "Internationalization helper report must include the preview-manifest materializer path",
  );
  assert.equal(helperReport.stale_file_count, 0);
  assert.equal(helperReport.missing_file_count, 0);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
  assert.equal(
    helperReport.zed_visibility,
    "internationalization:receipt-hash-refresh",
  );
});

test("Internationalization receipt hash helper refreshes stale mirrors", () => {
  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-intl-hashes-"));
  try {
    const selectedFiles = [
      "examples/template/next-intl-dashboard-locale-contract.ts",
      "examples/template/next-intl-dashboard-locale.tsx",
      "core/src/ecosystem/forge_next_intl.rs",
      runbookFixturePath,
      previewManifestMaterializerPath,
      ...catalogValidationFiles,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const internationalizationHashFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "i18n/next-intl",
      package_name: "Internationalization",
      official_package_name: "Internationalization",
      upstream_package: "next-intl",
      upstream_version: "4.12.0",
      source_mirror: "G:/WWW/inspirations/next-intl",
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
          official_package_name: "Internationalization",
          package_id: "i18n/next-intl",
          upstream_package: "next-intl",
          upstream_version: "4.12.0",
          source_mirror: "G:/WWW/inspirations/next-intl",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "next-intl-dashboard-locale-workflow",
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
            {
              surface_id: "next-intl-dashboard-message-contract",
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
            {
              surface_id: "internationalization-source-guard-runbook",
              hash_algorithm: "sha256",
              file_hashes: {
                [runbookFixturePath]: "stale",
              },
            },
          ],
        },
      ],
      zed_receipt_surfaces: [
        "internationalization:next-intl-dashboard-locale-workflow",
      ],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "const internationalizationFileHashes = {",
        ...selectedFiles.flatMap((selectedFile) => [
          `  "${selectedFile}":`,
          '    "stale",',
        ]),
        "} as const;",
        "",
        "export const internationalizationPackageVisibility = {",
        '  officialName: "Internationalization",',
        '  packageId: "i18n/next-intl",',
        '  upstreamPackage: "next-intl",',
        '  upstreamVersion: "4.12.0",',
        '  sourceMirror: "G:/WWW/inspirations/next-intl",',
        '  packageReceiptPath:',
        `    "${receiptPath}",`,
        "  selectedSurfaces: [",
        "    { fileHashes: internationalizationFileHashes },",
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.status, "current");
    assert.equal(report.zed_visibility, "internationalization:receipt-hash-refresh");
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);

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
      const mirroredSurfaces = refreshedStatus.package_lane_visibility[0]
        .selected_surfaces.filter((surface) =>
          Object.prototype.hasOwnProperty.call(
            surface.file_hashes ?? {},
            selectedFile,
          ),
        );
      assert.ok(
        mirroredSurfaces.length > 0,
        `${selectedFile} missing from package-status mirrors`,
      );
      for (const surface of mirroredSurfaces) {
        assert.equal(surface.file_hashes[selectedFile], refreshedHash);
      }
    }

    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path: helperPath,
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        preview_manifest_materializer: previewManifestMaterializerPath,
        receipt_path: receiptPath,
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        tracked_files: selectedFiles,
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "internationalization:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Internationalization receipt hash freshness only.",
          "ADAPTER-BOUNDARY: locale routing, translation quality, SEO alternates, middleware placement, and runtime dependency installation stay app-owned.",
        ],
      },
    );
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "internationalization-source-guard-runbook" &&
          surface.file_hashes?.[runbookFixturePath] ===
            refreshedReceipt.file_hashes[runbookFixturePath],
      ),
      "Internationalization runbook fixture surface should refresh its hash",
    );
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "internationalization-preview-manifest-materializer" &&
          surface.file_hashes?.[previewManifestMaterializerPath] ===
            refreshedReceipt.file_hashes[previewManifestMaterializerPath],
      ),
      "Internationalization materializer surface should refresh its hash",
    );
    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes(
        "internationalization:receipt-hash-refresh",
      ),
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});
