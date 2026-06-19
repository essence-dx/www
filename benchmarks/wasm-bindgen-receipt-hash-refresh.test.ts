const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const helperRelativePath =
  "examples/template/webassembly-bridge-receipt-hashes.ts";
const helperPath = path.join(repoRoot, helperRelativePath);
const packageStatusRelativePath =
  "examples/template/.dx/forge/package-status.json";
const readModelRelativePath =
  "examples/template/forge-package-status-read-model.ts";
const docsRelativePath = "docs/packages/wasm-bindgen.md";
const receiptRelativePath =
  "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json";
const generatedReceiptPath =
  ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json";
const sourceGuardRunbookFixture =
  "docs/packages/wasm-bindgen.source-guard-runbook.json";
const previewManifestMaterializer =
  "tools/launch/materialize-www-template.ts";
const lowerDxCheckSource =
  "core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs";
const checkPanelSource = "core/src/ecosystem/dx_check_receipt.rs";
const zedVisibility = "webassembly-bridge:receipt-hash-refresh";

const dxCheckMetrics = [
  "webassembly_bridge_receipt_hash_refresh_current",
  "webassembly_bridge_receipt_hash_refresh_stale",
  "webassembly_bridge_receipt_hash_refresh_missing",
];

const wasmRuntimeHelperFiles = [
  "examples/template/wasm/bindgen/loader.ts",
  "examples/template/wasm/bindgen/react.tsx",
  "examples/template/wasm/bindgen/readiness.ts",
  "examples/template/wasm/bindgen/metadata.ts",
  "examples/template/wasm/bindgen/README.md",
];

const selectedFiles = [
  "examples/template/wasm-interop-status.tsx",
  "examples/template/template-shell.tsx",
  "tools/launch/runtime-template/pages/index.html",
  "tools/launch/runtime-template/assets/launch-runtime.ts",
  "examples/template/dx-studio-edit-contract.ts",
  "tools/launch/materialize-www-template.ts",
  "examples/dashboard/src/components/WasmBindgenWorkflow.tsx",
  "examples/dashboard/src/lib/wasmBindgenDashboard.ts",
  "core/src/ecosystem/forge_wasm_bindgen.rs",
  sourceGuardRunbookFixture,
  lowerDxCheckSource,
  checkPanelSource,
  ...wasmRuntimeHelperFiles,
];

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), "utf8"));
}

function readText(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function writeJson(root, relativePath, value) {
  const targetPath = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(targetPath), { recursive: true });
  fs.writeFileSync(`${targetPath}.tmp`, `${JSON.stringify(value, null, 2)}\n`);
  fs.renameSync(`${targetPath}.tmp`, targetPath);
}

function writeText(root, relativePath, value) {
  const targetPath = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(targetPath), { recursive: true });
  fs.writeFileSync(targetPath, value);
}

function sha256(root, relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(root, relativePath)))
    .digest("hex");
}

function runHelper(args, options = {}) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd: options.cwd || repoRoot,
    encoding: "utf8",
  });
}

function parseJsonOutput(result, expectedStatus = 0) {
  assert.equal(result.status, expectedStatus, result.stderr || result.stdout);
  return JSON.parse(result.stdout);
}

