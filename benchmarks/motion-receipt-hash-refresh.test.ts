const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/motion-receipt-hashes.ts",
);
const motionSourceFiles = [
  "examples/template/motion/presets.ts",
  "examples/template/motion/provider.tsx",
  "examples/template/motion/controls.tsx",
  "examples/template/motion/frame.tsx",
  "examples/template/motion/layout.tsx",
  "examples/template/motion/lazy.tsx",
  "examples/template/motion/motion-values.tsx",
  "examples/template/motion/page-visibility.tsx",
  "examples/template/motion/presence.tsx",
  "examples/template/motion/reorder.tsx",
  "examples/template/motion/reveal.tsx",
  "examples/template/motion/scoped-animate.tsx",
  "examples/template/motion/scroll-progress.tsx",
  "examples/template/motion/will-change.tsx",
  "examples/template/motion/dashboard-workflow.ts",
  "examples/template/motion/metadata.ts",
  "examples/template/motion/README.md",
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

test("Motion & Animation receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Motion & Animation hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-motion-hashes-"));
  try {
    const legacySelectedFiles = [
      "examples/template/template-shell.tsx",
      "examples/template/motion-interaction-proof.tsx",
      "examples/dashboard/src/lib/motionDashboardWorkflow.ts",
      "examples/dashboard/src/components/MotionDashboardWorkflow.tsx",
      "docs/packages/motion-animation.source-guard-runbook.json",
    ];
    const selectedFiles = [...legacySelectedFiles, ...motionSourceFiles];
    const runbookFixturePath =
      "docs/packages/motion-animation.source-guard-runbook.json";
    const runbookFixturePaths = [
      {
        source_guard_id: "motion-animation-generated-starter-materialization",
        package_id: "animation/motion",
        fixture_path: runbookFixturePath,
        schema: "dx.forge.package.source_guard_runbook_fixture",
      },
    ];
    for (const selectedFile of selectedFiles) {
      if (selectedFile === runbookFixturePath) {
        continue;
      }
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const motionFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }
    writeJson(path.join(fixtureRoot, runbookFixturePath), {
      schema: "dx.forge.package.source_guard_runbook_fixture",
      fixture_path: runbookFixturePath,
      route: "/",
      package: {
        official_package_name: "Motion & Animation",
        package_id: "animation/motion",
        upstream_package: "motion",
        upstream_version: "12.38.0",
        source_mirror: "G:/WWW/inspirations/motion",
      },
      guard: {
        id: "motion-animation-generated-starter-materialization",
        fixture_path: runbookFixturePath,
      },
      runbook: {
        fixture_paths: runbookFixturePaths,
      },
    });

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.motion_animation.receipt",
      package_id: "animation/motion",
      official_package_name: "Motion & Animation",
      upstream_package: "motion",
      upstream_version: "12.38.0",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        legacySelectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        monitored_surfaces: [
          {
            surface_id: "motion-dashboard-workflow",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              legacySelectedFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      zed_receipt_surfaces: ["motion-animation:dashboard-workflow"],
      package_lane_visibility: [
        {
          official_package_name: "Motion & Animation",
          package_id: "animation/motion",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "motion-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                legacySelectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
          receipt_hash_refresh: {
            schema: "dx.forge.package.receipt_hash_refresh",
            status: "stale",
            helper_path: "examples/template/motion-receipt-hashes.ts",
            check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check",
            write_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --write",
            json_check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json",
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            tracked_file_count: legacySelectedFiles.length,
            stale_file_count: legacySelectedFiles.length,
            missing_file_count: 0,
            runtime_execution: false,
            secret_access: false,
            zed_visibility: "motion-animation:receipt-hash-refresh",
            runtime_limitations: [],
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
        "export const unrelatedPackageVisibility = {",
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        '        "examples/template/template-shell.tsx":',
        '          "do-not-touch-other-lane",',
        "      },",
        "    },",
        "  ],",
        "} as const;",
        "",
        "export const motionAnimationPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    status: "stale",',
        '    helperPath: "examples/template/motion-receipt-hashes.ts",',
        "    trackedFileCount: 0,",
        "    staleFileCount: 3,",
        "    missingFileCount: 0,",
        "  },",
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        ...legacySelectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "    {",
        "      fileHashes: {",
        ...legacySelectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "  ],",
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "animation/motion");
    assert.equal(staleReport.official_package_name, "Motion & Animation");
    assert.equal(staleReport.status, "missing");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(
      staleReport.zed_visibility,
      "motion-animation:receipt-hash-refresh",
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Motion & Animation receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.equal(freshReport.stale_file_count, 0);
    assert.equal(freshReport.missing_file_count, 0);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");

    assert.match(readModelText, /do-not-touch-other-lane/);

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      const refreshedReceiptSurface =
        refreshedReceipt.dx_check_visibility.monitored_surfaces.find(
          (surface) =>
            surface.file_hashes &&
            Object.prototype.hasOwnProperty.call(surface.file_hashes, selectedFile),
        );
      const refreshedPackageSurface =
        refreshedStatus.package_lane_visibility[0].selected_surfaces.find(
          (surface) =>
            surface.file_hashes &&
            Object.prototype.hasOwnProperty.call(surface.file_hashes, selectedFile),
        );
      assert.ok(refreshedReceiptSurface, `${selectedFile} missing from receipt surfaces`);
      assert.ok(
        refreshedPackageSurface,
        `${selectedFile} missing from package-status surfaces`,
      );
      assert.equal(refreshedReceiptSurface.file_hashes[selectedFile], refreshedHash);
      assert.equal(refreshedPackageSurface.file_hashes[selectedFile], refreshedHash);
      assert.equal(
        (readModelText.match(new RegExp(refreshedHash, "g")) ?? []).length,
        legacySelectedFiles.includes(selectedFile) ? 2 : 1,
      );
    }
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "motion-source-owned-template-helpers" &&
          motionSourceFiles.every((sourceFile) =>
            surface.files.includes(sourceFile),
          ),
      ),
      "package-status must expose source-owned Motion helper surface",
    );
    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path: "examples/template/motion-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        source_guard_runbook_fixture_paths: runbookFixturePaths,
        receipt_path: receiptPath,
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "motion-animation:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Motion & Animation receipt hash freshness only.",
          "ADAPTER-BOUNDARY: route choreography, reduced-motion policy, accessibility QA, animation budgets, and browser runtime proof stay app-owned.",
        ],
      },
    );
    assert.match(readModelText, /receiptHashRefresh/);
    assert.match(readModelText, /motion-animation:receipt-hash-refresh/);
    assert.match(readModelText, /sourceGuardRunbookFixturePaths/);
    assert.match(
      readModelText,
      /sourceGuardId: "motion-animation-generated-starter-materialization"/,
    );
    assert.match(readModelText, /fixturePath: "docs\/packages\/motion-animation\.source-guard-runbook\.json"/);
    assert.match(readModelText, /status: "current"/);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Motion & Animation receipt tracks source-guard runbook fixture freshness", () => {
  const runbookFixturePath =
    "docs/packages/motion-animation.source-guard-runbook.json";
  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.package_id, "animation/motion");
  assert.equal(helperReport.official_package_name, "Motion & Animation");
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.deepEqual(helperReport.source_guard_runbook_fixture_paths, [
    {
      source_guard_id: "motion-animation-generated-starter-materialization",
      package_id: "animation/motion",
      fixture_path: runbookFixturePath,
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
  ]);
  assert.equal(helperReport.tracked_file_count, 22);

  const receiptPath = path.join(
    root,
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
  );
  const receipt = JSON.parse(fs.readFileSync(receiptPath, "utf8"));
  assert.ok(
    Object.prototype.hasOwnProperty.call(receipt.file_hashes, runbookFixturePath),
    "receipt must hash the Motion & Animation source-guard runbook fixture",
  );
  assert.match(receipt.file_hashes[runbookFixturePath], /^[a-f0-9]{64}$/);

  const packageStatus = JSON.parse(
    fs.readFileSync(
      path.join(root, "examples/template/.dx/forge/package-status.json"),
      "utf8",
    ),
  );
  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "animation/motion",
  );
  assert.ok(visibility, "Motion & Animation package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.deepEqual(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture_paths,
    helperReport.source_guard_runbook_fixture_paths,
  );
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 22);
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "motion-source-owned-template-helpers" &&
        motionSourceFiles.every((sourceFile) => surface.files.includes(sourceFile)),
    ),
    "package-status must track the source-owned Motion helper surface",
  );

  const readModelText = fs.readFileSync(
    path.join(root, "examples/template/forge-package-status-read-model.ts"),
    "utf8",
  );
  assert.match(
    readModelText,
    /sourceGuardRunbookFixture: "docs\/packages\/motion-animation\.source-guard-runbook\.json"/,
  );
  assert.match(readModelText, /sourceGuardRunbookFixturePaths/);
  assert.match(
    readModelText,
    /sourceGuardId: "motion-animation-generated-starter-materialization"/,
  );
  assert.match(readModelText, /fixturePath: "docs\/packages\/motion-animation\.source-guard-runbook\.json"/);
  assert.match(readModelText, /trackedFileCount: 22/);
  assert.match(readModelText, /surfaceId: "motion-source-owned-template-helpers"/);
});

test("Motion & Animation docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/animation-motion.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/motion-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(
    packageDoc,
    /does not run browser animation runtime proof or route choreography/i,
  );
});
