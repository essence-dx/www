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
  "examples/template/ui-components-receipt-hashes.ts",
);
const runbookFixturePath = "docs/packages/ui-components.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const selectedDashboardSourcePaths = [
  "examples/template/components/ui/button.tsx",
  "examples/template/components/ui/slot.tsx",
  "examples/template/shadcn-dashboard-controls-contract.tsx",
  "examples/template/shadcn-dashboard-controls.tsx",
  runbookFixturePath,
];
const trackedHashPaths = [
  ...selectedDashboardSourcePaths,
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

test("UI Components receipt hash helper refreshes stale selected source hashes", () => {
  assert.ok(fs.existsSync(helperPath), "UI Components hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-ui-components-hashes-"));
  try {
    const selectedFile = "examples/template/components/ui/button.tsx";
    const selectedFilePath = path.join(fixtureRoot, selectedFile);
    fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
    fs.writeFileSync(
      selectedFilePath,
      "export function Button() { return 'fresh UI Components source'; }\n",
    );

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "shadcn/ui/button",
      official_package_name: "UI Components",
      upstream_package: "shadcn-ui",
      upstream_version: "0.0.1",
      source_mirrors: [
        "G:/WWW/inspirations/shadcn-ui",
        "G:/WWW/inspirations/radix-primitives",
      ],
      hash_algorithm: "sha256",
      file_hashes: [
        {
          path: selectedFile,
          sha256: "stale",
        },
      ],
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        file_hashes: {
          [selectedFile]: "stale",
        },
      },
    });

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /components\/ui\/button\.tsx/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const report = JSON.parse(fresh.stdout);
    assert.equal(report.official_package_name, "UI Components");
    assert.equal(report.package_id, "shadcn/ui/button");
    assert.equal(report.status, "current");
    assert.deepEqual(report.stale_files, []);
    assert.deepEqual(report.missing_files, []);
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.equal(report.zed_visibility, "ui-components:receipt-hash-refresh");

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    assert.match(refreshedReceipt.file_hashes[0].sha256, /^[a-f0-9]{64}$/);
    assert.equal(
      refreshedReceipt.file_hashes[0].sha256,
      refreshedReceipt.dx_check_visibility.file_hashes[selectedFile],
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("UI Components helper reports materializer-only stale paths without blaming selected UI source", () => {
  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-ui-components-materializer-stale-"),
  );
  try {
    const currentHashes = new Map(
      trackedHashPaths.map((relativePath) => [
        relativePath,
        copySourceIntoFixture(fixtureRoot, relativePath),
      ]),
    );
    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json";
    const receiptHashes = trackedHashPaths.map((relativePath) => ({
      path: relativePath,
      sha256:
        relativePath === previewManifestMaterializerPath
          ? "0".repeat(64)
          : currentHashes.get(relativePath),
    }));

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "shadcn/ui/button",
      official_package_name: "UI Components",
      upstream_package: "shadcn-ui",
      upstream_version: "0.0.1",
      source_mirrors: [
        "G:/WWW/inspirations/shadcn-ui",
        "G:/WWW/inspirations/radix-primitives",
      ],
      hash_algorithm: "sha256",
      file_hashes: receiptHashes,
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        file_hashes: Object.fromEntries(
          receiptHashes.map((entry) => [entry.path, entry.sha256]),
        ),
      },
    });

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.tracked_file_count, 6);
    assert.equal(staleReport.stale_file_count, 1);
    assert.equal(staleReport.missing_file_count, 0);
    assert.deepEqual(staleReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(staleReport.missing_files, []);

    const plain = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(plain.status, 0, plain.stdout + plain.stderr);
    const plainOutput = plain.stdout + plain.stderr;
    assert.match(
      plainOutput,
      new RegExp(escapeRegExp(previewManifestMaterializerPath)),
    );
    for (const selectedSourcePath of selectedDashboardSourcePaths) {
      assert.doesNotMatch(
        plainOutput,
        new RegExp(escapeRegExp(selectedSourcePath)),
      );
    }
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("UI Components docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/ui-components.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/ui-components-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run browser UI runtime proof/i);
});

test("UI Components receipt helper tracks the source-guard runbook fixture", () => {
  const receipt = JSON.parse(
    fs.readFileSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/2026-05-22-shadcn-dashboard-controls.json",
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

  const receiptHashPaths = receipt.file_hashes.map((entry) => entry.path);
  assert.ok(
    receiptHashPaths.includes(runbookFixturePath),
    "receipt must hash the UI Components source-guard runbook fixture",
  );
  assert.ok(
    receiptHashPaths.includes(previewManifestMaterializerPath),
    "receipt must hash the UI Components preview-manifest materializer integration",
  );
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(helperReport.tracked_file_count, receipt.file_hashes.length);
  assert.equal(helperReport.tracked_file_count, 6);
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "shadcn/ui/button",
  );
  assert.ok(visibility, "UI Components package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 6);
  assert.deepEqual(visibility.receipt_hash_refresh.stale_files, []);
  assert.deepEqual(visibility.receipt_hash_refresh.missing_files, []);
  assert.equal(
    visibility.source_hashes.files[previewManifestMaterializerPath],
    receipt.file_hashes.find((entry) => entry.path === previewManifestMaterializerPath)
      ?.sha256,
  );
  const materializerSurface = visibility.selected_surfaces.find(
    (entry) => entry.surface_id === "ui-components-preview-manifest-materializer",
  );
  assert.ok(materializerSurface, "UI Components materializer surface is missing");
  assert.equal(
    materializerSurface.file_hashes[previewManifestMaterializerPath],
    receipt.file_hashes.find((entry) => entry.path === previewManifestMaterializerPath)
      ?.sha256,
  );
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*\n\s*"docs\/packages\/ui-components\.source-guard-runbook\.json"/,
  );
  assert.match(readModel, /"tools\/launch\/materialize-www-template\.ts"/);
  assert.match(readModel, /trackedFileCount: 6/);
  assert.match(readModel, /staleFiles:\s*\[\]/);
  assert.match(readModel, /missingFiles:\s*\[\]/);
});