function createTempRoot() {
  const tempRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-wasm-bindgen-receipt-hash-"),
  );

  for (const relativePath of selectedFiles) {
    writeText(tempRoot, relativePath, `fixture:${relativePath}\n`);
  }

  const staleHashes = Object.fromEntries(
    selectedFiles.map((relativePath) => [relativePath, "0".repeat(64)]),
  );

  const receipt = {
    schema: "dx.forge.receipt",
    official_package_name: "WebAssembly Bridge",
    package_id: "wasm/bindgen",
    upstream_package: "wasm-bindgen",
    upstream_version: "0.2.121",
    source_mirror: "G:/WWW/inspirations/wasm-bindgen",
    surface: "dashboard-workflow",
    generated_at: "2026-05-22T00:00:00.000Z",
    runtime_execution: false,
    hash_algorithm: "sha256",
    files: selectedFiles,
    file_hashes: staleHashes,
    source_guard_runbook_fixture: sourceGuardRunbookFixture,
    source_guard_runbook_surface: {
      surface_id: "webassembly-bridge-source-guard-runbook",
      official_package_name: "WebAssembly Bridge",
      surface_type: "source_guard_runbook_fixture",
      status: "present",
      receipt_path: generatedReceiptPath,
      files: [sourceGuardRunbookFixture],
      source_markers: ["source_guard_runbook_index"],
      hash_algorithm: "sha256",
      file_hashes: {
        [sourceGuardRunbookFixture]: "0".repeat(64),
      },
      runtime_proof: false,
    },
    preview_manifest_materializer: previewManifestMaterializer,
  };

  const packageStatus = {
    schema: "dx.forge.package_status",
    generated_at: "2026-05-22T00:00:00.000Z",
    dx_check_metrics: [],
    zed_receipt_surfaces: [],
    package_lane_visibility: [
      {
        official_package_name: "WebAssembly Bridge",
        package_id: "wasm/bindgen",
        upstream_package: "wasm-bindgen",
        upstream_version: "0.2.121",
        source_mirror: "G:/WWW/inspirations/wasm-bindgen",
        status: "present",
        receipt_status: "present",
        package_receipt_path: generatedReceiptPath,
        dx_check_metrics: [],
        selected_surfaces: [
          {
            surface_id: "dashboard-workflow",
            hash_algorithm: "sha256",
            files: selectedFiles,
            file_hashes: staleHashes,
          },
        ],
      },
    ],
  };

  const readModel = `export const webAssemblyBridgeFileHashes = ${JSON.stringify(
    staleHashes,
    null,
    2,
  )} as const;

export const webAssemblyBridgePackageVisibility = {
  officialName: "WebAssembly Bridge",
  packageId: "wasm/bindgen",
  upstreamPackage: "wasm-bindgen",
  upstreamVersion: "0.2.121",
  sourceMirror: "G:/WWW/inspirations/wasm-bindgen",
  packageReceiptPath: "${generatedReceiptPath}",
  selectedSurfaces: [],
  dxCheckMetrics: [],
  zedReceiptSurfaces: [],
} as const;
`;

  writeJson(tempRoot, receiptRelativePath, receipt);
  writeJson(tempRoot, packageStatusRelativePath, packageStatus);
  writeText(tempRoot, readModelRelativePath, readModel);

  return tempRoot;
}

