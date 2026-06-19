const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Forge safety/archive rollback coverage is a Studio source guard", () => {
  const fixture = readJson(
    "docs/packages/forge-safety-archive.source-guard-runbook.json",
  );
  const studio = read("dx-www/src/cli/studio_manifest.rs");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const runbookReadModel = read(
    "examples/template/forge-safety-archive-runbook.ts",
  );
  const safetyArchivePanel = read(
    "examples/template/forge-safety-archive-panel.tsx",
  );
  const packageLockGuard = read("benchmarks/www-forge-package-lock.test.ts");

  assert.equal(
    fixture.schema,
    "dx.forge.safety_archive.source_guard_runbook_fixture",
  );
  assert.equal(fixture.guard.id, "forge-safety-archive-rollback-coverage");
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts",
  );
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.runtime_proof, false);
  assert.equal(fixture.surface.id, "forge-safety-archive-status");
  assert.equal(
    fixture.preview_manifest.fixture_path,
    "docs/packages/forge-safety-archive.source-guard-runbook.json",
  );

  assert.match(
    studio,
    /studio_source_guard_with_fixture\(\s*"forge-safety-archive-rollback-coverage"/,
  );
  assert.match(
    studio,
    /"fixture_path": "docs\/packages\/forge-safety-archive\.source-guard-runbook\.json"/,
  );
  assert.match(
    studio,
    /source_guard_contract_with_fixture\(\s*"forge-safety-archive-rollback-coverage"[\s\S]*?docs\/packages\/forge-safety-archive\.source-guard-runbook\.json/,
  );
  assert.ok(
    studio.includes(
      '"dx run --test .\\\\benchmarks\\\\www-forge-package-lock.test.ts"',
    ),
    "Studio runbook commands should expose the rollback coverage command",
  );
  assert.ok(
    studio.includes('"docs/packages/forge-safety-archive.source-guard-runbook.json"'),
    "Studio runbook commands should expose the safety/archive fixture path",
  );

  assert.match(
    materializer,
    /const FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE = \{/,
  );
  assert.match(
    materializer,
    /sourceGuardRunbookFixtures: \[\s*FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE,/,
  );
  assert.match(
    materializer,
    /sourceGuardRunbookFixtures: \[\s*FORGE_SAFETY_ARCHIVE_SOURCE_GUARD_RUNBOOK_FIXTURE\.fixture,/,
  );
  assert.match(
    materializer,
    /command: "dx run --test \.\\\\benchmarks\\\\www-forge-package-lock\.test\.ts"/,
  );
  assert.match(
    materializer,
    /nodeModulesRequired: false/,
  );

  assert.match(runtimeLaunch, /data-dx-component="forge-safety-archive-status"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-state="covered"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-safe-delete="true"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-rollback-coverage="100"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-receipt-count="3"/);
  assert.match(
    runtimeLaunch,
    /data-dx-safety-archive-runbook-source="public\/preview-manifest\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-safety-archive-runbook-fixture="docs\/packages\/forge-safety-archive\.source-guard-runbook\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-safety-archive-runbook-command="dx run --test \.\\benchmarks\\www-forge-package-lock\.test\.ts"/,
  );

  assert.match(editContract, /id: "forge-safety-archive-status"/);
  assert.match(editContract, /receiptPath: "\.dx\/forge\/receipts\/safety"/);
  assert.match(editContract, /data-dx-safety-archive-runbook-command/);

  assert.match(routeContract, /components\/template-app\/forge-safety-archive-runbook\.ts/);
  assert.match(
    runbookReadModel,
    /readForgeSafetyArchiveRunbookFromPreviewManifest/,
  );
  assert.match(runbookReadModel, /previewManifestSource: "public\/preview-manifest\.json"/);
  assert.match(runbookReadModel, /sourceGuardRunbookFixtures: \[forgeSafetyArchivePreviewManifestFixture\]/);
  assert.match(
    runbookReadModel,
    /command: fixture\?\.command \?\? forgeSafetyArchiveCommand/,
  );
  assert.match(safetyArchivePanel, /forgeSafetyArchiveRunbookReadModel/);
  assert.match(
    safetyArchivePanel,
    /data-dx-safety-archive-runbook-command=\{panelRunbook\.command\}/,
  );

  assert.match(packageLockGuard, /archiveReceipt\.schema/);
  assert.match(packageLockGuard, /forge\.package_safety_archive_receipt/);
  assert.match(packageLockGuard, /rollback_covered_package_count/);
});
