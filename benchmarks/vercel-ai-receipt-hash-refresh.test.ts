const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/ai-sdk-receipt-hashes.ts",
);
const helperRelativePath = "examples/template/ai-sdk-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json";
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath =
  "examples/template/forge-package-status-read-model.ts";
const sourceGuardRunbookFixturePath =
  "docs/packages/ai-sdk.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const helperFreshnessMetrics = [
  "ai_sdk_receipt_hash_refresh_current",
  "ai_sdk_receipt_hash_refresh_stale",
  "ai_sdk_receipt_hash_refresh_missing",
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

function writeAiSdkHashFixture(fixtureRoot, sourceMirror, selectedFiles) {
  for (const selectedFile of selectedFiles) {
    const filePath = selectedFile.startsWith("upstream:")
      ? path.join(sourceMirror, selectedFile.slice("upstream:".length))
      : path.join(fixtureRoot, selectedFile);
    fs.mkdirSync(path.dirname(filePath), { recursive: true });
    fs.writeFileSync(
      filePath,
      `export const aiSdkFixture = ${JSON.stringify(selectedFile)};\n`,
    );
  }

  writeJson(path.join(fixtureRoot, receiptPath), {
    schema: "dx.forge.package_dashboard_workflow_receipt",
    package_id: "ai/vercel-ai",
    package_name: "AI SDK",
    official_dx_package_name: "AI SDK",
    upstream_package: "ai",
    upstream_version: "7.0.0-canary.146",
    source_mirror: "G:/WWW/inspirations/vercel-ai",
    hash_algorithm: "sha256",
    file_hashes: Object.fromEntries(
      selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
    ),
    source_hashes: Object.fromEntries(
      selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
    ),
  });

  writeJson(path.join(fixtureRoot, packageStatusPath), {
    zed_receipt_surfaces: ["ai-sdk:chat-route"],
    package_lane_visibility: [
      {
        official_package_name: "AI SDK",
        package_id: "ai/vercel-ai",
        upstream_package: "ai",
        upstream_version: "7.0.0-canary.146",
        source_mirror: "G:/WWW/inspirations/vercel-ai",
        package_receipt_path: receiptPath,
        selected_surfaces: [
          {
            surface_id: "ai-chat-route",
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedFiles[0]]: "stale",
              [selectedFiles[5]]: "stale",
            },
          },
          {
            surface_id: "ai-launch-assistant-dashboard-workflow",
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedFiles[1]]: "stale",
              [selectedFiles[2]]: "stale",
              [selectedFiles[3]]: "stale",
              [selectedFiles[6]]: "stale",
            },
          },
          {
            surface_id: "ai-sdk-preview-manifest-materializer",
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            file_hashes: {
              [selectedFiles[4]]: "stale",
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

  const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
  fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
  fs.writeFileSync(
    absoluteReadModelPath,
    [
      "export const aiSdkPackageVisibility = {",
      "  receiptHashRefresh: undefined,",
      "  sourceHashes: {",
      "    algorithm: \"sha256\",",
      "    files: {",
      ...selectedFiles.map((selectedFile) => `      "${selectedFile}": "stale",`),
      "    },",
      "    staleReceiptPolicy: \"source-owned AI SDK files stale the launch assistant receipt\",",
      "  },",
      "  selectedSurfaces: [",
      "    { fileHashes: {",
      ...selectedFiles.map((selectedFile) => `      "${selectedFile}": "stale",`),
      "    } },",
      "  ],",
      "} as const;",
      "",
    ].join("\n"),
  );
}

test("AI SDK exposes receipt hash freshness through package-status and read model", () => {
  assert.ok(fs.existsSync(helperPath), "AI SDK hash helper is missing");

  const helperSource = read(helperRelativePath);
  const receipt = readJson(receiptPath);
  const status = readJson(packageStatusPath);
  const readModel = read(readModelPath);
  const packageDoc = read("docs/packages/ai-vercel-ai.md");

  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "AI SDK"/);
  assert.match(helperSource, /PACKAGE_ID = "ai\/vercel-ai"/);
  assert.match(helperSource, /UPSTREAM_PACKAGE = "ai"/);
  assert.match(helperSource, /SOURCE_MIRROR = "G:\/WWW\/inspirations\/vercel-ai"/);
  assert.match(helperSource, /SOURCE_GUARD_RUNBOOK_FIXTURE/);
  assert.match(helperSource, /PREVIEW_MANIFEST_MATERIALIZER/);
  assert.match(helperSource, /DX_CHECK_METRICS = \[/);
  assert.doesNotMatch(helperSource, /fetch\(|AI_PROVIDER_API_KEY|AI_GATEWAY_API_KEY/);

  assert.equal(receipt.package_id, "ai/vercel-ai");
  assert.equal(receipt.official_dx_package_name, "AI SDK");
  assert.equal(receipt.upstream_package, "ai");
  assert.equal(receipt.upstream_version, "7.0.0-canary.146");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/vercel-ai");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.file_hashes, "AI SDK receipt is missing file_hashes");

  const trackedFiles = Object.keys(receipt.file_hashes);
  assert.deepEqual(trackedFiles.sort(), [
    "core/src/ecosystem/forge_vercel_ai.rs",
    sourceGuardRunbookFixturePath,
    "docs/packages/ai-vercel-ai.md",
    "examples/template/ai-chat-status.tsx",
    previewManifestMaterializerPath,
    "upstream:packages/ai/src/generate-text/stream-text.ts",
    "upstream:packages/ai/src/ui/default-chat-transport.ts",
  ]);

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "ai/vercel-ai",
  );
  assert.ok(visibility, "AI SDK package-status row is missing");
  assert.equal(visibility.official_package_name, "AI SDK");
  assert.equal(visibility.upstream_package, "ai");
  assert.equal(visibility.source_hashes.algorithm, "sha256");
  assert.deepEqual(Object.keys(visibility.source_hashes.files).sort(), trackedFiles.sort());

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "AI SDK receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperRelativePath);
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ai-sdk-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ai-sdk-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/ai-sdk-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
  assert.equal(hashRefresh.preview_manifest_materializer, previewManifestMaterializerPath);
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, trackedFiles.length);
  assert.deepEqual([...hashRefresh.tracked_files].sort(), trackedFiles);
  assert.ok(
    hashRefresh.current_files.includes("examples/template/ai-chat-status.tsx"),
    "AI SDK receipt_hash_refresh must expose current launch assistant files",
  );
  assert.ok(
    hashRefresh.current_files.includes(previewManifestMaterializerPath),
    "AI SDK receipt_hash_refresh must expose the current materializer when fresh",
  );
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.deepEqual(hashRefresh.missing_files, []);
  assert.deepEqual(hashRefresh.stale_mirror_files, []);
  assert.deepEqual(hashRefresh.missing_mirror_files, []);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.mirror_problem_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "ai-sdk:receipt-hash-refresh");

  assert.ok(
    status.zed_receipt_surfaces.includes("ai-sdk:receipt-hash-refresh"),
    "AI SDK helper is missing from Zed receipt surfaces",
  );

  assert.match(readModel, /export const aiSdkPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /sourceGuardRunbookFixture/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(readModel, /docs\/packages\/ai-sdk\.source-guard-runbook\.json/);
  assert.match(readModel, /tools\/launch\/materialize-www-template\.ts/);
  assert.match(readModel, /ai-sdk:receipt-hash-refresh/);
  for (const metric of helperFreshnessMetrics) {
    assert.ok(visibility.dx_check_metrics.includes(metric));
    assert.ok(status.dx_check_metrics.includes(metric));
    assert.match(readModel, new RegExp(metric));
  }
  assert.match(
    readModel,
    /zedReceiptSurfaces:\s*\[[\s\S]*"ai-sdk:receipt-hash-refresh"/,
  );
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /ai-sdk-receipt-hashes\.ts --check/);

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "ai/vercel-ai");
  assert.equal(helperReport.official_package_name, "AI SDK");
  assert.equal(helperReport.upstream_package, "ai");
  assert.equal(helperReport.upstream_version, "7.0.0-canary.146");
  assert.equal(helperReport.source_mirror, "G:/WWW/inspirations/vercel-ai");
  assert.equal(helperReport.status, "current");
  assert.equal(helperReport.tracked_file_count, trackedFiles.length);
  assert.deepEqual([...helperReport.tracked_files].sort(), trackedFiles);
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.deepEqual(helperReport.stale_mirror_files, []);
  assert.deepEqual(helperReport.missing_mirror_files, []);
  assert.equal(helperReport.stale_file_count, 0);
  assert.equal(helperReport.missing_file_count, 0);
  assert.equal(helperReport.mirror_problem_count, 0);
  assert.equal(helperReport.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
  assert.equal(helperReport.preview_manifest_materializer, previewManifestMaterializerPath);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
  assert.equal(helperReport.zed_visibility, "ai-sdk:receipt-hash-refresh");
});

test("AI SDK receipt hash helper refreshes local and upstream mirrors", () => {
  assert.ok(fs.existsSync(helperPath), "AI SDK hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-ai-sdk-hashes-"));
  const sourceMirror = path.join(fixtureRoot, "upstream-vercel-ai");
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_vercel_ai.rs",
      "examples/template/ai-chat-status.tsx",
      "docs/packages/ai-vercel-ai.md",
      sourceGuardRunbookFixturePath,
      previewManifestMaterializerPath,
      "upstream:packages/ai/src/generate-text/stream-text.ts",
      "upstream:packages/ai/src/ui/default-chat-transport.ts",
    ];

    for (const selectedFile of selectedFiles) {
      const filePath = selectedFile.startsWith("upstream:")
        ? path.join(sourceMirror, selectedFile.slice("upstream:".length))
        : path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(filePath), { recursive: true });
      fs.writeFileSync(
        filePath,
        `export const aiSdkFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "ai/vercel-ai",
      package_name: "AI SDK",
      official_dx_package_name: "AI SDK",
      upstream_package: "ai",
      upstream_version: "7.0.0-canary.146",
      source_mirror: "G:/WWW/inspirations/vercel-ai",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      source_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
    });

    writeJson(path.join(fixtureRoot, packageStatusPath), {
      zed_receipt_surfaces: ["ai-sdk:chat-route"],
      package_lane_visibility: [
        {
          official_package_name: "AI SDK",
          package_id: "ai/vercel-ai",
          upstream_package: "ai",
          upstream_version: "7.0.0-canary.146",
          source_mirror: "G:/WWW/inspirations/vercel-ai",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "ai-chat-route",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[0]]: "stale",
                [selectedFiles[5]]: "stale",
              },
            },
            {
              surface_id: "ai-launch-assistant-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[1]]: "stale",
                [selectedFiles[2]]: "stale",
                [selectedFiles[3]]: "stale",
                [selectedFiles[6]]: "stale",
              },
            },
            {
              surface_id: "ai-sdk-preview-manifest-materializer",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [selectedFiles[4]]: "stale",
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

    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const aiSdkPackageVisibility = {",
        "  receiptHashRefresh: undefined,",
        "  sourceHashes: {",
        "    algorithm: \"sha256\",",
        "    files: {",
        ...selectedFiles.map((selectedFile) => `      "${selectedFile}": "stale",`),
        "    },",
        "    staleReceiptPolicy: \"source-owned AI SDK files stale the launch assistant receipt\",",
        "  },",
        "  selectedSurfaces: [",
        "    { fileHashes: {",
        ...selectedFiles.map((selectedFile) => `      "${selectedFile}": "stale",`),
        "    } },",
        "  ],",
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper([
      "--root",
      fixtureRoot,
      "--source-mirror",
      sourceMirror,
      "--check",
    ]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /stream-text\.ts/);
    assert.match(stale.stdout + stale.stderr, /ai-chat-status\.tsx/);

    const write = runHelper([
      "--root",
      fixtureRoot,
      "--source-mirror",
      sourceMirror,
      "--write",
    ]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper([
      "--root",
      fixtureRoot,
      "--source-mirror",
      sourceMirror,
      "--check",
      "--json",
    ]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.package_id, "ai/vercel-ai");
    assert.equal(report.official_package_name, "AI SDK");
    assert.equal(report.status, "current");
    assert.equal(report.zed_visibility, "ai-sdk:receipt-hash-refresh");
    assert.equal(report.source_guard_runbook_fixture, sourceGuardRunbookFixturePath);
    assert.equal(report.preview_manifest_materializer, previewManifestMaterializerPath);
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");

    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes("ai-sdk:receipt-hash-refresh"),
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh.zed_visibility,
      "ai-sdk:receipt-hash-refresh",
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh
        .source_guard_runbook_fixture,
      sourceGuardRunbookFixturePath,
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh
        .preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.match(readModelText, /sourceGuardRunbookFixture/);
    assert.match(readModelText, /previewManifestMaterializer/);
    assert.match(readModelText, /docs\/packages\/ai-sdk\.source-guard-runbook\.json/);
    assert.match(readModelText, /tools\/launch\/materialize-www-template\.ts/);
    for (const metric of helperFreshnessMetrics) {
      assert.ok(refreshedStatus.package_lane_visibility[0].dx_check_metrics.includes(metric));
      assert.ok(refreshedStatus.dx_check_metrics.includes(metric));
      assert.match(readModelText, new RegExp(metric));
    }

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.equal(refreshedReceipt.source_hashes[selectedFile], refreshedHash);
      assert.equal(
        refreshedStatus.package_lane_visibility[0].source_hashes.files[selectedFile],
        refreshedHash,
      );
      assert.match(readModelText, new RegExp(refreshedHash));
    }
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("AI SDK helper attributes materializer drift while selected sources stay current", () => {
  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-ai-sdk-materializer-"));
  const sourceMirror = path.join(fixtureRoot, "upstream-vercel-ai");
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_vercel_ai.rs",
      "examples/template/ai-chat-status.tsx",
      "docs/packages/ai-vercel-ai.md",
      sourceGuardRunbookFixturePath,
      previewManifestMaterializerPath,
      "upstream:packages/ai/src/generate-text/stream-text.ts",
      "upstream:packages/ai/src/ui/default-chat-transport.ts",
    ];
    writeAiSdkHashFixture(fixtureRoot, sourceMirror, selectedFiles);

    const write = runHelper([
      "--root",
      fixtureRoot,
      "--source-mirror",
      sourceMirror,
      "--write",
    ]);
    assert.equal(write.status, 0, write.stdout + write.stderr);

    fs.appendFileSync(
      path.join(fixtureRoot, previewManifestMaterializerPath),
      "export const aiSdkMaterializerDrift = true;\n",
    );

    const helper = runHelper([
      "--root",
      fixtureRoot,
      "--source-mirror",
      sourceMirror,
      "--check",
      "--json",
    ]);
    assert.equal(helper.status, 1, helper.stdout + helper.stderr);
    const report = JSON.parse(helper.stdout);

    assert.equal(report.status, "stale");
    assert.equal(report.mirror_problem_count, 3);
    assert.deepEqual(report.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(report.missing_files, []);
    assert.deepEqual(report.stale_mirror_files, [previewManifestMaterializerPath]);
    assert.deepEqual(report.missing_mirror_files, []);

    const selectedSourceFiles = [
      "core/src/ecosystem/forge_vercel_ai.rs",
      "examples/template/ai-chat-status.tsx",
      "docs/packages/ai-vercel-ai.md",
      sourceGuardRunbookFixturePath,
      "upstream:packages/ai/src/generate-text/stream-text.ts",
      "upstream:packages/ai/src/ui/default-chat-transport.ts",
    ];
    for (const relativePath of selectedSourceFiles) {
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
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});