test("WebAssembly Bridge exposes a source-owned receipt hash refresh surface", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "expected WebAssembly Bridge receipt hash helper to exist",
  );

  const helperSource = fs.readFileSync(helperPath, "utf8");
  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "WebAssembly Bridge"/);
  assert.match(helperSource, /PACKAGE_ID = "wasm\/bindgen"/);
  assert.match(helperSource, /UPSTREAM_PACKAGE = "wasm-bindgen"/);
  assert.match(helperSource, /SOURCE_MIRROR = "G:\/WWW\/inspirations\/wasm-bindgen"/);
  assert.match(helperSource, /SOURCE_GUARD_RUNBOOK_FIXTURE/);
  assert.match(helperSource, /PREVIEW_MANIFEST_MATERIALIZER/);
  assert.match(helperSource, /LOWER_DX_CHECK_SOURCE/);
  assert.match(helperSource, /CHECK_PANEL_SOURCE/);
  assert.match(helperSource, /WEBASSEMBLY_BRIDGE_SOURCE_FILES/);
  assert.doesNotMatch(helperSource, /\bwasm-pack\b/);
  assert.doesNotMatch(helperSource, /\bcargo\s+build\b/);
  assert.doesNotMatch(helperSource, /\bwasm-bindgen\s+.*--target\b/);
  assert.doesNotMatch(helperSource, /WebAssembly\.instantiate/);
  assert.doesNotMatch(helperSource, /\bfetch\(/);
  assert.doesNotMatch(helperSource, /\b(?:npm install|pnpm install|yarn add)\b/);

  const receipt = readJson(receiptRelativePath);
  assert.equal(receipt.official_package_name, "WebAssembly Bridge");
  assert.equal(receipt.package_id, "wasm/bindgen");
  assert.equal(receipt.upstream_package, "wasm-bindgen");
  assert.equal(receipt.source_guard_runbook_fixture, sourceGuardRunbookFixture);

  const packageStatus = readJson(packageStatusRelativePath);
  const wasmRow = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "wasm/bindgen",
  );
  assert.ok(wasmRow, "expected WebAssembly Bridge package visibility row");
  assert.ok(
    wasmRow.receipt_hash_refresh,
    "expected WebAssembly Bridge receipt_hash_refresh mirror",
  );

  const refresh = wasmRow.receipt_hash_refresh;
  assert.equal(refresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(refresh.status, "current");
  assert.equal(refresh.helper_path, helperRelativePath);
  assert.equal(refresh.check_command, "node tools/launch/run-template-receipt-helper.js examples/template/webassembly-bridge-receipt-hashes.ts --check");
  assert.equal(refresh.write_command, "node tools/launch/run-template-receipt-helper.js examples/template/webassembly-bridge-receipt-hashes.ts --write");
  assert.equal(refresh.json_check_command, "node tools/launch/run-template-receipt-helper.js examples/template/webassembly-bridge-receipt-hashes.ts --check --json");
  assert.equal(refresh.source_guard_runbook_fixture, sourceGuardRunbookFixture);
  assert.equal(refresh.preview_manifest_materializer, previewManifestMaterializer);
  assert.equal(refresh.lower_dx_check_source, lowerDxCheckSource);
  assert.equal(refresh.check_panel_source, checkPanelSource);
  assert.equal(refresh.receipt_path, generatedReceiptPath);
  assert.equal(refresh.hash_algorithm, "sha256");
  assert.equal(refresh.tracked_file_count, selectedFiles.length);
  assert.deepEqual(refresh.tracked_files, selectedFiles);
  assert.deepEqual(refresh.current_files, selectedFiles);
  assert.deepEqual(refresh.stale_files, []);
  assert.deepEqual(refresh.missing_files, []);
  assert.deepEqual(refresh.stale_mirror_files, []);
  assert.deepEqual(refresh.missing_mirror_files, []);
  assert.equal(refresh.mirror_problem_count, 0);
  assert.equal(refresh.runtime_execution, false);
  assert.equal(refresh.secret_access, false);
  assert.equal(refresh.runs_package_install, false);
  assert.equal(refresh.zed_visibility, zedVisibility);

  for (const metric of dxCheckMetrics) {
    assert.ok(
      packageStatus.dx_check_metrics.includes(metric),
      `expected package status root metric ${metric}`,
    );
    assert.ok(
      wasmRow.dx_check_metrics.includes(metric),
      `expected WebAssembly Bridge row metric ${metric}`,
    );
  }
  assert.ok(packageStatus.zed_receipt_surfaces.includes(zedVisibility));

  const readModelSource = readText(readModelRelativePath);
  assert.match(readModelSource, /receiptHashRefresh:/);
  assert.match(readModelSource, new RegExp(helperRelativePath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(readModelSource, new RegExp(sourceGuardRunbookFixture.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(readModelSource, new RegExp(previewManifestMaterializer.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(readModelSource, new RegExp(lowerDxCheckSource.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(readModelSource, new RegExp(checkPanelSource.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(readModelSource, new RegExp(zedVisibility.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  for (const metric of dxCheckMetrics) {
    assert.match(readModelSource, new RegExp(metric));
  }

  const docs = readText(docsRelativePath);
  assert.match(docs, /webassembly-bridge-receipt-hashes\.ts/);
  assert.match(docs, /receipt hash refresh/i);

  const report = parseJsonOutput(runHelper(["--check", "--json"]));
  assert.equal(report.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(report.official_package_name, "WebAssembly Bridge");
  assert.equal(report.package_id, "wasm/bindgen");
  assert.equal(report.upstream_package, "wasm-bindgen");
  assert.equal(report.upstream_version, "0.2.121");
  assert.equal(report.source_mirror, "G:/WWW/inspirations/wasm-bindgen");
  assert.equal(report.status, "current");
  assert.equal(report.helper_path, helperRelativePath);
  assert.equal(report.lower_dx_check_source, lowerDxCheckSource);
  assert.equal(report.check_panel_source, checkPanelSource);
  assert.equal(report.receipt_path, generatedReceiptPath);
  assert.deepEqual(report.tracked_files, selectedFiles);
  assert.deepEqual(report.current_files, selectedFiles);
  assert.deepEqual(report.stale_files, []);
  assert.deepEqual(report.missing_files, []);
  assert.equal(report.mirror_problem_count, 0);
});

test("WebAssembly Bridge helper refreshes receipts and reports stale or missing files", () => {
  const tempRoot = createTempRoot();
  try {
    const writeResult = runHelper(["--write", "--root", tempRoot]);
    assert.equal(writeResult.status, 0, writeResult.stderr || writeResult.stdout);

    const receipt = JSON.parse(
      fs.readFileSync(path.join(tempRoot, receiptRelativePath), "utf8"),
    );
    for (const relativePath of selectedFiles) {
      assert.equal(receipt.file_hashes[relativePath], sha256(tempRoot, relativePath));
    }
    assert.equal(
      receipt.source_guard_runbook_surface.file_hashes[
        sourceGuardRunbookFixture
      ],
      sha256(tempRoot, sourceGuardRunbookFixture),
      "helper should refresh the receipt's nested source-guard surface hash",
    );

    const packageStatus = JSON.parse(
      fs.readFileSync(path.join(tempRoot, packageStatusRelativePath), "utf8"),
    );
    const wasmRow = packageStatus.package_lane_visibility.find(
      (entry) => entry.package_id === "wasm/bindgen",
    );
    assert.equal(wasmRow.receipt_hash_refresh.status, "current");
    assert.equal(wasmRow.receipt_hash_refresh.lower_dx_check_source, lowerDxCheckSource);
    assert.equal(wasmRow.receipt_hash_refresh.check_panel_source, checkPanelSource);
    assert.deepEqual(wasmRow.receipt_hash_refresh.current_files, selectedFiles);
    assert.deepEqual(wasmRow.receipt_hash_refresh.stale_files, []);
    assert.deepEqual(wasmRow.receipt_hash_refresh.missing_files, []);
    assert.ok(packageStatus.zed_receipt_surfaces.includes(zedVisibility));
    const runtimeHelperSurface = wasmRow.selected_surfaces.find(
      (surface) => surface.surface_id === "webassembly-bridge-source-owned-runtime-helpers",
    );
    assert.ok(runtimeHelperSurface, "expected source-owned runtime helpers surface");
    assert.deepEqual(runtimeHelperSurface.files, wasmRuntimeHelperFiles);
    for (const relativePath of wasmRuntimeHelperFiles) {
      assert.equal(
        runtimeHelperSurface.file_hashes[relativePath],
        sha256(tempRoot, relativePath),
      );
    }

    const lowerDxCheckSurface = wasmRow.selected_surfaces.find(
      (surface) => surface.surface_id === "webassembly-bridge-lower-dx-check-source",
    );
    assert.ok(lowerDxCheckSurface, "expected lower dx-check selected surface");
    assert.deepEqual(lowerDxCheckSurface.files, [lowerDxCheckSource]);
    assert.equal(
      lowerDxCheckSurface.file_hashes[lowerDxCheckSource],
      sha256(tempRoot, lowerDxCheckSource),
    );
    assert.ok(
      lowerDxCheckSurface.source_markers.includes(
        "webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays",
      ),
    );

    const checkPanelSurface = wasmRow.selected_surfaces.find(
      (surface) => surface.surface_id === "webassembly-bridge-check-panel-source",
    );
    assert.ok(checkPanelSurface, "expected check-panel selected surface");
    assert.deepEqual(checkPanelSurface.files, [checkPanelSource]);
    assert.equal(
      checkPanelSurface.file_hashes[checkPanelSource],
      sha256(tempRoot, checkPanelSource),
    );
    assert.ok(
      checkPanelSurface.source_markers.includes(
        "dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row",
      ),
    );

    const readModel = fs.readFileSync(
      path.join(tempRoot, readModelRelativePath),
      "utf8",
    );
    assert.match(readModel, /receiptHashRefresh:/);
    assert.match(readModel, /webassembly-bridge:receipt-hash-refresh/);
    assert.match(readModel, /lowerDxCheckSource:\s*"core\/src\/ecosystem\/project_check\/wasm_bindgen_dx_check\.rs"/);
    assert.match(readModel, /checkPanelSource:\s*"core\/src\/ecosystem\/dx_check_receipt\.rs"/);
    assert.match(readModel, /surfaceId: "webassembly-bridge-lower-dx-check-source"/);
    assert.match(readModel, /surfaceId: "webassembly-bridge-check-panel-source"/);

    writeText(tempRoot, previewManifestMaterializer, "changed materializer\n");
    const staleReport = parseJsonOutput(
      runHelper(["--check", "--json", "--root", tempRoot]),
      1,
    );
    assert.equal(staleReport.status, "stale");
    assert.deepEqual(staleReport.stale_files, [previewManifestMaterializer]);
    assert.deepEqual(staleReport.missing_files, []);

    fs.rmSync(path.join(tempRoot, sourceGuardRunbookFixture));
    const missingReport = parseJsonOutput(
      runHelper(["--check", "--json", "--root", tempRoot]),
      1,
    );
    assert.equal(missingReport.status, "missing");
    assert.ok(missingReport.missing_files.includes(sourceGuardRunbookFixture));
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("WebAssembly Bridge helper isolates check-panel-only stale source drift", () => {
  const tempRoot = createTempRoot();
  try {
    const writeResult = runHelper(["--write", "--root", tempRoot]);
    assert.equal(writeResult.status, 0, writeResult.stderr || writeResult.stdout);

    writeText(tempRoot, checkPanelSource, "changed check-panel source only\n");
    const staleReport = parseJsonOutput(
      runHelper(["--check", "--json", "--root", tempRoot]),
      1,
    );

    assert.equal(staleReport.status, "stale");
    assert.deepEqual(staleReport.stale_files, [checkPanelSource]);
    assert.deepEqual(staleReport.missing_files, []);
    assert.deepEqual(staleReport.stale_surface_ids, [
      "webassembly-bridge-check-panel-source",
    ]);
    assert.deepEqual(staleReport.stale_surface_types, ["check_panel_source"]);
    assert.deepEqual(staleReport.missing_surface_ids, []);
    assert.deepEqual(staleReport.missing_surface_types, []);
    assert.ok(!staleReport.stale_files.includes(lowerDxCheckSource));
    assert.ok(
      !staleReport.stale_surface_ids.includes(
        "webassembly-bridge-lower-dx-check-source",
      ),
    );
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.runs_package_install, false);

    const runbook = readJson(sourceGuardRunbookFixture);
    const staleFixture =
      runbook.receipt_hash_refresh_stale_only_fixtures.find(
        (entry) => entry.id === "webassembly-bridge-check-panel-source-stale-only",
      );
    assert.ok(staleFixture, "expected check-panel stale-only fixture metadata");
    assert.equal(staleFixture.mutates_only, checkPanelSource);
    assert.deepEqual(staleFixture.expected_stale_files, [checkPanelSource]);
    assert.deepEqual(staleFixture.expected_stale_surface_ids, [
      "webassembly-bridge-check-panel-source",
    ]);
    assert.deepEqual(staleFixture.expected_stale_surface_types, [
      "check_panel_source",
    ]);
    assert.equal(staleFixture.runtime_proof, false);
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});
