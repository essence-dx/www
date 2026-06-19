const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath =
  "examples/template/markdown-mdx-content-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-content-react-markdown-source-guard.json";
const runbookFixturePath =
  "docs/packages/content-react-markdown.source-guard-runbook.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function runHelper(args) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd: root,
    encoding: "utf8",
  });
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("Markdown & MDX Content tracks source-guard fixture freshness through a package-owned helper", () => {
  const helperSource = read(helperPath);
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const packageDoc = read("docs/packages/content-react-markdown.md");

  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "Markdown & MDX Content"/);
  assert.match(helperSource, /PACKAGE_ID = "content\/react-markdown"/);
  assert.match(
    helperSource,
    /SOURCE_GUARD_RUNBOOK_FIXTURE =\s*"docs\/packages\/content-react-markdown\.source-guard-runbook\.json"/,
  );
  assert.doesNotMatch(helperSource, /fetch\(|localStorage|sessionStorage/);

  assert.equal(receipt.package_id, "content/react-markdown");
  assert.equal(receipt.official_package_name, "Markdown & MDX Content");
  assert.equal(
    receipt.upstream_package,
    "react-markdown; @mdx-js/mdx; @mdx-js/react",
  );
  assert.equal(
    receipt.upstream_version,
    "react-markdown@10.1.0; @mdx-js/mdx@3.1.1; @mdx-js/react@3.1.1",
  );
  assert.equal(
    receipt.source_mirror,
    "G:/WWW/inspirations/react-markdown; G:/WWW/inspirations/mdx",
  );
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.equal(receipt.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(receipt.runtime_execution, false);
  assert.equal(receipt.secret_access, false);

  const trackedFiles = Object.keys(receipt.file_hashes).sort();
  assert.deepEqual(trackedFiles, [
    "benchmarks/markdown-mdx-content-slice.test.ts",
    "docs/packages/content-react-markdown.md",
    runbookFixturePath,
    "dx-www/src/cli/studio_manifest.rs",
  ]);

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "content/react-markdown",
  );
  assert.ok(visibility, "Markdown & MDX Content package-status row is missing");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(visibility.official_package_name, "Markdown & MDX Content");

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Markdown & MDX Content receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperPath);
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/markdown-mdx-content-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/markdown-mdx-content-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/markdown-mdx-content-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, trackedFiles.length);
  assert.deepEqual([...hashRefresh.tracked_files].sort(), trackedFiles);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(
    hashRefresh.zed_visibility,
    "markdown-mdx-content:receipt-hash-refresh",
  );

  const runbookSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "source-guard-runbook-fixture",
  );
  assert.ok(runbookSurface, "source guard runbook surface is missing");
  assert.equal(runbookSurface.status, "present");
  assert.equal(runbookSurface.hash_algorithm, "sha256");
  assert.equal(
    runbookSurface.file_hashes[runbookFixturePath],
    receipt.file_hashes[runbookFixturePath],
  );

  assert.ok(
    status.zed_receipt_surfaces.includes(
      "markdown-mdx-content:receipt-hash-refresh",
    ),
    "Markdown & MDX Content helper is missing from Zed receipt surfaces",
  );

  const readModelStart = readModel.indexOf(
    "export const markdownMdxContentPackageVisibility = {",
  );
  const readModelEnd = readModel.indexOf(
    "} as const satisfies LaunchForgePackageLaneVisibility;",
    readModelStart,
  );
  assert.notEqual(readModelStart, -1, "Markdown & MDX Content read model is missing");
  assert.notEqual(readModelEnd, -1, "Markdown & MDX Content read model is not closed");
  const readModelBlock = readModel.slice(readModelStart, readModelEnd);

  assert.match(readModelBlock, /receiptHashRefresh/);
  assert.match(
    readModelBlock,
    /sourceGuardRunbookFixture:\s*"docs\/packages\/content-react-markdown\.source-guard-runbook\.json"/,
  );
  assert.match(readModelBlock, /markdown-mdx-content:receipt-hash-refresh/);
  for (const relativePath of trackedFiles) {
    const mirrors = [
      ...readModelBlock.matchAll(
        new RegExp(
          `"${escapeRegex(relativePath)}"\\s*:\\s*(?:\\r?\\n\\s*)?"([^"]+)"`,
          "g",
        ),
      ),
    ].map((match) => match[1]);
    assert.ok(
      mirrors.length > 0,
      `${relativePath} missing from Markdown & MDX Content read model`,
    );
    assert.ok(
      mirrors.every((hash) => hash === receipt.file_hashes[relativePath]),
      `${relativePath} has stale Markdown & MDX Content read model mirrors: ${mirrors.join(", ")}`,
    );
  }
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /markdown-mdx-content-receipt-hashes\.ts --check/);

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "content/react-markdown");
  assert.equal(helperReport.official_package_name, "Markdown & MDX Content");
  assert.equal(
    helperReport.upstream_package,
    "react-markdown; @mdx-js/mdx; @mdx-js/react",
  );
  assert.equal(helperReport.status, "current");
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(helperReport.tracked_file_count, trackedFiles.length);
  assert.equal(helperReport.stale_file_count, 0);
  assert.equal(helperReport.missing_file_count, 0);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
  assert.equal(
    helperReport.zed_visibility,
    "markdown-mdx-content:receipt-hash-refresh",
  );
});
