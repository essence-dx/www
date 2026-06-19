const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/reactive-store-receipt-hashes.ts",
);
const runbookFixturePath =
  "docs/packages/reactive-store.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const templateReactiveDashboardStorePath =
  "examples/template/components/template-app/dashboard-reactive-store.ts";
const selectedReactiveStoreFiles = [
  "lib/forge/state/reactive-store/context.tsx",
  "lib/forge/state/reactive-store/metadata.ts",
  "lib/forge/state/reactive-store/README.md",
  runbookFixturePath,
  previewManifestMaterializerPath,
  templateReactiveDashboardStorePath,
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

function readModelExport(source, exportName) {
  const start = source.indexOf(`export const ${exportName} = {`);
  assert.notEqual(start, -1, `${exportName} export is missing`);
  const nextExport = source.indexOf("\n\nexport const ", start + 1);
  return source.slice(start, nextExport === -1 ? undefined : nextExport);
}

function fixtureSelectedFilePath(fixtureRoot, selectedFile) {
  const relativePath =
    selectedFile.startsWith("docs/") ||
    selectedFile.startsWith("tools/") ||
    selectedFile.startsWith("examples/template/")
      ? selectedFile
      : `examples/template/${selectedFile}`;
  return path.join(fixtureRoot, ...relativePath.split("/"));
}

function writeReactiveStoreFixture(fixtureRoot) {
  for (const selectedFile of selectedReactiveStoreFiles) {
    const selectedFilePath = fixtureSelectedFilePath(fixtureRoot, selectedFile);
    fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
    fs.writeFileSync(
      selectedFilePath,
      `export const reactiveStoreHashFixture = ${JSON.stringify(selectedFile)};\n`,
    );
  }

  const receiptPath =
    "examples/template/.dx/forge/receipts/packages/reactive-store.json";
  writeJson(path.join(fixtureRoot, receiptPath), {
    schema: "dx.forge.reactive_store_receipt",
    official_package_name: "Reactive Store",
    package_id: "reactive/store",
    upstream_package: "@tanstack/store",
    based_on: "@tanstack/react-store",
    source_mirror: "G:/WWW/inspirations/tanstack-store",
    upstream_version: "0.11.0",
    hash_algorithm: "sha256",
    file_hashes: Object.fromEntries(
      selectedReactiveStoreFiles.map((selectedFile) => [selectedFile, "stale"]),
    ),
  });

  const packageStatusPath =
    "examples/template/.dx/forge/package-status.json";
  writeJson(path.join(fixtureRoot, packageStatusPath), {
    zed_receipt_surfaces: ["reactive-store:react-context"],
    package_lane_visibility: [
      {
        official_package_name: "Reactive Store",
        package_id: "reactive/store",
        package_receipt_path:
          ".dx/forge/receipts/packages/reactive-store.json",
        selected_surfaces: [
          {
            surface_id: "react-context",
            receipt_path: ".dx/forge/receipts/packages/reactive-store.json",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedReactiveStoreFiles.map((selectedFile) => [
                selectedFile,
                "stale",
              ]),
            ),
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
      `    "${selectedReactiveStoreFiles[0]}":`,
      '      "stale-unrelated-copy",',
      "  },",
      "};",
      "",
      "export const reactiveStorePackageVisibility = {",
      '  packageId: "reactive/store",',
      '  packageReceiptPath: ".dx/forge/receipts/packages/reactive-store.json",',
      "  selectedSurfaces: [",
      "    {",
      "      fileHashes: {",
      ...selectedReactiveStoreFiles.flatMap((selectedFile) => [
        `        "${selectedFile}":`,
        '          "stale",',
      ]),
      "      },",
      "    },",
      "  ],",
      "  statusVocabulary: [],",
      "};",
      "",
    ].join("\n"),
  );

  return {
    receiptPath,
    packageStatusPath,
    readModelPath,
    absoluteReadModelPath,
  };
}

function materializerDriftFixture() {
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-reactive-store-materializer-drift-"),
  );
  writeReactiveStoreFixture(fixtureRoot);

  const write = runHelper(["--root", fixtureRoot, "--write"]);
  assert.equal(write.status, 0, write.stdout + write.stderr);

  fs.appendFileSync(
    fixtureSelectedFilePath(fixtureRoot, previewManifestMaterializerPath),
    "\n// Reactive Store materializer drift fixture\n",
  );

  return fixtureRoot;
}

test("Reactive Store receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Reactive Store hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-reactive-store-hashes-"),
  );
  try {
    const { receiptPath, packageStatusPath, absoluteReadModelPath } =
      writeReactiveStoreFixture(fixtureRoot);

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /context\.tsx/);
    assert.match(stale.stdout + stale.stderr, /metadata\.ts/);
    assert.match(stale.stdout + stale.stderr, /README\.md/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.schema, "dx.forge.package.receipt_hash_refresh");
    assert.equal(report.official_package_name, "Reactive Store");
    assert.equal(report.package_id, "reactive/store");
    assert.equal(report.upstream_package, "@tanstack/store");
    assert.equal(report.based_on, "@tanstack/react-store");
    assert.equal(report.source_mirror, "G:/WWW/inspirations/tanstack-store");
    assert.equal(report.status, "current");
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.equal(report.zed_visibility, "reactive-store:receipt-hash-refresh");
    assert.equal(report.source_guard_runbook_fixture, runbookFixturePath);
    assert.equal(
      report.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.equal(report.tracked_file_count, selectedReactiveStoreFiles.length);
    assert.ok(report.tracked_files.includes(runbookFixturePath));
    assert.ok(report.tracked_files.includes(previewManifestMaterializerPath));
    assert.deepEqual(report.current_files, selectedReactiveStoreFiles);
    assert.deepEqual(report.stale_files, []);
    assert.deepEqual(report.missing_files, []);
    assert.deepEqual(report.stale_mirror_files, []);
    assert.deepEqual(report.missing_mirror_files, []);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");
    const reactiveStoreReadModel = readModelExport(
      readModelText,
      "reactiveStorePackageVisibility",
    );

    for (const selectedFile of selectedReactiveStoreFiles) {
      assert.match(refreshedReceipt.file_hashes[selectedFile], /^[a-f0-9]{64}$/);
      assert.match(
        reactiveStoreReadModel,
        new RegExp(refreshedReceipt.file_hashes[selectedFile]),
      );
    }
    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/reactive-store-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/reactive-store-receipt-hashes.ts --check --json",
        receipt_path:
          "examples/template/.dx/forge/receipts/packages/reactive-store.json",
        hash_algorithm: "sha256",
        tracked_file_count: selectedReactiveStoreFiles.length,
        tracked_files: selectedReactiveStoreFiles,
        current_files: selectedReactiveStoreFiles,
        stale_files: [],
        missing_files: [],
        stale_mirror_files: [],
        missing_mirror_files: [],
        stale_file_count: 0,
        missing_file_count: 0,
        source_guard_runbook_fixture: runbookFixturePath,
        preview_manifest_materializer: previewManifestMaterializerPath,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "reactive-store:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Reactive Store receipt hash freshness only.",
          "SOURCE-OWNED TEMPLATE STORE: the default dashboard store and snapshot summary execute from template source without node_modules.",
          "ADAPTER-BOUNDARY: app-wide persistence, runtime dependency installation, render-performance review, and browser subscription QA stay app-owned.",
        ],
      },
    );
    assert.match(readModelText, /receiptHashRefresh: \{/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture:\s*"docs\/packages\/reactive-store\.source-guard-runbook\.json"/,
    );
    assert.match(
      readModelText,
      /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
    );
    assert.match(
      readModelText,
      /trackedFiles:\s*\[[\s\S]*?"tools\/launch\/materialize-www-template\.ts"/,
    );
    assert.match(
      readModelText,
      /zedVisibility: "reactive-store:receipt-hash-refresh"/,
    );
    assert.deepEqual(refreshedStatus.zed_receipt_surfaces, [
      "reactive-store:react-context",
      "reactive-store:receipt-hash-refresh",
    ]);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Reactive Store helper attributes materializer drift while react-context sources stay current", () => {
  const fixtureRoot = materializerDriftFixture();
  try {
    const helper = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(helper.status, 1, helper.stdout + helper.stderr);
    const helperReport = JSON.parse(helper.stdout);

    assert.equal(helperReport.status, "stale");
    assert.deepEqual(helperReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(helperReport.missing_files, []);
    assert.ok(
      helperReport.stale_mirror_files.includes(previewManifestMaterializerPath),
      "materializer drift must be attributed as stale mirror evidence",
    );

    for (const relativePath of selectedReactiveStoreFiles.filter(
      (selectedFile) => selectedFile !== previewManifestMaterializerPath,
    )) {
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
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Reactive Store receipt helper tracks the source-guard runbook fixture", () => {
  const receipt = JSON.parse(
    fs.readFileSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/packages/reactive-store.json",
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

  const report = runHelper(["--check", "--json"]);
  assert.equal(report.status, 0, report.stdout + report.stderr);
  const helperReport = JSON.parse(report.stdout);

  assert.ok(
    Object.prototype.hasOwnProperty.call(receipt.file_hashes, runbookFixturePath),
    "Reactive Store receipt must hash the source-guard runbook fixture",
  );
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(helperReport.tracked_file_count, selectedReactiveStoreFiles.length);
  assert.ok(helperReport.tracked_files.includes(runbookFixturePath));
  assert.ok(helperReport.tracked_files.includes(previewManifestMaterializerPath));

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "reactive/store",
  );
  assert.ok(visibility, "Reactive Store package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.tracked_file_count,
    selectedReactiveStoreFiles.length,
  );
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(
      previewManifestMaterializerPath,
    ),
    "Reactive Store receipt_hash_refresh must list the preview-manifest materializer",
  );
  assert.equal(
    visibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "source-guard-runbook-fixture" &&
        surface.file_hashes?.[runbookFixturePath] ===
          receipt.file_hashes[runbookFixturePath],
    ),
    "Reactive Store package-status must mirror the runbook fixture hash",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "reactive-store-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath] ===
          receipt.file_hashes[previewManifestMaterializerPath],
    ),
    "Reactive Store package-status must mirror the materializer hash",
  );
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/reactive-store\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /previewManifestMaterializer:\s*"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(readModel, /trackedFileCount: 6/);
  assert.match(
    readModel,
    /trackedFiles:\s*\[[\s\S]*?"tools\/launch\/materialize-www-template\.ts"/,
  );
});

test("Reactive Store docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/reactive-store.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/reactive-store-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /materialize-www-template\.ts/);
  assert.match(packageDoc, /does not run React browser runtime proof/i);
});
