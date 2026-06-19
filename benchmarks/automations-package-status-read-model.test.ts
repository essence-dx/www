const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json";
const sourceGuardRunbookPath =
  "docs/packages/automation-connectors.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const studioManifestSourcePath = "dx-www/src/cli/studio_manifest.rs";
const lowerDxCheckSourcePath =
  "core/src/ecosystem/project_check/automation_connectors_dx_check.rs";
const checkPanelSourcePath = "core/src/ecosystem/dx_check_receipt.rs";
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Automation Connectors visibility is consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const sourceGuardRunbook = readJson(sourceGuardRunbookPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/automations-n8n.md");
  const receiptTrackedFileCount = Object.keys(receipt.file_hashes).length;

  const automationsVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "automations/n8n",
  );

  assert.ok(
    automationsVisibility,
    "Automation Connectors visibility row is missing",
  );
  assert.equal(automationsVisibility.official_package_name, "Automation Connectors");
  assert.equal(automationsVisibility.upstream_package, "n8n-nodes-base");
  assert.equal(automationsVisibility.upstream_version, "2.22.0");
  assert.equal(automationsVisibility.source_mirror, "G:/WWW/inspirations/n8n/packages/nodes-base");
  assert.equal(automationsVisibility.status, "present");
  assert.equal(automationsVisibility.receipt_status, "present");
  assert.equal(automationsVisibility.package_receipt_path, receiptPath);
  assert.deepEqual(automationsVisibility.status_vocabulary, statusVocabulary);
  assert.equal(
    automationsVisibility.receipt_hash_refresh.source_guard_runbook_fixture,
    sourceGuardRunbookPath,
  );
  assert.equal(
    automationsVisibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(
    automationsVisibility.receipt_hash_refresh.studio_manifest_source,
    studioManifestSourcePath,
  );
  assert.equal(
    automationsVisibility.receipt_hash_refresh.lower_dx_check_source,
    lowerDxCheckSourcePath,
  );
  assert.equal(
    automationsVisibility.receipt_hash_refresh.check_panel_source,
    checkPanelSourcePath,
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.tracked_files.includes(
      previewManifestMaterializerPath,
    ),
    "Automation Connectors package-status should expose materializer hash tracking",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.tracked_files.includes(
      studioManifestSourcePath,
    ),
    "Automation Connectors package-status should expose Studio manifest hash tracking",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.tracked_files.includes(
      lowerDxCheckSourcePath,
    ),
    "Automation Connectors package-status should expose lower dx-check hash tracking",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.tracked_files.includes(
      checkPanelSourcePath,
    ),
    "Automation Connectors package-status should expose check-panel hash tracking",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.current_files.includes(
      previewManifestMaterializerPath,
    ),
    "Automation Connectors package-status should attribute the current materializer file",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.current_files.includes(
      studioManifestSourcePath,
    ),
    "Automation Connectors package-status should attribute the current Studio manifest file",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.current_files.includes(
      lowerDxCheckSourcePath,
    ),
    "Automation Connectors package-status should attribute the current lower dx-check file",
  );
  assert.ok(
    automationsVisibility.receipt_hash_refresh.current_files.includes(
      checkPanelSourcePath,
    ),
    "Automation Connectors package-status should attribute the current check-panel file",
  );
  assert.deepEqual(automationsVisibility.receipt_hash_refresh.stale_files, []);
  assert.deepEqual(automationsVisibility.receipt_hash_refresh.missing_files, []);
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-connectors-source-guard-runbook" &&
        surface.files.includes(sourceGuardRunbookPath) &&
        surface.source_markers.includes("automation-connectors-package-lane-panel") &&
        surface.source_markers.includes("automation-connectors:receipt-hash-refresh"),
    ),
    "Automation Connectors source-guard runbook fixture surface is missing",
  );
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id ===
          "automation-connectors-preview-manifest-materializer" &&
        surface.files.includes(previewManifestMaterializerPath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes?.[previewManifestMaterializerPath] &&
        surface.source_markers.includes(
          "AUTOMATION_CONNECTORS_SOURCE_GUARD_RUNBOOK_FIXTURE",
        ) &&
        surface.source_markers.includes("sourceGuardRunbookFixtures"),
    ),
    "Automation Connectors preview-manifest materializer surface is missing",
  );
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-connectors-studio-manifest-source" &&
        surface.files.includes(studioManifestSourcePath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes?.[studioManifestSourcePath] &&
        surface.source_markers.includes(
          "automation-connectors-package-lane-panel",
        ) &&
        surface.source_markers.includes("source_guard_runbook_index"),
    ),
    "Automation Connectors Studio manifest source surface is missing",
  );
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-connectors-lower-dx-check-source" &&
        surface.files.includes(lowerDxCheckSourcePath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes?.[lowerDxCheckSourcePath] &&
        surface.source_markers.includes(
          "forge_automation_connectors_package_metrics",
        ) &&
        surface.source_markers.includes(
          "automation_connectors_receipt_hash_refresh_current",
        ),
    ),
    "Automation Connectors lower dx-check source surface is missing",
  );
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-connectors-check-panel-source" &&
        surface.files.includes(checkPanelSourcePath) &&
        surface.hash_algorithm === "sha256" &&
        surface.file_hashes?.[checkPanelSourcePath] &&
        surface.source_markers.includes(
          "automation_connectors_package_lane_row",
        ) &&
        surface.source_markers.includes(
          "check_panel.view_model.package_lane_rows",
        ),
    ),
    "Automation Connectors check-panel source surface is missing",
  );
  assert.deepEqual(
    automationsVisibility.inspected_upstream_files,
    receipt.inspected_upstream_files,
    "Automation Connectors package-status row should mirror inspected upstream source files",
  );
  assert.ok(
    automationsVisibility.inspected_upstream_files.includes(
      "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
    ),
    "Automation Connectors package-status row should expose inspected Slack V2 source",
  );
  assert.ok(
    automationsVisibility.inspected_upstream_files.includes(
      "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
    ),
    "Automation Connectors package-status row should expose inspected Webhook source",
  );
  assert.deepEqual(
    automationsVisibility.upstream_public_apis,
    receipt.upstream_public_apis,
    "Automation Connectors package-status row should mirror upstream public API boundaries",
  );
  for (const upstreamApi of [
    "ITriggerFunctions",
    "IExecuteFunctions",
    "IWebhookFunctions",
  ]) {
    assert.ok(
      automationsVisibility.upstream_public_apis.includes(upstreamApi),
      `${upstreamApi} missing from Automation Connectors package-status row`,
    );
  }

  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-launch-dashboard-workflow" &&
        surface.receipt_path === receiptPath &&
        surface.files.includes("components/template-app/automations-status.tsx") &&
        surface.source_markers.includes(
          'data-dx-component="launch-automation-dashboard-workflow"',
        ) &&
        surface.source_markers.includes(
          'data-dx-dashboard-workflow="automation-release-receipt"',
        ),
    ),
    "Automation Connectors launch workflow surface is missing",
  );
  assert.ok(
    automationsVisibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "automation-zed-run-handoff" &&
        surface.files.includes("components/template-app/automations-status.tsx") &&
        surface.source_markers.includes(
          'data-dx-automation-safe-action="prepare-zed-run-handoff"',
        ) &&
        surface.source_markers.includes(
          'data-dx-automation-handoff="zed-run-receipt"',
        ),
    ),
    "Automation Connectors Zed run handoff surface is missing",
  );

  for (const metric of [
    "automation_connectors_receipt_present",
    "automation_connectors_receipt_stale",
    "automation_connectors_missing_receipt",
    "automation_connectors_blocked_surface",
    "automation_connectors_unsupported_surface",
    "automation_connectors_hash_manifest_present",
    "automation_connectors_hash_mismatch",
    "automation_connectors_upstream_runtime_boundary_present",
    "automation_connectors_upstream_runtime_boundary_missing",
  ]) {
    assert.ok(
      automationsVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Automation Connectors visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.ok(
      receipt.dx_check_visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Automation Connectors receipt visibility`,
    );
  }

  assert.equal(receipt.official_package_name, "Automation Connectors");
  assert.equal(receipt.upstream_package, "n8n-nodes-base");
  assert.equal(receipt.upstream_version, "2.22.0");
  assert.equal(
    receipt.dx_check_visibility.schema,
    "dx.forge.package.dx_check_visibility",
  );
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) => surface.id === "automation-launch-dashboard-workflow",
    ),
  );
  assert.ok(
    receipt.file_hashes["examples/template/automations-status.tsx"],
    "Automation Connectors receipt should carry source file hashes",
  );
  assert.ok(
    receipt.file_hashes[sourceGuardRunbookPath],
    "Automation Connectors receipt should hash-track the source-guard runbook fixture",
  );
  assert.ok(
    receipt.file_hashes[previewManifestMaterializerPath],
    "Automation Connectors receipt should hash-track generated preview-manifest materialization",
  );
  assert.ok(
    receipt.file_hashes[studioManifestSourcePath],
    "Automation Connectors receipt should hash-track Studio manifest source discovery",
  );
  assert.ok(
    receipt.file_hashes[lowerDxCheckSourcePath],
    "Automation Connectors receipt should hash-track lower dx-check helper metrics",
  );
  assert.ok(
    receipt.file_hashes[checkPanelSourcePath],
    "Automation Connectors receipt should hash-track the shared check-panel source",
  );
  assert.ok(
    receipt.source_files.includes(previewManifestMaterializerPath),
    "Automation Connectors receipt source files should include the preview-manifest materializer",
  );
  assert.ok(
    receipt.source_files.includes(studioManifestSourcePath),
    "Automation Connectors receipt source files should include the Studio manifest source",
  );
  assert.ok(
    receipt.source_files.includes(lowerDxCheckSourcePath),
    "Automation Connectors receipt source files should include the lower dx-check source",
  );
  assert.ok(
    receipt.source_files.includes(checkPanelSourcePath),
    "Automation Connectors receipt source files should include the check-panel source",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id ===
          "automation-connectors-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath],
    ),
    "Automation Connectors receipt visibility should monitor materializer drift",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-studio-manifest-source" &&
        surface.file_hashes?.[studioManifestSourcePath],
    ),
    "Automation Connectors receipt visibility should monitor Studio manifest drift",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-lower-dx-check-source" &&
        surface.file_hashes?.[lowerDxCheckSourcePath],
    ),
    "Automation Connectors receipt visibility should monitor lower dx-check drift",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "automation-connectors-check-panel-source" &&
        surface.file_hashes?.[checkPanelSourcePath],
    ),
    "Automation Connectors receipt visibility should monitor check-panel drift",
  );

  assert.equal(sourceGuardRunbook.schema, "dx.forge.package.source_guard_runbook_fixture");
  assert.equal(sourceGuardRunbook.package.official_package_name, "Automation Connectors");
  assert.equal(sourceGuardRunbook.package.package_id, "automations/n8n");
  assert.equal(sourceGuardRunbook.package.upstream_package, "n8n-nodes-base");
  assert.equal(sourceGuardRunbook.package.upstream_version, "2.22.0");
  assert.equal(sourceGuardRunbook.guard.id, "automation-connectors-package-lane-panel");
  assert.equal(
    sourceGuardRunbook.guard.command,
    "dx run --test .\\benchmarks\\automations-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(sourceGuardRunbook.guard.starts_server, false);
  assert.equal(sourceGuardRunbook.guard.runs_package_install, false);
  assert.equal(sourceGuardRunbook.guard.runs_full_build, false);
  assert.equal(sourceGuardRunbook.receipt.source_guard_runbook_fixture, sourceGuardRunbookPath);
  assert.equal(
    sourceGuardRunbook.receipt.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(
    sourceGuardRunbook.receipt.studio_manifest_source,
    studioManifestSourcePath,
  );
  assert.equal(sourceGuardRunbook.receipt.lower_dx_check_source, lowerDxCheckSourcePath);
  assert.equal(sourceGuardRunbook.receipt.check_panel_source, checkPanelSourcePath);
  assert.equal(
    sourceGuardRunbook.receipt.tracked_file_count,
    receiptTrackedFileCount,
    "Automation Connectors source-guard runbook tracked file count should match the receipt hash manifest",
  );
  assert.equal(
    sourceGuardRunbook.preview_manifest.tracked_by_receipt_hash_helper,
    true,
  );
  assert.equal(
    sourceGuardRunbook.studio_manifest.tracked_by_receipt_hash_helper,
    true,
  );
  assert.equal(sourceGuardRunbook.honesty_label, "ADAPTER-BOUNDARY");
  assert.equal(sourceGuardRunbook.runtime_proof, false);

  const hashRefresh = automationsVisibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Automation Connectors hash refresh status is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(
    hashRefresh.helper_path,
    "examples/template/automation-connectors-receipt-hashes.ts",
  );
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/automation-connectors-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/automation-connectors-receipt-hashes.ts --write",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(
    hashRefresh.tracked_file_count,
    receiptTrackedFileCount,
  );
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, "automation-connectors:receipt-hash-refresh");
  assert.equal(hashRefresh.source_guard_runbook_fixture, sourceGuardRunbookPath);
  assert.equal(
    hashRefresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(hashRefresh.studio_manifest_source, studioManifestSourcePath);
  assert.equal(hashRefresh.lower_dx_check_source, lowerDxCheckSourcePath);
  assert.equal(hashRefresh.check_panel_source, checkPanelSourcePath);
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.deepEqual(hashRefresh.missing_files, []);

  const helper = spawnSync(
    process.execPath,
    [
      "examples/template/automation-connectors-receipt-hashes.ts",
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "automations/n8n");
  assert.equal(helperReport.status, hashRefresh.status);
  assert.equal(helperReport.tracked_file_count, hashRefresh.tracked_file_count);
  assert.equal(helperReport.stale_file_count, hashRefresh.stale_file_count);
  assert.equal(helperReport.missing_file_count, hashRefresh.missing_file_count);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);
  assert.equal(helperReport.source_guard_runbook_fixture, sourceGuardRunbookPath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(helperReport.studio_manifest_source, studioManifestSourcePath);
  assert.equal(helperReport.lower_dx_check_source, lowerDxCheckSourcePath);
  assert.equal(helperReport.check_panel_source, checkPanelSourcePath);
  assert.deepEqual(helperReport.stale_files, []);
  assert.deepEqual(helperReport.missing_files, []);
  assert.ok(helperReport.current_files.includes(previewManifestMaterializerPath));
  assert.ok(helperReport.current_files.includes(studioManifestSourcePath));
  assert.ok(helperReport.current_files.includes(lowerDxCheckSourcePath));
  assert.ok(helperReport.current_files.includes(checkPanelSourcePath));

  assert.match(readModel, /export const automationConnectorsPackageVisibility/);
  assert.match(readModel, /automationConnectorsPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /automation-connectors:receipt-hash-refresh/);
  assert.match(readModel, /sourceGuardRunbookFixture/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(readModel, /studioManifestSource/);
  assert.match(readModel, /lowerDxCheckSource/);
  assert.match(readModel, /checkPanelSource/);
  assert.match(readModel, /automation-connectors\.source-guard-runbook\.json/);
  assert.match(readModel, /automation-connectors-preview-manifest-materializer/);
  assert.match(readModel, /automation-connectors-studio-manifest-source/);
  assert.match(readModel, /automation-connectors-lower-dx-check-source/);
  assert.match(readModel, /automation-connectors-check-panel-source/);
  assert.match(readModel, /tools\/launch\/materialize-www-template\.ts/);
  assert.match(readModel, /dx-www\/src\/cli\/studio_manifest\.rs/);
  assert.match(
    readModel,
    /core\/src\/ecosystem\/project_check\/automation_connectors_dx_check\.rs/,
  );
  assert.match(readModel, /core\/src\/ecosystem\/dx_check_receipt\.rs/);
  assert.match(readModel, /inspectedUpstreamFiles/);
  assert.match(readModel, /upstreamPublicApis/);
  assert.match(readModel, /"packages\/nodes-base\/nodes\/Slack\/V2\/SlackV2\.node\.ts"/);
  assert.match(readModel, /"packages\/nodes-base\/nodes\/Webhook\/Webhook\.node\.ts"/);
  assert.match(readModel, /"IWebhookFunctions"/);
  assert.match(
    readModel,
    new RegExp(`trackedFileCount:\\s*${hashRefresh.tracked_file_count}`),
  );
  assert.match(
    readModel,
    /"examples\/template\/template-route-contract\.ts"/,
  );
  assert.doesNotMatch(readModel, /"dx-www\/src\/cli\/mod\.rs"/);
  assert.ok(
    !automationsVisibility.receipt_hash_refresh.tracked_files.includes(
      "dx-www/src/cli/mod.rs",
    ),
    "Automation Connectors should keep the legacy giant CLI module out of tracked hash proof",
  );
  assert.ok(
    !hashRefresh.tracked_files.includes("dx-www/src/cli/mod.rs"),
    "Automation Connectors receipt hash helper should not reintroduce dx-www/src/cli/mod.rs",
  );
  assert.match(readModel, /currentFiles: \[/);
  assert.match(readModel, /staleFiles: \[\]/);
  assert.match(readModel, /missingFiles: \[\]/);
  assert.match(statusSource, /automationConnectorsPackageVisibility/);
  assert.match(
    statusSource,
    /automationConnectorsVisibility: automationConnectorsPackageVisibility/,
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "automation-connectors:launch-dashboard-workflow",
    ),
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "automation-connectors:zed-run-handoff",
    ),
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "automation-connectors:receipt-hash-refresh",
    ),
  );
  assert.match(packageDoc, /shared dx-check\/Zed package-status read model/i);
  assert.match(packageDoc, /receipt_hash_refresh/);
});
