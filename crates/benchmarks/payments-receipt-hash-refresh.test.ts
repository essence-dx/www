const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/payments-receipt-hashes.ts",
);
const runbookFixturePath = "docs/packages/payments.source-guard-runbook.json";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";

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

test("Payments receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Payments hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-payments-hashes-"));
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_stripe_js.rs",
      "examples/template/payments-status.tsx",
      "docs/packages/payments-stripe-js.md",
      runbookFixturePath,
      studioManifestSourcePath,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const paymentsFixture = ${JSON.stringify(selectedFile)};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.receipt",
      package_id: "payments/stripe-js",
      package_name: "Payments",
      upstream_package: "@stripe/stripe-js",
      upstream_version: "9.6.0",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        monitored_surfaces: [
          {
            id: "payments-launch-billing-checkout-workflow",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Payments",
          package_id: "payments/stripe-js",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "payments-launch-billing-checkout-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
          receipt_hash_refresh: {
            schema: "dx.forge.package.receipt_hash_refresh",
            status: "stale",
            helper_path: "examples/template/payments-receipt-hashes.ts",
            check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check",
            write_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --write",
            json_check_command:
              "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check --json",
            receipt_path: receiptPath,
            hash_algorithm: "sha256",
            tracked_file_count: selectedFiles.length,
            stale_file_count: selectedFiles.length,
            missing_file_count: 0,
            runtime_execution: false,
            secret_access: false,
            zed_visibility: "payments:receipt-hash-refresh",
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
        "const paymentsBillingWorkflowFileHashes = {",
        ...selectedFiles.flatMap((selectedFile) => [
          `  "${selectedFile}":`,
          '    "stale",',
        ]),
        "} as const;",
        "",
        "export const paymentsPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    status: "stale",',
        '    helperPath: "examples/template/payments-receipt-hashes.ts",',
        "    trackedFileCount: 0,",
        `    staleFileCount: ${selectedFiles.length},`,
        "    missingFileCount: 0,",
        "  },",
        "} as const;",
        "",
        "export const backendPlatformClientPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    helperPath: "examples/template/backend-platform-client-receipt-hashes.ts",',
        '    zedVisibility: "backend-platform-client:receipt-hash-refresh",',
        "  },",
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "payments/stripe-js");
    assert.equal(staleReport.official_package_name, "Payments");
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(staleReport.zed_visibility, "payments:receipt-hash-refresh");

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Payments receipt hashes updated/);

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

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.equal(
        refreshedReceipt.dx_check_visibility.monitored_surfaces[0].file_hashes[
          selectedFile
        ],
        refreshedHash,
      );
      assert.equal(
        refreshedStatus.package_lane_visibility[0].selected_surfaces[0]
          .file_hashes[selectedFile],
        refreshedHash,
      );
      assert.match(readModelText, new RegExp(refreshedHash));
    }
    assert.deepEqual(refreshedStatus.package_lane_visibility[0].receipt_hash_refresh, {
      schema: "dx.forge.package.receipt_hash_refresh",
      status: "current",
      helper_path: "examples/template/payments-receipt-hashes.ts",
      check_command:
        "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check",
      write_command:
        "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --write",
      json_check_command:
        "node tools/launch/run-template-receipt-helper.js examples/template/payments-receipt-hashes.ts --check --json",
      source_guard_runbook_fixture: runbookFixturePath,
      studio_manifest_source: studioManifestSourcePath,
      receipt_path: receiptPath,
      hash_algorithm: "sha256",
      tracked_file_count: selectedFiles.length,
      stale_file_count: 0,
      missing_file_count: 0,
      runtime_execution: false,
      secret_access: false,
      zed_visibility: "payments:receipt-hash-refresh",
      runtime_limitations: [
        "SOURCE-ONLY: this helper checks local Payments receipt hash freshness only.",
        "ADAPTER-BOUNDARY: Stripe credentials, Price IDs, Checkout redirects, webhooks, fulfillment, and compliance stay app-owned.",
      ],
    });
    assert.match(readModelText, /receiptHashRefresh/);
    assert.match(readModelText, /payments:receipt-hash-refresh/);
    assert.match(
      readModelText,
      /backend-platform-client-receipt-hashes\.ts/,
      "Payments helper must not rewrite the next package lane's hash refresh block",
    );
    assert.match(readModelText, /status: "current"/);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Payments docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/payments-stripe-js.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/payments-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run live Stripe Checkout or read Stripe secrets/i);
});

test("Payments receipt helper tracks the source-guard runbook fixture", () => {
  const receipt = JSON.parse(
    fs.readFileSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
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
  assert.ok(report.stdout.trim(), report.stderr);
  const helperReport = JSON.parse(report.stdout);

  assert.ok(
    Object.prototype.hasOwnProperty.call(receipt.file_hashes, runbookFixturePath),
    "receipt must hash the Payments source-guard runbook fixture",
  );
  assert.ok(
    Object.prototype.hasOwnProperty.call(
      receipt.file_hashes,
      studioManifestSourcePath,
    ),
    "receipt must hash the Payments Studio manifest source",
  );
  assert.ok(
    Object.prototype.hasOwnProperty.call(
      receipt.file_hashes,
      "examples/template/app/api/payments/stripe-js/readiness/route.ts",
    ),
    "receipt must hash the Payments readiness route",
  );
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(helperReport.studio_manifest_source, studioManifestSourcePath);
  assert.equal(helperReport.tracked_file_count, Object.keys(receipt.file_hashes).length);
  assert.equal(helperReport.tracked_file_count, 7);

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "payments/stripe-js",
  );
  assert.ok(visibility, "Payments package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.studio_manifest_source,
    studioManifestSourcePath,
  );
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 7);
  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/payments\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /studioManifestSource:\s*"dx-www\/src\/cli\/studio_manifest\.rs"/,
  );
  assert.match(readModel, /trackedFileCount: 7/);
});
