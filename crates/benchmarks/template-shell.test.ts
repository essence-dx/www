const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const catalogPath = path.join(root, "examples", "template", "package-catalog.ts");
const shellPath = path.join(root, "examples", "template", "template-shell.tsx");
const templateNodeModulesPath = path.join(root, "examples", "template", "node_modules");
const dashboardNavPath = path.join(root, "examples", "template", "template-dashboard-nav.tsx");
const leadFormPath = path.join(root, "examples", "template", "template-lead-form.tsx");
const shadcnDashboardControlsPath = path.join(root, "examples", "template", "shadcn-dashboard-controls.tsx");
const shadcnDashboardControlsContractPath = path.join(root, "examples", "template", "shadcn-dashboard-controls-contract.tsx");
const editContractPath = path.join(root, "examples", "template", "dx-studio-edit-contract.ts");
const markdownPreviewPath = path.join(root, "examples", "template", "react-markdown-preview.tsx");
const homeRoutePath = path.join(root, "examples", "template", "app", "page.tsx");
const routePath = homeRoutePath;
const routeContractPath = path.join(root, "examples", "template", "template-route-contract.ts");
const safetyArchiveContractPath = path.join(root, "examples", "template", "forge-safety-archive-contract.ts");
const safetyArchivePanelPath = path.join(root, "examples", "template", "forge-safety-archive-panel.tsx");
const remoteHeadHealthContractPath = path.join(root, "examples", "template", "forge-remote-head-health-contract.ts");
const remoteHeadHealthPanelPath = path.join(root, "examples", "template", "forge-remote-head-health-panel.tsx");
const templateSurfaceRegistryPath = path.join(root, "examples", "template", "template-surface-registry.ts");
const trpcExamplePath = path.join(root, "examples", "template", "trpc-launch-health.tsx");
const trpcContractPath = path.join(root, "examples", "template", "trpc-launch-contract.ts");
const authStatusPath = path.join(root, "examples", "template", "auth-session-status.tsx");
const aiChatPath = path.join(root, "examples", "template", "ai-chat-status.tsx");
const dataStatusPath = path.join(root, "examples", "template", "data-status.tsx");
const paymentsStatusPath = path.join(root, "examples", "template", "payments-status.tsx");
const docsStatusPath = path.join(root, "examples", "template", "docs-status.tsx");
const automationStatusPath = path.join(root, "examples", "template", "automations-status.tsx");
const automationMetadataPath = path.join(root, "examples", "template", "automations", "automations-metadata.ts");
const launchScenePath = path.join(root, "examples", "template", "launch-scene.tsx");
const sceneTypesPath = path.join(root, "examples", "template", "scene", "types.ts");
const scenePresetPath = path.join(root, "examples", "template", "scene", "preset.ts");
const sceneWebglRuntimePath = path.join(root, "examples", "template", "scene", "webgl-runtime.ts");
const sceneMetadataPath = path.join(root, "examples", "template", "scene", "metadata.ts");
const sceneReadmePath = path.join(root, "examples", "template", "scene", "README.md");
const queryStatusPath = path.join(root, "examples", "template", "query-cache-status.tsx");
const iconStatusPath = path.join(root, "examples", "template", "icon-status.tsx");
const nextIntlStatusPath = path.join(root, "examples", "template", "next-intl-status.tsx");
const zodStatusPath = path.join(root, "examples", "template", "zod-validation-status.tsx");
const instantStatusPath = path.join(root, "examples", "template", "instantdb-status.tsx");
const wasmStatusPath = path.join(root, "examples", "template", "wasm-interop-status.tsx");
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const cliNewCommandPath = path.join(root, "dx-www", "src", "cli", "new_command.rs");
const templateSourcesPath = path.join(root, "dx-www", "src", "cli", "default_template_sources.rs");
const tsxLaunchRuntimePath = path.join(root, "dx-www", "src", "cli", "tsx_launch_runtime.rs");
const launchReadinessBundlePath = path.join(root, "dx-www", "src", "cli", "launch_readiness_bundle.rs");
const runtimeEvidenceReviewPath = path.join(root, "dx-www", "src", "cli", "launch_runtime_evidence_review.rs");
const launchEvidencePacketPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_packet.rs");
const launchEvidenceOperatorIndexPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_operator_index.rs");
const launchEvidenceStatusTimelinePath = path.join(root, "dx-www", "src", "cli", "launch_evidence_status_timeline.rs");
const launchEvidenceHandoffDigestPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_handoff_digest.rs");
const launchEvidenceReleaseChecklistPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_release_checklist.rs");
const launchEvidenceShareManifestPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_share_manifest.rs");
const launchEvidenceArchiveIndexPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_archive_index.rs");
const launchEvidenceArchiveReceiptPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_archive_receipt.rs");
const launchEvidenceArchiveLedgerPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_archive_ledger.rs");
const launchEvidenceRetentionPolicyPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_retention_policy.rs");
const launchEvidenceRetentionReviewPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_retention_review.rs");
const launchEvidenceReleaseSealPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_release_seal.rs");
const launchEvidenceOperatorSummaryPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_operator_summary.rs");
const launchEvidenceCompletionLedgerPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_completion_ledger.rs");
const launchEvidenceClosureMemoPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_closure_memo.rs");
const launchEvidenceFinalBriefPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_final_brief.rs");
const launchEvidenceOperatorRunbookPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_operator_runbook.rs");
const launchEvidenceHandoffCapsulePath = path.join(root, "dx-www", "src", "cli", "launch_evidence_handoff_capsule.rs");
const launchEvidenceResumptionIndexPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_resumption_index.rs");
const launchEvidenceRecoveryBriefPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_recovery_brief.rs");
const launchEvidenceContinuationPacketPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_continuation_packet.rs");
const launchEvidenceOperatorResumeCardPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_operator_resume_card.rs");
const launchEvidenceRestartLedgerPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_ledger.rs");
const launchEvidenceRestartChecklistPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_checklist.rs");
const launchEvidenceRestartBriefPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_brief.rs");
const launchEvidenceRestartManifestPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_manifest.rs");
const launchEvidenceRestartReceiptPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_receipt.rs");
const launchEvidenceRestartSummaryPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_summary.rs");
const launchEvidenceRestartSnapshotPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_snapshot.rs");
const launchEvidenceRestartDispatchPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_dispatch.rs");
const launchEvidenceRestartCloseoutPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_closeout.rs");
const launchEvidenceRestartSignoffPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_restart_signoff.rs");
const launchEvidenceAcceptanceIndexPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_acceptance_index.rs");
const launchEvidenceAcceptanceDigestPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_acceptance_digest.rs");
const launchEvidenceFridayBatonPath = path.join(root, "dx-www", "src", "cli", "launch_evidence_friday_baton.rs");
const supabaseForgePath = path.join(root, "core", "src", "ecosystem", "forge_supabase.rs");
const forgeRegistryPath = path.join(root, "core", "src", "ecosystem", "forge_registry.rs");

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("template shell consumes the combined package catalog", () => {
  const catalog = readRequiredFile(catalogPath);
  const shell = readRequiredFile(shellPath);
  const dashboardNav = readRequiredFile(dashboardNavPath);
  const leadForm = readRequiredFile(leadFormPath);
  const shadcnDashboardControls = readRequiredFile(shadcnDashboardControlsPath);
  const shadcnDashboardControlsContract = readRequiredFile(shadcnDashboardControlsContractPath);
  const editContract = readRequiredFile(editContractPath);
  const markdownPreview = readRequiredFile(markdownPreviewPath);
  const homeRoute = readRequiredFile(homeRoutePath);
  const route = readRequiredFile(routePath);
  const routeContract = readRequiredFile(routeContractPath);
  const safetyArchiveContract = readRequiredFile(safetyArchiveContractPath);
  const safetyArchivePanel = readRequiredFile(safetyArchivePanelPath);
  const remoteHeadHealthContract = readRequiredFile(remoteHeadHealthContractPath);
  const remoteHeadHealthPanel = readRequiredFile(remoteHeadHealthPanelPath);
  const templateSurfaceRegistry = readRequiredFile(templateSurfaceRegistryPath);
  const trpcExample = readRequiredFile(trpcExamplePath);
  const trpcContract = readRequiredFile(trpcContractPath);
  const authStatus = readRequiredFile(authStatusPath);
  const aiChat = readRequiredFile(aiChatPath);
  const dataStatus = readRequiredFile(dataStatusPath);
  const paymentsStatus = readRequiredFile(paymentsStatusPath);
  const docsStatus = readRequiredFile(docsStatusPath);
  const automationStatus = readRequiredFile(automationStatusPath);
  const automationMetadata = readRequiredFile(automationMetadataPath);
  const launchScene = readRequiredFile(launchScenePath);
  const sceneTypes = readRequiredFile(sceneTypesPath);
  const scenePreset = readRequiredFile(scenePresetPath);
  const sceneWebglRuntime = readRequiredFile(sceneWebglRuntimePath);
  const sceneMetadata = readRequiredFile(sceneMetadataPath);
  const sceneReadme = readRequiredFile(sceneReadmePath);
  const queryStatus = readRequiredFile(queryStatusPath);
  const iconStatus = readRequiredFile(iconStatusPath);
  const nextIntlStatus = readRequiredFile(nextIntlStatusPath);
  const zodStatus = readRequiredFile(zodStatusPath);
  const instantStatus = readRequiredFile(instantStatusPath);
  const wasmStatus = readRequiredFile(wasmStatusPath);
  const cliMod = readRequiredFile(cliPath);
  const cliNewCommand = readRequiredFile(cliNewCommandPath);
  const cli = [cliMod, cliNewCommand].join("\n");
  const templateSources = readRequiredFile(templateSourcesPath);
  const tsxLaunchRuntime = readRequiredFile(tsxLaunchRuntimePath);
  const launchReadinessBundle = readRequiredFile(launchReadinessBundlePath);
  const runtimeEvidenceReview = readRequiredFile(runtimeEvidenceReviewPath);
  const launchEvidencePacket = readRequiredFile(launchEvidencePacketPath);
  const launchEvidenceOperatorIndex = readRequiredFile(launchEvidenceOperatorIndexPath);
  const launchEvidenceStatusTimeline = readRequiredFile(launchEvidenceStatusTimelinePath);
  const launchEvidenceHandoffDigest = readRequiredFile(launchEvidenceHandoffDigestPath);
  const launchEvidenceReleaseChecklist = readRequiredFile(launchEvidenceReleaseChecklistPath);
  const launchEvidenceShareManifest = readRequiredFile(launchEvidenceShareManifestPath);
  const launchEvidenceArchiveIndex = readRequiredFile(launchEvidenceArchiveIndexPath);
  const launchEvidenceArchiveReceipt = readRequiredFile(launchEvidenceArchiveReceiptPath);
  const launchEvidenceArchiveLedger = readRequiredFile(launchEvidenceArchiveLedgerPath);
  const launchEvidenceRetentionPolicy = readRequiredFile(launchEvidenceRetentionPolicyPath);
  const launchEvidenceRetentionReview = readRequiredFile(launchEvidenceRetentionReviewPath);
  const launchEvidenceReleaseSeal = readRequiredFile(launchEvidenceReleaseSealPath);
  const launchEvidenceOperatorSummary = readRequiredFile(launchEvidenceOperatorSummaryPath);
  const launchEvidenceCompletionLedger = readRequiredFile(launchEvidenceCompletionLedgerPath);
  const launchEvidenceClosureMemo = readRequiredFile(launchEvidenceClosureMemoPath);
  const launchEvidenceFinalBrief = readRequiredFile(launchEvidenceFinalBriefPath);
  const launchEvidenceOperatorRunbook = readRequiredFile(launchEvidenceOperatorRunbookPath);
  const launchEvidenceHandoffCapsule = readRequiredFile(launchEvidenceHandoffCapsulePath);
  const launchEvidenceResumptionIndex = readRequiredFile(launchEvidenceResumptionIndexPath);
  const launchEvidenceRecoveryBrief = readRequiredFile(launchEvidenceRecoveryBriefPath);
  const launchEvidenceContinuationPacket = readRequiredFile(launchEvidenceContinuationPacketPath);
  const launchEvidenceOperatorResumeCard = readRequiredFile(launchEvidenceOperatorResumeCardPath);
  const launchEvidenceRestartLedger = readRequiredFile(launchEvidenceRestartLedgerPath);
  const launchEvidenceRestartChecklist = readRequiredFile(launchEvidenceRestartChecklistPath);
  const launchEvidenceRestartBrief = readRequiredFile(launchEvidenceRestartBriefPath);
  const launchEvidenceRestartManifest = readRequiredFile(launchEvidenceRestartManifestPath);
  const launchEvidenceRestartReceipt = readRequiredFile(launchEvidenceRestartReceiptPath);
  const launchEvidenceRestartSummary = readRequiredFile(launchEvidenceRestartSummaryPath);
  const launchEvidenceRestartSnapshot = readRequiredFile(launchEvidenceRestartSnapshotPath);
  const launchEvidenceRestartDispatch = readRequiredFile(launchEvidenceRestartDispatchPath);
  const launchEvidenceRestartCloseout = readRequiredFile(launchEvidenceRestartCloseoutPath);
  const launchEvidenceRestartSignoff = readRequiredFile(launchEvidenceRestartSignoffPath);
  const launchEvidenceAcceptanceIndex = readRequiredFile(launchEvidenceAcceptanceIndexPath);
  const launchEvidenceAcceptanceDigest = readRequiredFile(launchEvidenceAcceptanceDigestPath);
  const launchEvidenceFridayBaton = readRequiredFile(launchEvidenceFridayBatonPath);
  const supabaseForge = readRequiredFile(supabaseForgePath);
  const forgeRegistry = readRequiredFile(forgeRegistryPath);
  const cliProduction = [cliMod.split("\n#[cfg(test)]")[0], cliNewCommand].join("\n");

  assert.ok(
    !fs.existsSync(templateNodeModulesPath),
    "www template must not contain a local node_modules folder",
  );

  for (const packageId of [
    "shadcn/ui/badge",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
    "auth/better-auth",
    "animation/motion",
    "i18n/next-intl",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "payments/stripe-js",
    "automations/n8n",
    "state/zustand",
    "ai/vercel-ai",
    "api/trpc",
    "content/fumadocs-next",
    "content/react-markdown",
    "supabase/client",
    "db/drizzle-sqlite",
    "instantdb/react",
    "wasm/bindgen",
    "3d/launch-scene",
  ]) {
    assert.match(catalog, new RegExp(`packageId: "${packageId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`));
  }

  assert.match(catalog, /export function launchPackageRoleSummary/);
  assert.match(shell, /launchPackageRoleSummary/);
  assert.match(shell, /route = "\/"/);
  assert.match(shell, /data-dx-route=\{route\}/);
  assert.match(shell, /data-dx-source="examples\/template\/template-shell\.tsx"/);
  assert.match(shell, /data-dx-forge="dx-www\/template-shell"/);
  assert.match(shell, /data-dx-package="dx-www\/template-shell"/);
  assert.match(shell, /data-dx-hot-reload-target=\{`route:\$\{route\}`\}/);
  assert.match(tsxLaunchRuntime, /render_template_shell_tsx_route/);
  assert.match(tsxLaunchRuntime, /data-dx-renderer="tsx-app-router-template"/);
  assert.match(tsxLaunchRuntime, /data-dx-route="__DX_ROUTE__"/);
  assert.match(tsxLaunchRuntime, /data-dx-source="__DX_SOURCE__"/);
  assert.match(tsxLaunchRuntime, /data-dx-component-source="components\/template-app\/template-shell\.tsx"/);
  assert.match(tsxLaunchRuntime, /app\/page\.tsx/);
  assert.doesNotMatch(tsxLaunchRuntime, /app\/launch\/page\.tsx/);
  assert.match(tsxLaunchRuntime, /components\/template-app\/template-shell\.tsx/);
  assert.doesNotMatch(
    tsxLaunchRuntime,
    /tools[\\/]launch[\\/]runtime-template[\\/]pages[\\/](?:index|launch)\.html|pages[\\/](?:index|launch)\.html|launch\.cm/,
  );
  assert.match(shell, /data-dx-package-maturity="mixed-source-slices-and-adapter-boundaries"/);
  assert.match(shell, /data-dx-ready="source-owned-package-shell"/);
  assert.match(shell, /data-dx-node-modules="forbidden"/);
  assert.match(cliNewCommand, /project\(name="\{\}" version=0\.1\.0 kind=www-app\)/);
  assert.match(cliNewCommand, /www\(\s+app_dir=app\s+output_dir=\.dx\/www\/output\s+\)/);
  assert.match(cliNewCommand, /style\(\s+mode=generated-css\s+tokens=styles\/theme\.css\s+generated_css=styles\/generated\.css\s+\)/);
  assert.match(
    cliNewCommand,
    /icons\(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components\/icons\)/,
  );
  assert.match(cliNewCommand, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(cliNewCommand, /check\(score_scale=500 lighthouse=true\)/);
  assert.match(cliNewCommand, /docs\(\s+route=\/docs\s+content=content\/docs\s+openapi=openapi\/dx-www\.yaml\s+\)/);
  assert.doesNotMatch(cliNewCommand, /paths\[name value\]\(|tooling\.|shadcn\.|next\.config\.js/);
  assert.ok(
    cliProduction.indexOf("Self::write_next_familiar_launch_route(&project_dir)?") <
      cliProduction.indexOf("Self::write_launch_forge_package_slices(&project_dir)?"),
    "dx new must materialize TSX template route files before Forge package slice writes",
  );
  assert.match(forgeRegistry, /\("js\/openapi"\.to_string\(\), "openapi"\.to_string\(\)\)/);
  assert.match(forgeRegistry, /materialize_path\("js\/openapi\/dx-launch\.yaml"\)/);
  assert.match(shell, /import \{ LaunchShadcnDashboardControls \} from "\.\/shadcn-dashboard-controls";/);
  assert.match(shell, /import \{ ForgeSafetyArchivePanel \} from "\.\/forge-safety-archive-panel";/);
  assert.match(shell, /import \{ ForgeRemoteHeadHealthPanel \} from "\.\/forge-remote-head-health-panel";/);
  assert.match(shell, /<LaunchShadcnDashboardControls \/>/);
  assert.match(shell, /<ForgeSafetyArchivePanel \/>/);
  assert.match(shell, /<ForgeRemoteHeadHealthPanel \/>/);
  assert.match(routeContract, /components\/template-app\/forge-safety-archive-contract\.ts/);
  assert.match(routeContract, /components\/template-app\/forge-safety-archive-panel\.tsx/);
  assert.match(routeContract, /components\/template-app\/forge-remote-head-health-contract\.ts/);
  assert.match(routeContract, /components\/template-app\/forge-remote-head-health-panel\.tsx/);
  assert.match(safetyArchiveContract, /dx\.forge\.safety_archive_contract/);
  assert.match(safetyArchiveContract, /launchForgePackageStatus\.safetyArchiveStatus/);
  assert.match(safetyArchiveContract, /operationSafetySurface: "archive-before-delete"/);
  assert.match(safetyArchiveContract, /safeForDestructivePackageOperations/);
  assert.match(safetyArchiveContract, /Generate missing safety archive receipts/);
  assert.match(safetyArchivePanel, /forgeSafetyArchiveContract/);
  assert.match(safetyArchivePanel, /data-dx-safety-archive-contract=\{panelContract\.schema\}/);
  assert.match(safetyArchivePanel, /data-dx-component=\{panelContract\.component\}/);
  assert.match(safetyArchivePanel, /data-dx-zed-surface=\{panelContract\.zedSurface\}/);
  assert.match(
    safetyArchivePanel,
    /data-dx-safety-archive-rollback-coverage=\{[\s\S]*?panelContract\.rollbackCoveragePercent[\s\S]*?\}/,
  );
  assert.match(remoteHeadHealthContract, /dx\.forge\.remote_head_health_panel_contract/);
  assert.match(remoteHeadHealthContract, /createForgeRemoteHeadHealthPanelContract/);
  assert.match(remoteHeadHealthContract, /launchForgePackageStatus\.remoteObjectHeadHealth/);
  assert.match(remoteHeadHealthContract, /zedSurface: "remote-object-head-health"/);
  assert.match(remoteHeadHealthContract, /safeForRemoteInstall/);
  assert.match(remoteHeadHealthContract, /dx forge remote-head/);
  assert.match(remoteHeadHealthPanel, /forgeRemoteHeadHealthPanelContract/);
  assert.match(remoteHeadHealthPanel, /data-dx-remote-head-contract=\{panelContract\.schema\}/);
  assert.match(remoteHeadHealthPanel, /data-dx-component=\{panelContract\.component\}/);
  assert.match(remoteHeadHealthPanel, /data-dx-zed-surface=\{panelContract\.zedSurface\}/);
  assert.match(cliProduction, /NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_CONTRACT_TS/);
  assert.match(cliProduction, /NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_PANEL_TSX/);
  assert.match(cliProduction, /NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_CONTRACT_TS/);
  assert.match(cliProduction, /NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_PANEL_TSX/);
  assert.match(cliProduction, /examples\/template\/forge-safety-archive-contract\.ts/);
  assert.match(cliProduction, /examples\/template\/forge-safety-archive-panel\.tsx/);
  assert.match(cliProduction, /examples\/template\/forge-remote-head-health-contract\.ts/);
  assert.match(cliProduction, /examples\/template\/forge-remote-head-health-panel\.tsx/);
  assert.match(cliProduction, /components\/template-app\/forge-safety-archive-contract\.ts/);
  assert.match(cliProduction, /components\/template-app\/forge-safety-archive-panel\.tsx/);
  assert.match(cliProduction, /components\/template-app\/forge-remote-head-health-contract\.ts/);
  assert.match(cliProduction, /components\/template-app\/forge-remote-head-health-panel\.tsx/);
  assert.match(shell, /data-dx-section="launch-dashboard-controls"/);
  assert.match(shell, /data-dx-editable-section="launch-dashboard-controls"/);
  assert.doesNotMatch(shell, /LaunchShadcnUiProof/);
  assert.doesNotMatch(shell, /data-dx-component="shadcn-ui-proof-card"/);
  assert.match(shell, /data-dx-edit-contract=\{launchStudioEditContract\.schema\}/);
  assert.match(shell, /data-dx-edit-ops="insert_component,move_reorder_section,update_design_token,update_text_content,insert_icon_media"/);
  assert.match(shell, /data-dx-section="hero"/);
  assert.match(shell, /data-dx-section="proof-grid"/);
  assert.match(shell, /data-dx-section="studio-proof-flow"/);
  assert.match(shell, /data-dx-section="package-catalog"/);
  assert.match(shell, /data-dx-editable-section="launch-studio-proof-flow"/);
  assert.match(shell, /data-dx-media-slot="launch-scene"/);
  assert.match(shell, /data-dx-package=\{item\.packageId\}/);
  assert.match(shell, /data-dx-package-maturity=\{item\.maturity\}/);
  assert.match(shadcnDashboardControlsContract, /export const shadcnLaunchDashboardMetadata/);
  assert.match(shadcnDashboardControlsContract, /sourceMirror: "G:\/WWW\/inspirations\/shadcn-ui"/);
  assert.match(shadcnDashboardControlsContract, /export function createShadcnLaunchDashboardReceipt/);
  assert.match(shadcnDashboardControls, /from "\.\/shadcn-dashboard-controls-contract"/);
  assert.match(shadcnDashboardControls, /data-dx-component="shadcn-dashboard-controls"/);
  assert.match(shadcnDashboardControls, /data-dx-dashboard-workflow="operator-controls"/);
  assert.match(shadcnDashboardControls, /data-dx-shadcn-dashboard-action="set-density"/);
  assert.match(shadcnDashboardControls, /data-dx-shadcn-dashboard-action="select-queue"/);
  assert.match(shadcnDashboardControls, /data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/);
  assert.match(shadcnDashboardControls, /data-slot="button"/);
  assert.match(shadcnDashboardControls, /data-slot="field"/);
  assert.match(shadcnDashboardControls, /data-slot="item"/);
  assert.match(shadcnDashboardControls, /<dx-icon name="pack:settings"/);
  assert.doesNotMatch(shadcnDashboardControls, /#[0-9a-fA-F]{3,8}|rgb\(|hsl\(/);
  assert.match(shell, /import \{[\s\S]*ItemActions[\s\S]*ItemContent[\s\S]*ItemGroup[\s\S]*ItemMedia[\s\S]*ItemTitle[\s\S]*\} from "@\/components\/ui\/item";/);
  assert.match(shell, /data-dx-package="shadcn\/ui\/item"/);
  assert.match(shell, /const displayName = launchPackageDisplayName\(item\);/);
  assert.match(shell, /<ItemTitle[\s\S]*\{displayName\}[\s\S]*<\/ItemTitle>/);
  assert.match(
    shell,
    /<p className="mt-1 break-all text-xs text-muted-foreground">\s*\{item\.packageId\}\s*<\/p>/,
  );
  assert.match(shell, /data-dx-package-role=\{item\.role\}/);
  assert.match(shell, /<Badge variant="outline">\{item\.maturity\}<\/Badge>/);
  assert.match(shell, /data-dx-source="examples\/template\/package-catalog\.ts"/);
  assert.match(shell, /data-dx-forge=\{item\.command\}/);
  assert.match(shell, /DxMotionPresence/);
  assert.match(shell, /MotionControlledStatus/);
  assert.match(shell, /DxLazyMotionProvider/);
  assert.match(shell, /MotionLazyBox/);
  assert.match(shell, /MotionPresenceItem/);
  assert.match(shell, /DxMotionLayoutGroup/);
  assert.match(shell, /MotionLayoutItem/);
  assert.match(shell, /useDxInstantLayoutTransition/);
  assert.match(shell, /dxMotionLayoutId/);
  assert.match(shell, /MotionValueMeter/);
  assert.match(shell, /DxReorderGroup/);
  assert.match(shell, /DxReorderItem/);
  assert.match(shell, /useDxReorderControls/);
  assert.match(shell, /MotionScrollProgress/);
  assert.match(shell, /useDxMotionPressFeedback/);
  assert.match(shell, /data-dx-motion="launch-scroll-progress"/);
  assert.match(shell, /data-dx-motion="launch-controlled-status"/);
  assert.match(shell, /data-dx-motion="launch-lazy-capability-row"/);
  assert.match(shell, /data-dx-motion="launch-package-meter"/);
  assert.match(shell, /data-dx-motion="launch-role-meter"/);
  assert.match(shell, /data-dx-motion="launch-package-presence"/);
  assert.match(shell, /data-dx-motion="launch-package-layout"/);
  assert.match(shell, /data-dx-motion="launch-package-reorder"/);
  assert.match(shell, /data-dx-motion="launch-package-drag-handle"/);
  assert.match(shell, /data-dx-motion="pressable"/);
  assert.match(shell, /LaunchDashboardSidebar/);
  assert.match(shell, /LaunchCommandBar/);
  assert.match(shell, /CapabilityBadge/);
  assert.match(shell, /LaunchLeadForm/);
  assert.match(shell, /import \{ TrpcLaunchHealth \} from "\.\/trpc-launch-health";/);
  assert.match(shell, /<TrpcLaunchHealth \/>/);
  assert.match(shell, /data-dx-component="launch-trpc-api-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-card="typed-api"/);
  assert.match(shell, /data-dx-dashboard-workflow="typed-api-readiness"/);
  assert.match(shell, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(shell, /wwwTemplateSlotSummary/);
  assert.match(shell, /launchPackageWorkerRegistry/);
  assert.match(shell, /data-dx-component="template-worker-registry"/);
  assert.match(shell, /data-dx-component="template-worker-slot"/);
  assert.match(shell, /data-dx-template-registry-source="examples\/template\/template-surface-registry\.ts"/);
  assert.match(templateSurfaceRegistry, /schema: "dx\.www\.template_surface_registry"/);
  assert.match(templateSurfaceRegistry, /export const wwwTemplateSurfaces/);
  assert.match(templateSurfaceRegistry, /export const launchPackageWorkerRegistry/);
  assert.match(templateSurfaceRegistry, /export function wwwTemplateSlotSummary/);
  assert.match(templateSurfaceRegistry, /id: "account-access"/);
  assert.match(templateSurfaceRegistry, /id: "settings-validation"/);
  assert.match(templateSurfaceRegistry, /id: "billing"/);
  assert.match(templateSurfaceRegistry, /id: "data-backend"/);
  assert.match(templateSurfaceRegistry, /id: "state-query"/);
  assert.match(templateSurfaceRegistry, /id: "content-docs"/);
  assert.match(templateSurfaceRegistry, /id: "automation-ai-visuals"/);
  assert.match(templateSurfaceRegistry, /noNodeModulesRequired: true/);
  assert.match(cli, /NEXT_FAMILIAR_TEMPLATE_SURFACE_REGISTRY_TS/);
  assert.match(cli, /"components\/template-app\/template-surface-registry\.ts"/);
  assert.doesNotMatch(cliProduction, /data-dx-component="launch-trpc-route-proof"/);
  assert.match(shell, /LaunchAutomationBridgeStatus/);
  assert.match(shell, /data-dx-section="launch-automation-ops"/);
  assert.match(shell, /data-dx-component="launch-automation-dashboard-workflow"/);
  assert.match(shell, /surface="dashboard"/);
  assert.match(shell, /onWorkflowChange=\{setAutomationDashboardState\}/);
  assert.match(automationStatus, /automationSummary/);
  assert.match(automationStatus, /data-dx-package="automations\/n8n"/);
  assert.match(automationMetadata, /automationRoutes/);
  assert.doesNotMatch(routeContract, /shadcn-ui-proof/);
  assert.match(routeContract, /"components\/template-app\/automations-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/automations\/automations-metadata\.ts"/);
  assert.doesNotMatch(cli, /NEXT_FAMILIAR_SHADCN_UI_PROOF_TSX|shadcn-ui-proof/);
  assert.match(cli, /"components\/template-app\/automations-status\.tsx"/);
  assert.match(cli, /"components\/template-app\/automations\/automations-metadata\.ts"/);
  assert.match(shell, /import \{ Separator \} from "@\/components\/ui\/separator";/);
  assert.match(shell, /data-dx-package="shadcn\/ui\/separator"/);
  assert.match(dashboardNav, /export function LaunchDashboardSidebar/);
  assert.match(dashboardNav, /export function LaunchCommandBar/);
  assert.match(dashboardNav, /export function CapabilityBadge/);
  assert.match(dashboardNav, /data-dx-component="launch-dashboard-sidebar"/);
  assert.match(dashboardNav, /data-dx-insert-slot="launch-section-nav"/);
  assert.match(dashboardNav, /href: "#docs"/);
  assert.match(dashboardNav, /href: "#payments"/);
  assert.match(dashboardNav, /href: "#forms"/);
  assert.doesNotMatch(dashboardNav, /<Button\s+type="button"[^>]*>/);
  assert.match(leadForm, /DxHookForm/);
  assert.match(leadForm, /DxInputField/);
  assert.match(leadForm, /import \{[\s\S]*FieldError[\s\S]*FieldGroup[\s\S]*FieldLabel[\s\S]*\} from "@\/components\/ui\/field";/);
  assert.match(leadForm, /data-dx-package="shadcn\/ui\/field"/);
  assert.match(leadForm, /data-dx-component="template-lead-form"/);
  assert.match(leadForm, /data-dx-insert-slot="template-lead-form"/);
  assert.match(leadForm, /<FieldError[\s\S]*errors=\{\[form\.formState\.errors\.notes\]\}/);
  assert.match(leadForm, /<FieldLabel htmlFor="launch-notes">Launch notes<\/FieldLabel>/);
  assert.match(leadForm, /<FieldDescription id="launch-notes-help">/);
  assert.match(leadForm, /Textarea/);
  assert.match(editContract, /schema: "dx\.studio\.launch_edit_contract"/);
  assert.match(editContract, /operation: "move_reorder_section"/);
  assert.match(editContract, /selector: '\[data-dx-section="hero"\]'/);
  assert.match(editContract, /absolutePositioning: false/);
  assert.match(editContract, /noNodeModulesRequired: true/);
  assert.match(shell, /DxMarkdown/);
  assert.match(markdownPreview, /DxMarkdown/);
  assert.match(markdownPreview, /skipHtml/);
  assert.match(homeRoute, /import \{ TemplateLandingPage \} from "@\/components\/template-app\/landing-page";/);
  assert.match(homeRoute, /const wwwFrameworkMetrics = \{/);
  assert.match(homeRoute, /runtime: "App Router authoring"/);
  assert.match(homeRoute, /<TemplateLandingPage metrics=\{wwwFrameworkMetrics\} \/>/);
  assert.doesNotMatch(homeRoute, /TemplateShell|templateRouteContract|DxIntlProvider|loadDxMessages/);
  assert.match(route, /import \{ TemplateLandingPage \} from "@\/components\/template-app\/landing-page";/);
  assert.match(route, /const wwwFrameworkMetrics = \{/);
  assert.match(route, /runtime: "App Router authoring"/);
  assert.match(route, /<TemplateLandingPage metrics=\{wwwFrameworkMetrics\} \/>/);
  assert.doesNotMatch(route, /TemplateShell|templateRouteContract|DxIntlProvider|loadDxMessages/);
  assert.doesNotMatch(route, /LaunchMarkdownPreview/);
  assert.doesNotMatch(route, /TrpcLaunchHealth/);
  assert.doesNotMatch(route, /data-dx-component="launch-trpc-route-proof"/);
  assert.match(route, /metadata/);
  assert.doesNotMatch(route, /templateRouteContract/);
  assert.match(routeContract, /route: "\/"/);
  assert.match(routeContract, /routeAliases: \[\]/);
  assert.match(routeContract, /sourceRouteFile: "examples\/template\/app\/page\.tsx"/);
  assert.match(routeContract, /materializedRouteFile: "app\/page\.tsx"/);
  assert.doesNotMatch(routeContract, /secondaryRouteFile/);
  assert.match(routeContract, /sourceSmokeCommand: "dx run --test \.\\\\benchmarks\\\\template-shell\.test\.ts"/);
  assert.match(routeContract, /runtimeVerification: "pending-governed-runtime-pass"/);
  assert.match(routeContract, /runtimeVerificationRequiresExplicitPermission: true/);
  assert.match(routeContract, /runtimeVerificationRequest/);
  assert.match(routeContract, /approvalStatus: "requires-explicit-permission"/);
  assert.match(routeContract, /automationDefault: "skip-runtime-build-preview"/);
  assert.match(routeContract, /expectedEvidence: \[/);
  assert.match(routeContract, /file: "\.dx\/forge\/template-readiness\/launch-route\.json"/);
  assert.match(routeContract, /launchReadinessBundle/);
  assert.match(routeContract, /file: "\.dx\/forge\/template-readiness\/launch-readiness-bundle\.json"/);
  assert.match(routeContract, /templateReadinessReceipt/);
  assert.match(routeContract, /packageId: "dx-www\/template-shell"/);
  assert.match(routeContract, /variant: "next-familiar"/);
  assert.match(routeContract, /export const launchRouteMaterializedFiles = \[/);
  assert.match(routeContract, /"components\/template-app\/auth-session-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/template-dashboard-nav\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/template-lead-form\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/ai-chat-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/data-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/payments-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/docs-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/template-surface-registry\.ts"/);
  assert.match(routeContract, /"components\/scene\/launch-scene\.tsx"/);
  assert.match(routeContract, /"lib\/scene\/types\.ts"/);
  assert.match(routeContract, /"lib\/scene\/preset\.ts"/);
  assert.match(routeContract, /"lib\/scene\/webgl-runtime\.ts"/);
  assert.match(routeContract, /"lib\/scene\/metadata\.ts"/);
  assert.match(routeContract, /"lib\/scene\/README\.md"/);
  assert.match(routeContract, /"components\/template-app\/instantdb-status\.tsx"/);
  assert.match(
    routeContract,
    /\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json/,
  );
  assert.match(routeContract, /instantDbRealtimeDashboard/);
  assert.match(routeContract, /packageId: "instantdb\/react"/);
  assert.match(routeContract, /component: "instantdb-runtime-dashboard-workflow"/);
  assert.match(routeContract, /dashboardWorkflow: "realtime-data-readiness"/);
  assert.match(routeContract, /runtimeSourceFile: "examples\/template\/app\/page\.tsx"/);
  assert.match(routeContract, /instantdb-dashboard-workflow\.test\.ts/);
  assert.match(routeContract, /"components\/template-app\/wasm-interop-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/zod-validation-status\.tsx"/);
  assert.match(routeContract, /"components\/template-app\/template-console\.tsx"/);
  assert.match(routeContract, /"server\/templateCatalog\.ts"/);
  assert.match(routeContract, /materializedFiles: launchRouteMaterializedFiles/);
  assert.match(routeContract, /export function launchRouteReadinessSummary/);
  assert.match(routeContract, /materializedFileCount: launchRouteMaterializedFiles\.length/);
  assert.match(shell, /LaunchCounterControl/);
  assert.match(shell, /LaunchAuthSessionStatus/);
  assert.match(authStatus, /@\/auth\/better-auth\/client/);
  assert.match(authStatus, /useSession/);
  assert.match(authStatus, /signOut/);
  assert.match(authStatus, /data\?\.user/);
  assert.match(shell, /LaunchAiChatStatus/);
  assert.match(aiChat, /@\/lib\/ai\/client-chat/);
  assert.match(aiChat, /DxAIClientChat/);
  assert.match(aiChat, /\/api\/ai\/chat/);
  assert.match(aiChat, /initialMessages/);
  assert.match(shell, /LaunchDataStatus/);
  assert.match(dataStatus, /@\/db\/drizzle\/metadata/);
  assert.match(dataStatus, /@\/lib\/supabase\/env/);
  assert.match(dataStatus, /@\/lib\/instant\/metadata/);
  assert.match(dataStatus, /readSupabasePublicConfig/);
  assert.match(dataStatus, /dxDrizzlePackage/);
  assert.match(dataStatus, /data-dx-data-status/);
  assert.match(shell, /LaunchPaymentStatus/);
  assert.match(paymentsStatus, /@\/lib\/payments\/stripe-js\/config/);
  assert.match(paymentsStatus, /@\/lib\/payments\/stripe-js\/checkout/);
  assert.match(paymentsStatus, /@\/lib\/payments\/stripe-js\/dashboard-checkout/);
  assert.match(paymentsStatus, /@\/lib\/forms\/react-hook-form\/form/);
  assert.match(paymentsStatus, /createDxZodResolver/);
  assert.match(paymentsStatus, /readDxStripeClientConfig/);
  assert.match(paymentsStatus, /submitDxStripeCheckoutContact/);
  assert.match(paymentsStatus, /createDxStripeDashboardCheckoutRequest/);
  assert.match(paymentsStatus, /createDxStripeDashboardMissingConfigReceipt/);
  assert.match(paymentsStatus, /LaunchCheckoutContactValues/);
  assert.match(paymentsStatus, /formSubmitState/);
  assert.match(paymentsStatus, /isSubmitting/);
  assert.match(paymentsStatus, /NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY/);
  assert.match(paymentsStatus, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(paymentsStatus, /data-dx-dashboard-flow="billing-checkout"/);
  assert.match(paymentsStatus, /data-dx-payment-status/);
  assert.doesNotMatch(paymentsStatus, /cardNumber|4242|fake card/i);
  assert.match(shell, /LaunchDocsStatus/);
  assert.match(docsStatus, /@\/lib\/fumadocs\/route-contract/);
  assert.match(docsStatus, /dxFumadocsRouteContract/);
  assert.match(docsStatus, /DxMarkdown/);
  assert.match(docsStatus, /data-dx-docs-status/);
  assert.match(shell, /LaunchScene/);
  assert.match(launchScene, /createDxLaunchScenePreset/);
  assert.match(launchScene, /createDxSceneRendererHandoff/);
  assert.match(launchScene, /mountDxSceneWithRenderer/);
  assert.match(launchScene, /requestAnimationFrame/);
  assert.match(launchScene, /data-dx-scene-renderer/);
  assert.match(launchScene, /data-dx-scene-status/);
  assert.match(sceneTypes, /export type DxSceneNode/);
  assert.match(sceneTypes, /DxSceneLighting/);
  assert.match(sceneTypes, /DxSceneRendererAdapter/);
  assert.match(scenePreset, /createDxLaunchScenePreset/);
  assert.match(scenePreset, /splineLikeLayers/);
  assert.match(sceneWebglRuntime, /mountDxWebGLScene/);
  assert.match(sceneWebglRuntime, /prefers-reduced-motion/);
  assert.match(sceneMetadata, /packageId: "3d\/launch-scene"/);
  assert.match(sceneMetadata, /upstreamPackages/);
  assert.match(sceneMetadata, /@react-three\/fiber/);
  assert.match(sceneMetadata, /@react-three\/drei/);
  assert.match(sceneReadme, /Web Preview-safe/);
  assert.match(sceneReadme, /Renderer handoff/);
  assert.match(sceneReadme, /License boundary/);
  assert.match(shell, /LaunchQueryCacheStatus/);
  assert.match(queryStatus, /dxQueryOptions/);
  assert.match(queryStatus, /useQuery/);
  assert.match(queryStatus, /staleTime: 60_000/);
  assert.match(shell, /IconLaunchStatus/);
  assert.match(iconStatus, /@\/components\/icons\/icon/);
  assert.match(iconStatus, /<Icon name="pack:check" \/>/);
  assert.match(shell, /LaunchInstantStatus/);
  assert.match(instantStatus, /@\/lib\/instant\/client/);
  assert.match(instantStatus, /NEXT_PUBLIC_INSTANT_APP_ID/);
  assert.match(instantStatus, /db\.useQuery\(\{ todos: \{\} \}\)/);
  assert.match(instantStatus, /db\.rooms\.usePresence/);
  assert.match(instantStatus, /db\.rooms\.useTypingIndicator/);
  assert.match(instantStatus, /uploadInstantLaunchFile/);
  assert.match(instantStatus, /createInstantLaunchWriteStream/);
  assert.match(instantStatus, /data-dx-instant-storage/);
  assert.match(instantStatus, /data-dx-instant-streams/);
  assert.match(instantStatus, /data-dx-instant-typing/);
  assert.match(instantStatus, /data-dx-instant-status/);
  assert.match(shell, /LaunchWasmInteropStatus/);
  assert.match(shell, /data-dx-component="launch-wasm-compute-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-workflow="local-compute-readiness"/);
  assert.match(shell, /data-dx-dashboard-card="local-compute"/);
  assert.match(shell, /data-dx-product-surface="launch-dashboard"/);
  assert.match(shell, /data-dx-package="wasm\/bindgen"/);
  assert.match(shell, /data-dx-package-role="backend-client,api,wasm"/);
  assert.match(wasmStatus, /@\/wasm\/bindgen\/react/);
  assert.match(wasmStatus, /@\/wasm\/bindgen\/loader/);
  assert.match(wasmStatus, /useWasmBindgenModule/);
  assert.match(wasmStatus, /WasmBindgenFactory/);
  assert.match(wasmStatus, /WasmBindgenInput/);
  assert.match(wasmStatus, /cacheKey: "dx-launch-wasm-status"/);
  assert.match(trpcExample, /data-dx-dashboard-card="typed-api-health"/);
  assert.match(trpcExample, /data-dx-dashboard-flow="typed-api-readiness"/);
  assert.match(trpcExample, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(trpcExample, /data-dx-trpc-action="prepare-launch-event"/);
  assert.match(wasmStatus, /enabled: Boolean\(importModule \|\| localReadinessEnabled\)/);
  assert.match(wasmStatus, /data-dx-component="wasm-bindgen-readiness-workflow"/);
  assert.match(wasmStatus, /data-dx-package="wasm\/bindgen"/);
  assert.match(wasmStatus, /data-dx-wasm-interaction="local-add-readiness"/);
  assert.match(wasmStatus, /data-dx-wasm-action="run-local-add"/);
  assert.match(wasmStatus, /data-dx-wasm-add-result=\{addResult \?\? "idle"\}/);
  assert.match(wasmStatus, /React\.createElement\("dx-icon"/);
  assert.match(wasmStatus, /name: "pack:wasm-bindgen"/);
  assert.match(wasmStatus, /module\?\.add/);
  assert.match(shell, /LaunchStatusMessage/);
  assert.match(shell, /<LaunchStatusMessage phase="beta" \/>/);
  assert.match(nextIntlStatus, /useTranslations\("Launch"\)/);
  assert.match(nextIntlStatus, /phaseStatus/);
  assert.match(nextIntlStatus, /data-launch-i18n-phase/);
  assert.match(shell, /LaunchZodValidationStatus/);
  assert.match(zodStatus, /@\/lib\/validation\/zod\/schemas/);
  assert.match(zodStatus, /@\/lib\/validation\/zod\/parse/);
  assert.match(zodStatus, /@\/lib\/validation\/zod\/json-schema/);
  assert.match(zodStatus, /dxLaunchSignupSchema/);
  assert.match(zodStatus, /validateDxInput/);
  assert.match(zodStatus, /dxToJsonSchema/);
  assert.match(zodStatus, /data-dx-zod-status/);
  assert.match(shell, /requiredLaunchEnv/);
  assert.match(shell, /appOwnedBoundaries/);
  assert.match(shell, /useTranslations/);
  assert.match(trpcExample, /"use client";/);
  assert.match(trpcExample, /import \* as React from "react";/);
  assert.match(trpcExample, /from "\.\/trpc-launch-contract";/);
  assert.match(trpcExample, /data-dx-package="api\/trpc"/);
  assert.match(trpcExample, /data-dx-component="trpc-launch-health-workflow"/);
  assert.match(trpcExample, /data-dx-dashboard-workflow="typed-api-health"/);
  assert.match(trpcExample, /data-dx-style-surface="theme-token"/);
  assert.match(trpcExample, /<dx-icon name="api:trpc" aria-hidden="true" \/>/);
  assert.match(trpcExample, /bg-card/);
  assert.match(trpcExample, /text-card-foreground/);
  assert.match(trpcExample, /data-trpc-workflow="template-visible"/);
  assert.match(trpcExample, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(trpcExample, /data-trpc-node-modules="not-required-for-workflow"/);
  assert.match(trpcContract, /trpc\.health\.queryOptions\(\)/);
  assert.match(trpcContract, /trpc\.launchEvent\.mutationOptions/);
  assert.match(trpcContract, /trpc\.health\.queryFilter\(\)/);
  assert.match(trpcExample, /setResult\(createLocalLaunchEvent\(nextSequence\)\)/);
  assert.match(trpcExample, /data-api-mutation=\{result\?\.status \?\? "idle"\}/);
  assert.doesNotMatch(trpcExample, /text-sky-|bg-black|border-neutral|text-neutral|bg-neutral/);
  assert.doesNotMatch(trpcExample, /from "@trpc\/client"/);
  assert.doesNotMatch(trpcExample, /from "@tanstack\/react-query"/);
  assert.doesNotMatch(trpcExample, /from "@\/lib\/trpc\/provider"/);
  assert.doesNotMatch(trpcExample, /data-trpc-proof=/);
  assert.match(cli, /NEXT_FAMILIAR_AUTH_STATUS_TSX/);
  assert.match(cli, /examples\/template\/auth-session-status\.tsx/);
  assert.match(cli, /"components\/template-app\/auth-session-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_AI_CHAT_STATUS_TSX/);
  assert.match(cli, /examples\/template\/ai-chat-status\.tsx/);
  assert.match(cli, /"components\/template-app\/ai-chat-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_DATA_STATUS_TSX/);
  assert.match(cli, /examples\/template\/data-status\.tsx/);
  assert.match(cli, /"components\/template-app\/data-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_PAYMENTS_STATUS_TSX/);
  assert.match(cli, /examples\/template\/payments-status\.tsx/);
  assert.match(cli, /"components\/template-app\/payments-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_DOCS_STATUS_TSX/);
  assert.match(cli, /examples\/template\/docs-status\.tsx/);
  assert.match(cli, /"components\/template-app\/docs-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_SCENE_TSX/);
  assert.match(cli, /examples\/template\/launch-scene\.tsx/);
  assert.match(cli, /"components\/scene\/launch-scene\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_TEMPLATE_DASHBOARD_NAV_TSX/);
  assert.match(cli, /examples\/template\/template-dashboard-nav\.tsx/);
  assert.match(cli, /"components\/template-app\/template-dashboard-nav\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_TEMPLATE_LEAD_FORM_TSX/);
  assert.match(cli, /examples\/template\/template-lead-form\.tsx/);
  assert.match(cli, /"components\/template-app\/template-lead-form\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_TYPES_TS/);
  assert.match(cli, /examples\/template\/scene\/types\.ts/);
  assert.match(cli, /"lib\/scene\/types\.ts"/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_PRESET_TS/);
  assert.match(cli, /examples\/template\/scene\/preset\.ts/);
  assert.match(cli, /"lib\/scene\/preset\.ts"/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_WEBGL_RUNTIME_TS/);
  assert.match(cli, /examples\/template\/scene\/webgl-runtime\.ts/);
  assert.match(cli, /"lib\/scene\/webgl-runtime\.ts"/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_METADATA_TS/);
  assert.match(cli, /examples\/template\/scene\/metadata\.ts/);
  assert.match(cli, /"lib\/scene\/metadata\.ts"/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_README_MD/);
  assert.match(cli, /examples\/template\/scene\/README\.md/);
  assert.match(cli, /"lib\/scene\/README\.md"/);
  assert.match(cli, /NEXT_FAMILIAR_QUERY_STATUS_TSX/);
  assert.match(cli, /examples\/template\/query-cache-status\.tsx/);
  assert.match(cli, /"components\/template-app\/query-cache-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_INSTANT_STATUS_TSX/);
  assert.match(cli, /examples\/template\/instantdb-status\.tsx/);
  assert.match(cli, /"components\/template-app\/instantdb-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_WASM_STATUS_TSX/);
  assert.match(cli, /examples\/template\/wasm-interop-status\.tsx/);
  assert.match(cli, /"components\/template-app\/wasm-interop-status\.tsx"/);
  assert.match(cli, /NEXT_FAMILIAR_ZOD_STATUS_TSX/);
  assert.match(cli, /examples\/template\/zod-validation-status\.tsx/);
  assert.match(cli, /"components\/template-app\/zod-validation-status\.tsx"/);
  assert.match(cli, /"app_router_entrypoint"/);
  assert.match(cli, /"www_template_entrypoint"/);
  assert.match(cli, /"route": "\/"/);
  assert.match(cli, /"route_aliases": \[\]/);
  assert.match(cli, /"materialized_file": "app\/page\.tsx"/);
  assert.doesNotMatch(cli, /"secondary_materialized_file": "app\/page\.tsx"/);
  assert.match(cli, /"contract_materialized_file": "components\/template-app\/template-route-contract\.ts"/);
  assert.match(cli, /"runtime_component_materialized_file": "components\/template-app\/template-console\.tsx"/);
  assert.match(cli, /"runtime_catalog_materialized_file": "server\/templateCatalog\.ts"/);
  assert.match(cli, /"component_materialized_file": "components\/template-app\/template-shell\.tsx"/);
  assert.match(cli, /"zed_template_handoff"/);
  assert.match(cli, /"schema": "dx\.zed\.template_handoff"/);
  assert.match(cli, /"handoff_kind": "app-router-template"/);
  assert.match(cli, /"owner": "dx-www"/);
  assert.match(cli, /let primary_route = DEFAULT_TEMPLATE_APP_ROUTE_SOURCES\[0\];/);
  assert.doesNotMatch(cli, /let secondary_route = DEFAULT_TEMPLATE_APP_ROUTE_SOURCES\[1\];/);
  assert.match(cli, /"entrypoint_file": primary_route\.materialized_file/);
  assert.match(cli, /"source_entrypoint_file": primary_route\.source_file/);
  assert.match(cli, /"entrypoint_role": primary_route\.role/);
  assert.doesNotMatch(cli, /"secondary_entrypoint_file": secondary_route\.materialized_file/);
  assert.doesNotMatch(cli, /"secondary_source_entrypoint_file": secondary_route\.source_file/);
  assert.doesNotMatch(cli, /"secondary_entrypoint_role": secondary_route\.role/);
  assert.match(cli, /"readiness_receipt": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE/);
  assert.match(cli, /"readiness_bundle": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE/);
  assert.match(cli, /"safe_source_checks": \[/);
  assert.match(cli, /"dx run --test \.\\\\benchmarks\\\\template-shell\.test\.ts"/);
  assert.match(cli, /"open_files": \[/);
  assert.match(cli, /"path": "components\/template-app\/template-route-contract\.ts"/);
  assert.match(cli, /"blocked_without_permission": \["dev-server", "full-build", "production-preview"\]/);
  assert.match(cli, /"no_execution": true/);
  assert.match(supabaseForge, /export function defaultSupabaseEnv\(\): DxSupabaseEnv/);
  assert.match(supabaseForge, /export function requiredEnv\(env: DxSupabaseEnv, key: string\): string/);
  assert.match(supabaseForge, /readSupabasePublicConfig,\s+type DxSupabaseEnv,/);
  assert.match(cli, /"runtime_evidence_review": "dx forge launch-runtime-evidence-review --project <path> --json"/);
  assert.match(cli, /"runtime_evidence_review_report": NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE/);
  assert.match(cli, /"launch_evidence_packet": "dx forge launch-evidence-packet --project <path> --json"/);
  assert.match(cli, /"launch_evidence_packet_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE/);
  assert.match(cli, /"launch_evidence_operator_index": "dx forge launch-evidence-operator-index --project <path> --json"/);
  assert.match(cli, /"launch_evidence_operator_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE/);
  assert.match(cli, /"launch_evidence_status_timeline": "dx forge launch-evidence-status-timeline --project <path> --json"/);
  assert.match(cli, /"launch_evidence_status_timeline_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE/);
  assert.match(cli, /"launch_evidence_handoff_digest": "dx forge launch-evidence-handoff-digest --project <path> --write"/);
  assert.match(cli, /"launch_evidence_handoff_digest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE/);
  assert.match(cli, /"launch_evidence_release_checklist": "dx forge launch-evidence-release-checklist --project <path> --write"/);
  assert.match(cli, /"launch_evidence_release_checklist_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE/);
  assert.match(cli, /"launch_evidence_share_manifest": "dx forge launch-evidence-share-manifest --project <path> --write"/);
  assert.match(cli, /"launch_evidence_share_manifest_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE/);
  assert.match(cli, /"launch_evidence_archive_index": "dx forge launch-evidence-archive-index --project <path> --write"/);
  assert.match(cli, /"launch_evidence_archive_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE/);
  assert.match(cli, /"launch_evidence_archive_receipt": "dx forge launch-evidence-archive-receipt --project <path> --write"/);
  assert.match(cli, /"launch_evidence_archive_receipt_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE/);
  assert.match(cli, /"launch_evidence_archive_ledger": "dx forge launch-evidence-archive-ledger --project <path> --write"/);
  assert.match(cli, /"launch_evidence_archive_ledger_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE/);
  assert.match(cli, /"launch_evidence_retention_policy": "dx forge launch-evidence-retention-policy --project <path> --write"/);
  assert.match(cli, /"launch_evidence_retention_policy_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE/);
  assert.match(cli, /"launch_evidence_retention_review": "dx forge launch-evidence-retention-review --project <path> --write"/);
  assert.match(cli, /"launch_evidence_retention_review_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE/);
  assert.match(cli, /"launch_evidence_release_seal": "dx forge launch-evidence-release-seal --project <path> --write"/);
  assert.match(cli, /"launch_evidence_release_seal_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE/);
  assert.match(cli, /"launch_evidence_operator_summary": "dx forge launch-evidence-operator-summary --project <path> --write"/);
  assert.match(cli, /"launch_evidence_operator_summary_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE/);
  assert.match(cli, /"launch_evidence_completion_ledger": "dx forge launch-evidence-completion-ledger --project <path> --write"/);
  assert.match(cli, /"launch_evidence_completion_ledger_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE/);
  assert.match(cli, /"launch_evidence_closure_memo": "dx forge launch-evidence-closure-memo --project <path> --write"/);
  assert.match(cli, /"launch_evidence_closure_memo_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE/);
  assert.match(cli, /"launch_evidence_final_brief": "dx forge launch-evidence-final-brief --project <path> --write"/);
  assert.match(cli, /"launch_evidence_final_brief_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE/);
  assert.match(cli, /"launch_evidence_operator_runbook": "dx forge launch-evidence-operator-runbook --project <path> --write"/);
  assert.match(cli, /"launch_evidence_operator_runbook_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE/);
  assert.match(cli, /"launch_evidence_handoff_capsule": "dx forge launch-evidence-handoff-capsule --project <path> --write"/);
  assert.match(cli, /"launch_evidence_handoff_capsule_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE/);
  assert.match(cli, /"launch_evidence_resumption_index": "dx forge launch-evidence-resumption-index --project <path> --write"/);
  assert.match(cli, /"launch_evidence_resumption_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE: &str\s*=\s*"\.dx\/forge\/runtime\/final-launch-evidence-review\.json"/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-packet\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-operator-index\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-status-timeline\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-handoff-digest\.md"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-release-checklist\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-share-manifest\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-archive-index\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-archive-receipt\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-archive-ledger\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-retention-policy\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-retention-review\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-release-seal\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-operator-summary\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-completion-ledger\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-closure-memo\.md"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-final-brief\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-operator-runbook\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-handoff-capsule\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-resumption-index\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-recovery-brief\.md"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-continuation-packet\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-operator-resume-card\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-restart-ledger\.json"/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-restart-closeout\.md"/);
  assert.match(cli, /"launch_runtime_evidence_review": \{/);
  assert.match(cli, /"schema": "dx\.launch\.runtime_evidence_review"/);
  assert.match(cli, /"launch_evidence_packet": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_packet"/);
  assert.match(cli, /"launch_evidence_operator_index": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_operator_index"/);
  assert.match(cli, /"launch_evidence_status_timeline": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_status_timeline"/);
  assert.match(cli, /"launch_evidence_handoff_digest": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_handoff_digest"/);
  assert.match(cli, /"launch_evidence_release_checklist": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_release_checklist"/);
  assert.match(cli, /"launch_evidence_share_manifest": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_share_manifest"/);
  assert.match(cli, /"launch_evidence_archive_index": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_archive_index"/);
  assert.match(cli, /"launch_evidence_archive_receipt": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_archive_receipt"/);
  assert.match(cli, /"launch_evidence_archive_ledger": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_archive_ledger"/);
  assert.match(cli, /"launch_evidence_retention_policy": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_retention_policy"/);
  assert.match(cli, /"launch_evidence_retention_review": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_retention_review"/);
  assert.match(cli, /"launch_evidence_release_seal": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_release_seal"/);
  assert.match(cli, /"launch_evidence_operator_summary": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_operator_summary"/);
  assert.match(cli, /"launch_evidence_completion_ledger": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_completion_ledger"/);
  assert.match(cli, /"launch_evidence_closure_memo": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_closure_memo"/);
  assert.match(cli, /"launch_evidence_final_brief": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_final_brief"/);
  assert.match(cli, /"launch_evidence_operator_runbook": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_operator_runbook"/);
  assert.match(cli, /"launch_evidence_handoff_capsule": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_handoff_capsule"/);
  assert.match(cli, /"launch_evidence_resumption_index": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_resumption_index"/);
  assert.match(cli, /"launch_evidence_recovery_brief": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_recovery_brief"/);
  assert.match(cli, /"launch_evidence_continuation_packet": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_continuation_packet"/);
  assert.match(cli, /"launch_evidence_operator_resume_card": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_operator_resume_card"/);
  assert.match(cli, /"launch_evidence_restart_ledger": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_restart_ledger"/);
  assert.match(cli, /"launch_evidence_restart_checklist": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_restart_checklist"/);
  assert.match(cli, /"launch_evidence_restart_brief": \{/);
  assert.match(cli, /"launch_evidence_restart_manifest": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_restart_brief"/);
  assert.match(cli, /"launch_evidence_restart_receipt": \{/);
  assert.match(cli, /"launch_evidence_restart_summary": \{/);
  assert.match(cli, /"launch_evidence_restart_snapshot": \{/);
  assert.match(cli, /"launch_evidence_restart_dispatch": \{/);
  assert.match(cli, /"launch_evidence_restart_closeout": \{/);
  assert.match(cli, /"launch_evidence_restart_signoff": \{/);
  assert.match(cli, /"launch_evidence_acceptance_index": \{/);
  assert.match(cli, /"launch_evidence_acceptance_digest": \{/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_restart_receipt"/);
  assert.match(cli, /mod launch_runtime_evidence_review;/);
  assert.match(cli, /mod launch_evidence_packet;/);
  assert.match(cli, /mod launch_evidence_operator_index;/);
  assert.match(cli, /mod launch_evidence_status_timeline;/);
  assert.match(cli, /mod launch_evidence_handoff_digest;/);
  assert.match(cli, /mod launch_evidence_release_checklist;/);
  assert.match(cli, /mod launch_evidence_share_manifest;/);
  assert.match(cli, /mod launch_evidence_archive_index;/);
  assert.match(cli, /mod launch_evidence_archive_receipt;/);
  assert.match(cli, /mod launch_evidence_archive_ledger;/);
  assert.match(cli, /mod launch_evidence_retention_policy;/);
  assert.match(cli, /mod launch_evidence_retention_review;/);
  assert.match(cli, /mod launch_evidence_release_seal;/);
  assert.match(cli, /mod launch_evidence_operator_summary;/);
  assert.match(cli, /mod launch_evidence_completion_ledger;/);
  assert.match(cli, /mod launch_evidence_closure_memo;/);
  assert.match(cli, /mod launch_evidence_final_brief;/);
  assert.match(cli, /mod launch_evidence_operator_runbook;/);
  assert.match(cli, /mod launch_evidence_handoff_capsule;/);
  assert.match(cli, /mod launch_evidence_resumption_index;/);
  assert.match(cli, /mod launch_evidence_recovery_brief;/);
  assert.match(cli, /mod launch_evidence_continuation_packet;/);
  assert.match(cli, /mod launch_evidence_operator_resume_card;/);
  assert.match(cli, /mod launch_evidence_restart_ledger;/);
  assert.match(cli, /mod launch_evidence_restart_checklist;/);
  assert.match(cli, /mod launch_evidence_restart_brief;/);
  assert.match(cli, /mod launch_evidence_restart_manifest;/);
  assert.match(cli, /mod launch_evidence_restart_receipt;/);
  assert.match(cli, /mod launch_evidence_restart_summary;/);
  assert.match(cli, /mod launch_evidence_restart_snapshot;/);
  assert.match(cli, /mod launch_evidence_restart_dispatch;/);
  assert.match(cli, /mod launch_evidence_restart_closeout;/);
  assert.match(cli, /mod launch_evidence_restart_signoff;/);
  assert.match(cli, /mod launch_evidence_acceptance_index;/);
  assert.match(cli, /mod launch_evidence_acceptance_digest;/);
  assert.match(cli, /"launch-runtime-evidence-review"/);
  assert.match(cli, /"launch-evidence-packet"/);
  assert.match(cli, /"launch-evidence-operator-index"/);
  assert.match(cli, /"launch-evidence-status-timeline"/);
  assert.match(cli, /"launch-evidence-handoff-digest"/);
  assert.match(cli, /"launch-evidence-release-checklist"/);
  assert.match(cli, /"launch-evidence-share-manifest"/);
  assert.match(cli, /"launch-evidence-archive-index"/);
  assert.match(cli, /"launch-evidence-archive-receipt"/);
  assert.match(cli, /"launch-evidence-archive-ledger"/);
  assert.match(cli, /"launch-evidence-retention-policy"/);
  assert.match(cli, /"launch-evidence-retention-review"/);
  assert.match(cli, /"launch-evidence-release-seal"/);
  assert.match(cli, /"launch-evidence-operator-summary"/);
  assert.match(cli, /"launch-evidence-completion-ledger"/);
  assert.match(cli, /"launch-evidence-closure-memo"/);
  assert.match(cli, /"launch-evidence-final-brief"/);
  assert.match(cli, /"launch-evidence-operator-runbook"/);
  assert.match(cli, /"launch-evidence-handoff-capsule"/);
  assert.match(cli, /"launch-evidence-resumption-index"/);
  assert.match(cli, /"launch-evidence-recovery-brief"/);
  assert.match(cli, /"launch-evidence-continuation-packet"/);
  assert.match(cli, /"launch-evidence-operator-resume-card"/);
  assert.match(cli, /"launch-evidence-restart-ledger"/);
  assert.match(cli, /"launch-evidence-restart-checklist"/);
  assert.match(cli, /"launch-evidence-restart-brief"/);
  assert.match(cli, /"launch-evidence-restart-manifest"/);
  assert.match(cli, /"launch-evidence-restart-receipt"/);
  assert.match(cli, /"launch-evidence-restart-summary"/);
  assert.match(cli, /"launch-evidence-restart-snapshot"/);
  assert.match(cli, /"launch-evidence-restart-dispatch"/);
  assert.match(cli, /"launch-evidence-restart-closeout"/);
  assert.match(cli, /"launch-evidence-restart-signoff"/);
  assert.match(cli, /"launch-evidence-acceptance-index"/);
  assert.match(cli, /"launch-evidence-acceptance-digest"/);
  assert.match(cli, /run_launch_runtime_evidence_review/);
  assert.match(cli, /run_launch_evidence_packet/);
  assert.match(cli, /run_launch_evidence_operator_index/);
  assert.match(cli, /run_launch_evidence_status_timeline/);
  assert.match(cli, /run_launch_evidence_handoff_digest/);
  assert.match(cli, /run_launch_evidence_release_checklist/);
  assert.match(cli, /run_launch_evidence_share_manifest/);
  assert.match(cli, /run_launch_evidence_archive_index/);
  assert.match(cli, /run_launch_evidence_archive_receipt/);
  assert.match(cli, /run_launch_evidence_archive_ledger/);
  assert.match(cli, /run_launch_evidence_retention_policy/);
  assert.match(cli, /run_launch_evidence_retention_review/);
  assert.match(cli, /run_launch_evidence_release_seal/);
  assert.match(cli, /run_launch_evidence_operator_summary/);
  assert.match(cli, /run_launch_evidence_completion_ledger/);
  assert.match(cli, /run_launch_evidence_closure_memo/);
  assert.match(cli, /run_launch_evidence_final_brief/);
  assert.match(cli, /run_launch_evidence_operator_runbook/);
  assert.match(cli, /run_launch_evidence_handoff_capsule/);
  assert.match(cli, /run_launch_evidence_resumption_index/);
  assert.match(cli, /run_launch_evidence_recovery_brief/);
  assert.match(cli, /run_launch_evidence_continuation_packet/);
  assert.match(cli, /run_launch_evidence_operator_resume_card/);
  assert.match(cli, /run_launch_evidence_restart_ledger/);
  assert.match(cli, /run_launch_evidence_restart_checklist/);
  assert.match(cli, /run_launch_evidence_restart_brief/);
  assert.match(cli, /run_launch_evidence_restart_manifest/);
  assert.match(cli, /run_launch_evidence_restart_receipt/);
  assert.match(cli, /run_launch_evidence_restart_summary/);
  assert.match(cli, /run_launch_evidence_restart_snapshot/);
  assert.match(cli, /run_launch_evidence_restart_dispatch/);
  assert.match(cli, /run_launch_evidence_restart_closeout/);
  assert.match(cli, /run_launch_evidence_restart_signoff/);
  assert.match(cli, /run_launch_evidence_acceptance_index/);
  assert.match(cli, /run_launch_evidence_acceptance_digest/);
  assert.match(runtimeEvidenceReview, /REPORT_SCHEMA: &str = "dx\.forge\.launch_runtime_evidence_review"/);
  assert.match(runtimeEvidenceReview, /FINAL_RECEIPT_SCHEMA: &str = "dx\.launch\.runtime_evidence_finalization_receipt"/);
  assert.match(runtimeEvidenceReview, /build_launch_runtime_evidence_review_report/);
  assert.match(runtimeEvidenceReview, /source_hash_matches/);
  assert.match(runtimeEvidenceReview, /runtime-evidence-finalized/);
  assert.match(runtimeEvidenceReview, /finalization-receipt-hashes-match/);
  assert.match(launchEvidencePacket, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_packet"/);
  assert.match(launchEvidencePacket, /PACKET_SCHEMA: &str = "dx\.launch\.evidence_packet"/);
  assert.match(launchEvidencePacket, /build_launch_evidence_packet_report/);
  assert.match(launchEvidencePacket, /packet_integrity/);
  assert.match(launchEvidencePacket, /final-evidence-review-report/);
  assert.match(launchEvidencePacket, /fresh-final-evidence-review/);
  assert.match(launchEvidencePacket, /packet-integrity/);
  assert.match(launchEvidenceOperatorIndex, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_operator_index"/);
  assert.match(launchEvidenceOperatorIndex, /INDEX_SCHEMA: &str = "dx\.launch\.evidence_operator_index"/);
  assert.match(launchEvidenceOperatorIndex, /build_launch_evidence_operator_index_report/);
  assert.match(launchEvidenceOperatorIndex, /stale_step_hints/);
  assert.match(launchEvidenceOperatorIndex, /reads_runtime_artifact_contents/);
  assert.match(launchEvidenceOperatorIndex, /no-runtime-content-read/);
  assert.match(launchEvidenceOperatorIndex, /packet-evidence-indexed/);
  assert.match(launchEvidenceStatusTimeline, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_status_timeline"/);
  assert.match(launchEvidenceStatusTimeline, /TIMELINE_SCHEMA: &str = "dx\.launch\.evidence_status_timeline"/);
  assert.match(launchEvidenceStatusTimeline, /build_launch_evidence_status_timeline_report/);
  assert.match(launchEvidenceStatusTimeline, /latest_completed_step/);
  assert.match(launchEvidenceStatusTimeline, /next_blocked_step/);
  assert.match(launchEvidenceStatusTimeline, /operator_index_not_older_than_packet/);
  assert.match(launchEvidenceStatusTimeline, /no-runtime-content-read/);
  assert.match(launchEvidenceHandoffDigest, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_handoff_digest"/);
  assert.match(launchEvidenceHandoffDigest, /DIGEST_SCHEMA: &str = "dx\.launch\.evidence_handoff_digest"/);
  assert.match(launchEvidenceHandoffDigest, /build_launch_evidence_handoff_digest_report/);
  assert.match(launchEvidenceHandoffDigest, /zed_openable/);
  assert.match(launchEvidenceHandoffDigest, /digest_not_older_than_inputs/);
  assert.match(launchEvidenceHandoffDigest, /launch-evidence-release-checklist/);
  assert.match(launchEvidenceHandoffDigest, /no-runtime-content-read/);
  assert.match(launchEvidenceReleaseChecklist, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_release_checklist"/);
  assert.match(launchEvidenceReleaseChecklist, /CHECKLIST_SCHEMA: &str = "dx\.launch\.evidence_release_checklist"/);
  assert.match(launchEvidenceReleaseChecklist, /build_launch_evidence_release_checklist_report/);
  assert.match(launchEvidenceReleaseChecklist, /release_ready/);
  assert.match(launchEvidenceReleaseChecklist, /checklist_not_older_than_inputs/);
  assert.match(launchEvidenceReleaseChecklist, /no-runtime-content-read/);
  assert.match(launchEvidenceShareManifest, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_share_manifest"/);
  assert.match(launchEvidenceShareManifest, /MANIFEST_SCHEMA: &str = "dx\.launch\.evidence_share_manifest"/);
  assert.match(launchEvidenceShareManifest, /build_launch_evidence_share_manifest_report/);
  assert.match(launchEvidenceShareManifest, /export_target/);
  assert.match(launchEvidenceShareManifest, /manifest_not_older_than_release_artifacts/);
  assert.match(launchEvidenceShareManifest, /no-runtime-content-read/);
  assert.match(launchEvidenceArchiveIndex, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_archive_index"/);
  assert.match(launchEvidenceArchiveIndex, /ARCHIVE_SCHEMA: &str = "dx\.launch\.evidence_archive_index"/);
  assert.match(launchEvidenceArchiveIndex, /build_launch_evidence_archive_index_report/);
  assert.match(launchEvidenceArchiveIndex, /archive_target/);
  assert.match(launchEvidenceArchiveIndex, /archive_not_older_than_share_manifest/);
  assert.match(launchEvidenceArchiveIndex, /launch-evidence-archive-receipt/);
  assert.match(launchEvidenceArchiveIndex, /no-runtime-content-read/);
  assert.match(launchEvidenceArchiveReceipt, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_archive_receipt"/);
  assert.match(launchEvidenceArchiveReceipt, /RECEIPT_SCHEMA: &str = "dx\.launch\.evidence_archive_receipt"/);
  assert.match(launchEvidenceArchiveReceipt, /build_launch_evidence_archive_receipt_report/);
  assert.match(launchEvidenceArchiveReceipt, /operator_handoff_target/);
  assert.match(launchEvidenceArchiveReceipt, /receipt_not_older_than_archive_index/);
  assert.match(launchEvidenceArchiveReceipt, /launch-evidence-archive-ledger/);
  assert.match(launchEvidenceArchiveReceipt, /no-runtime-content-read/);
  assert.match(launchEvidenceArchiveLedger, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_archive_ledger"/);
  assert.match(launchEvidenceArchiveLedger, /LEDGER_SCHEMA: &str = "dx\.launch\.evidence_archive_ledger"/);
  assert.match(launchEvidenceArchiveLedger, /build_launch_evidence_archive_ledger_report/);
  assert.match(launchEvidenceArchiveLedger, /ledger_target/);
  assert.match(launchEvidenceArchiveLedger, /ledger_not_older_than_archive_receipt/);
  assert.match(launchEvidenceArchiveLedger, /launch-evidence-retention-policy/);
  assert.match(launchEvidenceArchiveLedger, /no-runtime-content-read/);
  assert.match(launchEvidenceRetentionPolicy, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_retention_policy"/);
  assert.match(launchEvidenceRetentionPolicy, /POLICY_SCHEMA: &str = "dx\.launch\.evidence_retention_policy"/);
  assert.match(launchEvidenceRetentionPolicy, /build_launch_evidence_retention_policy_report/);
  assert.match(launchEvidenceRetentionPolicy, /retention_actions/);
  assert.match(launchEvidenceRetentionPolicy, /policy_not_older_than_archive_ledger/);
  assert.match(launchEvidenceRetentionPolicy, /launch-evidence-retention-review/);
  assert.match(launchEvidenceRetentionPolicy, /no-runtime-content-read/);
  assert.match(launchEvidenceRetentionReview, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_retention_review"/);
  assert.match(launchEvidenceRetentionReview, /REVIEW_SCHEMA: &str = "dx\.launch\.evidence_retention_review"/);
  assert.match(launchEvidenceRetentionReview, /build_launch_evidence_retention_review_report/);
  assert.match(launchEvidenceRetentionReview, /review_target/);
  assert.match(launchEvidenceRetentionReview, /review_not_older_than_retention_policy/);
  assert.match(launchEvidenceRetentionReview, /launch-evidence-release-seal/);
  assert.match(launchEvidenceRetentionReview, /no-runtime-content-read/);
  assert.match(launchEvidenceReleaseSeal, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_release_seal"/);
  assert.match(launchEvidenceReleaseSeal, /SEAL_SCHEMA: &str = "dx\.launch\.evidence_release_seal"/);
  assert.match(launchEvidenceReleaseSeal, /build_launch_evidence_release_seal_report/);
  assert.match(launchEvidenceReleaseSeal, /seal_target/);
  assert.match(launchEvidenceReleaseSeal, /seal_not_older_than_retention_review/);
  assert.match(launchEvidenceReleaseSeal, /launch-evidence-operator-summary/);
  assert.match(launchEvidenceReleaseSeal, /no-runtime-content-read/);
  assert.match(launchEvidenceOperatorSummary, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_operator_summary"/);
  assert.match(launchEvidenceOperatorSummary, /SUMMARY_SCHEMA: &str = "dx\.launch\.evidence_operator_summary"/);
  assert.match(launchEvidenceOperatorSummary, /build_launch_evidence_operator_summary_report/);
  assert.match(launchEvidenceOperatorSummary, /summary_target/);
  assert.match(launchEvidenceOperatorSummary, /summary_not_older_than_release_seal/);
  assert.match(launchEvidenceOperatorSummary, /launch-evidence-completion-ledger/);
  assert.match(launchEvidenceOperatorSummary, /no-runtime-content-read/);
  assert.match(launchEvidenceCompletionLedger, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_completion_ledger"/);
  assert.match(launchEvidenceCompletionLedger, /LEDGER_SCHEMA: &str = "dx\.launch\.evidence_completion_ledger"/);
  assert.match(launchEvidenceCompletionLedger, /build_launch_evidence_completion_ledger_report/);
  assert.match(launchEvidenceCompletionLedger, /completion_target/);
  assert.match(launchEvidenceCompletionLedger, /ledger_not_older_than_operator_summary/);
  assert.match(launchEvidenceCompletionLedger, /launch-evidence-closure-memo/);
  assert.match(launchEvidenceCompletionLedger, /no-runtime-content-read/);
  assert.match(launchEvidenceClosureMemo, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_closure_memo"/);
  assert.match(launchEvidenceClosureMemo, /MEMO_SCHEMA: &str = "dx\.launch\.evidence_closure_memo"/);
  assert.match(launchEvidenceClosureMemo, /build_launch_evidence_closure_memo_report/);
  assert.match(launchEvidenceClosureMemo, /memo_target/);
  assert.match(launchEvidenceClosureMemo, /memo_not_older_than_completion_ledger/);
  assert.match(launchEvidenceClosureMemo, /zed-openable-markdown/);
  assert.match(launchEvidenceClosureMemo, /launch-evidence-final-brief/);
  assert.match(launchEvidenceClosureMemo, /no-runtime-content-read/);
  assert.match(launchEvidenceFinalBrief, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_final_brief"/);
  assert.match(launchEvidenceFinalBrief, /BRIEF_SCHEMA: &str = "dx\.launch\.evidence_final_brief"/);
  assert.match(launchEvidenceFinalBrief, /build_launch_evidence_final_brief_report/);
  assert.match(launchEvidenceFinalBrief, /brief_target/);
  assert.match(launchEvidenceFinalBrief, /dx-cli-zed-launch-closeout-pointer/);
  assert.match(launchEvidenceFinalBrief, /brief_not_older_than_closure_memo/);
  assert.match(launchEvidenceFinalBrief, /launch-evidence-operator-runbook/);
  assert.match(launchEvidenceFinalBrief, /no-runtime-content-read/);
  assert.match(launchEvidenceOperatorRunbook, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_operator_runbook"/);
  assert.match(launchEvidenceOperatorRunbook, /RUNBOOK_SCHEMA: &str = "dx\.launch\.evidence_operator_runbook"/);
  assert.match(launchEvidenceOperatorRunbook, /build_launch_evidence_operator_runbook_report/);
  assert.match(launchEvidenceOperatorRunbook, /runbook_target/);
  assert.match(launchEvidenceOperatorRunbook, /restartable-dx-worker-checklist/);
  assert.match(launchEvidenceOperatorRunbook, /runbook_not_older_than_final_brief/);
  assert.match(launchEvidenceOperatorRunbook, /launch-evidence-handoff-capsule/);
  assert.match(launchEvidenceOperatorRunbook, /no-runtime-content-read/);
  assert.match(launchEvidenceHandoffCapsule, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_handoff_capsule"/);
  assert.match(launchEvidenceHandoffCapsule, /CAPSULE_SCHEMA: &str = "dx\.launch\.evidence_handoff_capsule"/);
  assert.match(launchEvidenceHandoffCapsule, /build_launch_evidence_handoff_capsule_report/);
  assert.match(launchEvidenceHandoffCapsule, /capsule_target/);
  assert.match(launchEvidenceHandoffCapsule, /dx-cli-zed-restart-artifact/);
  assert.match(launchEvidenceHandoffCapsule, /capsule_not_older_than_operator_runbook/);
  assert.match(launchEvidenceHandoffCapsule, /launch-evidence-resumption-index/);
  assert.match(launchEvidenceHandoffCapsule, /no-runtime-content-read/);
  assert.match(launchEvidenceResumptionIndex, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_resumption_index"/);
  assert.match(launchEvidenceResumptionIndex, /INDEX_SCHEMA: &str = "dx\.launch\.evidence_resumption_index"/);
  assert.match(launchEvidenceResumptionIndex, /build_launch_evidence_resumption_index_report/);
  assert.match(launchEvidenceResumptionIndex, /resumption_target/);
  assert.match(launchEvidenceResumptionIndex, /ordered-dx-cli-zed-restart-lanes/);
  assert.match(launchEvidenceResumptionIndex, /source-only/);
  assert.match(launchEvidenceResumptionIndex, /runtime-approved/);
  assert.match(launchEvidenceResumptionIndex, /release-closeout/);
  assert.match(launchEvidenceResumptionIndex, /index_not_older_than_handoff_capsule/);
  assert.match(launchEvidenceResumptionIndex, /launch-evidence-recovery-brief/);
  assert.match(launchEvidenceResumptionIndex, /no-runtime-content-read/);
  assert.match(launchEvidenceRecoveryBrief, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_recovery_brief"/);
  assert.match(launchEvidenceRecoveryBrief, /BRIEF_SCHEMA: &str = "dx\.launch\.evidence_recovery_brief"/);
  assert.match(launchEvidenceRecoveryBrief, /build_launch_evidence_recovery_brief_report/);
  assert.match(launchEvidenceRecoveryBrief, /recovery_target/);
  assert.match(launchEvidenceRecoveryBrief, /human-readable-dx-worker-restart-brief/);
  assert.match(launchEvidenceRecoveryBrief, /brief_not_older_than_resumption_index/);
  assert.match(launchEvidenceRecoveryBrief, /zed-openable-markdown/);
  assert.match(launchEvidenceRecoveryBrief, /launch-evidence-continuation-packet/);
  assert.match(launchEvidenceRecoveryBrief, /no-runtime-content-read/);
  assert.match(launchEvidenceContinuationPacket, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_continuation_packet"/);
  assert.match(launchEvidenceContinuationPacket, /PACKET_SCHEMA: &str = "dx\.launch\.evidence_continuation_packet"/);
  assert.match(launchEvidenceContinuationPacket, /build_launch_evidence_continuation_packet_report/);
  assert.match(launchEvidenceContinuationPacket, /continuation_target/);
  assert.match(launchEvidenceContinuationPacket, /dx-cli-zed-continuation-packet/);
  assert.match(launchEvidenceContinuationPacket, /packet_not_older_than_recovery_brief/);
  assert.match(launchEvidenceContinuationPacket, /launch-evidence-operator-resume-card/);
  assert.match(launchEvidenceContinuationPacket, /no-runtime-content-read/);
  assert.match(launchEvidenceOperatorResumeCard, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_operator_resume_card"/);
  assert.match(launchEvidenceOperatorResumeCard, /CARD_SCHEMA: &str = "dx\.launch\.evidence_operator_resume_card"/);
  assert.match(launchEvidenceOperatorResumeCard, /build_launch_evidence_operator_resume_card_report/);
  assert.match(launchEvidenceOperatorResumeCard, /resume_target/);
  assert.match(launchEvidenceOperatorResumeCard, /terminal-first-dx-resume-card/);
  assert.match(launchEvidenceOperatorResumeCard, /card_not_older_than_continuation_packet/);
  assert.match(launchEvidenceOperatorResumeCard, /launch-evidence-restart-ledger/);
  assert.match(launchEvidenceOperatorResumeCard, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartLedger, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_ledger"/);
  assert.match(launchEvidenceRestartLedger, /LEDGER_SCHEMA: &str = "dx\.launch\.evidence_restart_ledger"/);
  assert.match(launchEvidenceRestartLedger, /build_launch_evidence_restart_ledger_report/);
  assert.match(launchEvidenceRestartLedger, /ledger_target/);
  assert.match(launchEvidenceRestartLedger, /durable-dx-restart-ledger/);
  assert.match(launchEvidenceRestartLedger, /ledger_not_older_than_operator_resume_card/);
  assert.match(launchEvidenceRestartLedger, /launch-evidence-restart-checklist/);
  assert.match(launchEvidenceRestartLedger, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartChecklist, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_checklist"/);
  assert.match(launchEvidenceRestartChecklist, /CHECKLIST_SCHEMA: &str = "dx\.launch\.evidence_restart_checklist"/);
  assert.match(launchEvidenceRestartChecklist, /build_launch_evidence_restart_checklist_report/);
  assert.match(launchEvidenceRestartChecklist, /checklist_target/);
  assert.match(launchEvidenceRestartChecklist, /dx-cli-zed-restart-next-actions/);
  assert.match(launchEvidenceRestartChecklist, /source-only/);
  assert.match(launchEvidenceRestartChecklist, /runtime-approved/);
  assert.match(launchEvidenceRestartChecklist, /release-closeout/);
  assert.match(launchEvidenceRestartChecklist, /checklist_not_older_than_restart_ledger/);
  assert.match(launchEvidenceRestartChecklist, /launch-evidence-restart-brief/);
  assert.match(launchEvidenceRestartChecklist, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartBrief, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_brief"/);
  assert.match(launchEvidenceRestartBrief, /BRIEF_SCHEMA: &str = "dx\.launch\.evidence_restart_brief"/);
  assert.match(launchEvidenceRestartBrief, /build_launch_evidence_restart_brief_report/);
  assert.match(launchEvidenceRestartBrief, /brief_target/);
  assert.match(launchEvidenceRestartBrief, /zed-openable-dx-restart-brief/);
  assert.match(launchEvidenceRestartBrief, /format: "markdown"/);
  assert.match(launchEvidenceRestartBrief, /brief_not_older_than_restart_checklist/);
  assert.match(launchEvidenceRestartBrief, /launch-evidence-restart-manifest/);
  assert.match(launchEvidenceRestartBrief, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartManifest, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_manifest"/);
  assert.match(launchEvidenceRestartManifest, /MANIFEST_SCHEMA: &str = "dx\.launch\.evidence_restart_manifest"/);
  assert.match(launchEvidenceRestartManifest, /build_launch_evidence_restart_manifest_report/);
  assert.match(launchEvidenceRestartManifest, /manifest_target/);
  assert.match(launchEvidenceRestartManifest, /dx-cli-zed-indexable-restart-manifest/);
  assert.match(launchEvidenceRestartManifest, /manifest_not_older_than_restart_brief/);
  assert.match(launchEvidenceRestartManifest, /launch-evidence-restart-receipt/);
  assert.match(launchEvidenceRestartManifest, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartReceipt, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_receipt"/);
  assert.match(launchEvidenceRestartReceipt, /RECEIPT_SCHEMA: &str = "dx\.launch\.evidence_restart_receipt"/);
  assert.match(launchEvidenceRestartReceipt, /build_launch_evidence_restart_receipt_report/);
  assert.match(launchEvidenceRestartReceipt, /receipt_target/);
  assert.match(launchEvidenceRestartReceipt, /latest-resumable-dx-zed-handoff/);
  assert.match(launchEvidenceRestartReceipt, /receipt_not_older_than_restart_manifest/);
  assert.match(launchEvidenceRestartReceipt, /launch-evidence-restart-summary/);
  assert.match(launchEvidenceRestartReceipt, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartSummary, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_summary"/);
  assert.match(launchEvidenceRestartSummary, /SUMMARY_SCHEMA: &str = "dx\.launch\.evidence_restart_summary"/);
  assert.match(launchEvidenceRestartSummary, /build_launch_evidence_restart_summary_report/);
  assert.match(launchEvidenceRestartSummary, /summary_target/);
  assert.match(launchEvidenceRestartSummary, /terminal-friendly-dx-zed-restart-handoff/);
  assert.match(launchEvidenceRestartSummary, /summary_not_older_than_restart_receipt/);
  assert.match(launchEvidenceRestartSummary, /restart-receipt/);
  assert.match(launchEvidenceRestartSummary, /launch-evidence-restart-snapshot/);
  assert.match(launchEvidenceRestartSummary, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartSnapshot, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_snapshot"/);
  assert.match(launchEvidenceRestartSnapshot, /SNAPSHOT_SCHEMA: &str = "dx\.launch\.evidence_restart_snapshot"/);
  assert.match(launchEvidenceRestartSnapshot, /build_launch_evidence_restart_snapshot_report/);
  assert.match(launchEvidenceRestartSnapshot, /snapshot_target/);
  assert.match(launchEvidenceRestartSnapshot, /latest-openable-dx-zed-restart-file/);
  assert.match(launchEvidenceRestartSnapshot, /snapshot_not_older_than_restart_summary/);
  assert.match(launchEvidenceRestartSnapshot, /restart-summary/);
  assert.match(launchEvidenceRestartSnapshot, /launch-evidence-restart-dispatch/);
  assert.match(launchEvidenceRestartSnapshot, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartDispatch, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_dispatch"/);
  assert.match(launchEvidenceRestartDispatch, /DISPATCH_SCHEMA: &str = "dx\.launch\.evidence_restart_dispatch"/);
  assert.match(launchEvidenceRestartDispatch, /build_launch_evidence_restart_dispatch_report/);
  assert.match(launchEvidenceRestartDispatch, /dispatch_target/);
  assert.match(launchEvidenceRestartDispatch, /one-command-next-worker-dispatch-card/);
  assert.match(launchEvidenceRestartDispatch, /dispatch_not_older_than_restart_snapshot/);
  assert.match(launchEvidenceRestartDispatch, /restart-snapshot/);
  assert.match(launchEvidenceRestartDispatch, /launch-evidence-restart-closeout/);
  assert.match(launchEvidenceRestartDispatch, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartCloseout, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_closeout"/);
  assert.match(launchEvidenceRestartCloseout, /CLOSEOUT_SCHEMA: &str = "dx\.launch\.evidence_restart_closeout"/);
  assert.match(launchEvidenceRestartCloseout, /build_launch_evidence_restart_closeout_report/);
  assert.match(launchEvidenceRestartCloseout, /closeout_target/);
  assert.match(launchEvidenceRestartCloseout, /final-friday-essencefromexistence-closeout-actions/);
  assert.match(launchEvidenceRestartCloseout, /closeout_not_older_than_restart_dispatch/);
  assert.match(launchEvidenceRestartCloseout, /restart-dispatch/);
  assert.match(launchEvidenceRestartCloseout, /format: "markdown"/);
  assert.match(launchEvidenceRestartCloseout, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartCloseout, /launch-evidence-restart-signoff/);
  assert.match(launchEvidenceRestartSignoff, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_signoff"/);
  assert.match(launchEvidenceRestartSignoff, /SIGNOFF_SCHEMA: &str = "dx\.launch\.evidence_restart_signoff"/);
  assert.match(launchEvidenceRestartSignoff, /build_launch_evidence_restart_signoff_report/);
  assert.match(launchEvidenceRestartSignoff, /signoff_target/);
  assert.match(launchEvidenceRestartSignoff, /friday-essencefromexistence-acceptance-receipt/);
  assert.match(launchEvidenceRestartSignoff, /signoff_not_older_than_restart_closeout/);
  assert.match(launchEvidenceRestartSignoff, /restart-closeout/);
  assert.match(launchEvidenceRestartSignoff, /acceptance_status: "reviewable"/);
  assert.match(launchEvidenceRestartSignoff, /no-runtime-content-read/);
  assert.match(launchEvidenceRestartSignoff, /launch-evidence-acceptance-index/);
  assert.match(launchEvidenceAcceptanceIndex, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_acceptance_index"/);
  assert.match(launchEvidenceAcceptanceIndex, /INDEX_SCHEMA: &str = "dx\.launch\.evidence_acceptance_index"/);
  assert.match(launchEvidenceAcceptanceIndex, /build_launch_evidence_acceptance_index_report/);
  assert.match(launchEvidenceAcceptanceIndex, /acceptance_target/);
  assert.match(launchEvidenceAcceptanceIndex, /friday-final-handoff-index/);
  assert.match(launchEvidenceAcceptanceIndex, /index_not_older_than_restart_signoff/);
  assert.match(launchEvidenceAcceptanceIndex, /restart-signoff/);
  assert.match(launchEvidenceAcceptanceIndex, /restart-closeout/);
  assert.match(launchEvidenceAcceptanceIndex, /restart-dispatch/);
  assert.match(launchEvidenceAcceptanceIndex, /restart-snapshot/);
  assert.match(launchEvidenceAcceptanceIndex, /format: "markdown"/);
  assert.match(launchEvidenceAcceptanceIndex, /no-runtime-content-read/);
  assert.match(launchEvidenceAcceptanceIndex, /launch-evidence-acceptance-digest/);
  assert.match(launchEvidenceAcceptanceDigest, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_acceptance_digest"/);
  assert.match(launchEvidenceAcceptanceDigest, /DIGEST_SCHEMA: &str = "dx\.launch\.evidence_acceptance_digest"/);
  assert.match(launchEvidenceAcceptanceDigest, /build_launch_evidence_acceptance_digest_report/);
  assert.match(launchEvidenceAcceptanceDigest, /digest_target/);
  assert.match(launchEvidenceAcceptanceDigest, /friday-terminal-final-status-line/);
  assert.match(launchEvidenceAcceptanceDigest, /digest_not_older_than_acceptance_index/);
  assert.match(launchEvidenceAcceptanceDigest, /terminal-first-final-status/);
  assert.match(launchEvidenceAcceptanceDigest, /DX launch acceptance digest:/);
  assert.match(launchEvidenceAcceptanceDigest, /no-runtime-content-read/);
  assert.match(launchEvidenceAcceptanceDigest, /launch-evidence-friday-baton/);
  assert.match(launchEvidenceFridayBaton, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_friday_baton"/);
  assert.match(launchEvidenceFridayBaton, /BATON_SCHEMA: &str = "dx\.launch\.evidence_friday_baton"/);
  assert.match(launchEvidenceFridayBaton, /build_launch_evidence_friday_baton_report/);
  assert.match(launchEvidenceFridayBaton, /baton_target/);
  assert.match(launchEvidenceFridayBaton, /friday-orchestrator-final-handoff/);
  assert.match(launchEvidenceFridayBaton, /baton_not_older_than_acceptance_digest/);
  assert.match(launchEvidenceFridayBaton, /acceptance-digest/);
  assert.match(launchEvidenceFridayBaton, /acceptance-index/);
  assert.match(launchEvidenceFridayBaton, /restart-signoff/);
  assert.match(launchEvidenceFridayBaton, /launch-verification-lane/);
  assert.match(launchEvidenceFridayBaton, /format: "markdown"/);
  assert.match(launchEvidenceFridayBaton, /no-runtime-content-read/);
  assert.match(routeContract, /launchEvidenceClosureMemo/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-closure-memo\.md"/);
  assert.match(routeContract, /launchEvidenceFinalBrief/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-final-brief\.json"/);
  assert.match(routeContract, /launchEvidenceOperatorRunbook/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-operator-runbook\.json"/);
  assert.match(routeContract, /launchEvidenceHandoffCapsule/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-handoff-capsule\.json"/);
  assert.match(routeContract, /launchEvidenceResumptionIndex/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-resumption-index\.json"/);
  assert.match(routeContract, /launchEvidenceRecoveryBrief/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-recovery-brief\.md"/);
  assert.match(routeContract, /launchEvidenceContinuationPacket/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-continuation-packet\.json"/);
  assert.match(routeContract, /launchEvidenceOperatorResumeCard/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-operator-resume-card\.json"/);
  assert.match(routeContract, /launchEvidenceRestartLedger/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-ledger\.json"/);
  assert.match(routeContract, /launchEvidenceRestartChecklist/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-checklist\.json"/);
  assert.match(routeContract, /launchEvidenceRestartBrief/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-brief\.md"/);
  assert.match(routeContract, /launchEvidenceRestartManifest/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-manifest\.json"/);
  assert.match(routeContract, /launchEvidenceRestartReceipt/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-receipt\.json"/);
  assert.match(routeContract, /launchEvidenceRestartSummary/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-summary\.json"/);
  assert.match(routeContract, /launchEvidenceRestartSnapshot/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-snapshot\.json"/);
  assert.match(routeContract, /launchEvidenceRestartDispatch/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-dispatch\.json"/);
  assert.match(routeContract, /launchEvidenceRestartCloseout/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-closeout\.md"/);
  assert.match(routeContract, /launchEvidenceRestartSignoff/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-signoff\.json"/);
  assert.match(routeContract, /restartSignoffFile: templateRouteContract\.launchEvidenceRestartSignoff\.file/);
  assert.match(routeContract, /launchEvidenceAcceptanceIndex/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-acceptance-index\.md"/);
  assert.match(routeContract, /acceptanceIndexFile: templateRouteContract\.launchEvidenceAcceptanceIndex\.file/);
  assert.match(routeContract, /launchEvidenceAcceptanceDigest/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-acceptance-digest\.json"/);
  assert.match(routeContract, /acceptanceDigestFile: templateRouteContract\.launchEvidenceAcceptanceDigest\.file/);
  assert.match(routeContract, /launchEvidenceFridayBaton/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-friday-baton\.md"/);
  assert.match(routeContract, /fridayBatonFile: templateRouteContract\.launchEvidenceFridayBaton\.file/);
  assert.match(routeContract, /launchEvidenceCompletionLedger/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-completion-ledger\.json"/);
  assert.match(routeContract, /launchEvidenceOperatorSummary/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-operator-summary\.json"/);
  assert.match(routeContract, /launchEvidenceReleaseSeal/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-release-seal\.json"/);
  assert.match(routeContract, /launchEvidenceRetentionReview/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-retention-review\.json"/);
  assert.match(routeContract, /launchEvidenceRetentionPolicy/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-retention-policy\.json"/);
  assert.match(routeContract, /launchEvidenceArchiveLedger/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-archive-ledger\.json"/);
  assert.match(routeContract, /launchEvidenceArchiveReceipt/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-archive-receipt\.json"/);
  assert.match(routeContract, /launchEvidenceArchiveIndex/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-archive-index\.json"/);
  assert.match(routeContract, /launchEvidenceShareManifest/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-share-manifest\.json"/);
  assert.match(routeContract, /launchEvidenceReleaseChecklist/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-release-checklist\.json"/);
  assert.match(routeContract, /launchEvidenceHandoffDigest/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-handoff-digest\.md"/);
  assert.match(routeContract, /launchEvidenceStatusTimeline/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-status-timeline\.json"/);
  assert.match(routeContract, /launchEvidenceOperatorIndex/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-operator-index\.json"/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE/);
  assert.match(cli, /"\.dx\/forge\/release\/launch-evidence-operator-index\.json"/);
  assert.match(cli, /"launch_evidence_operator_index": "dx forge launch-evidence-operator-index --project <path> --json"/);
  assert.match(cli, /"launch_evidence_operator_index_report": NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE/);
  assert.match(cli, /"schema": "dx\.launch\.evidence_operator_index"/);
  assert.match(cli, /mod launch_evidence_operator_index;/);
  assert.match(cli, /"launch-evidence-operator-index"/);
  assert.match(cli, /run_launch_evidence_operator_index/);
  assert.match(cli, /"launch_readiness_bundle"/);
  assert.match(cli, /"schema": "dx\.launch\.readiness_bundle"/);
  assert.match(cli, /"file": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE/);
  assert.match(cli, /"launch-readiness-bundle" \| "readiness-bundle" =>/);
  assert.match(cli, /cmd_forge_launch_readiness_bundle/);
  assert.match(launchReadinessBundle, /BUNDLE_SCHEMA: &str = "dx\.forge\.launch_readiness_bundle"/);
  assert.match(launchReadinessBundle, /build_launch_readiness_bundle_report/);
  assert.match(launchReadinessBundle, /launch_readiness_bundle_terminal/);
  assert.match(launchReadinessBundle, /www_template_receipt_present/);
  assert.match(cli, /"template_readiness": "dx templates verify-readiness --project <path> --json"/);
  assert.match(cli, /"forge_template_readiness": "dx forge template-readiness --project <path> --json"/);
  assert.match(cli, /"source_level_package_guard"/);
  assert.match(cli, /"dx run --test \.\\\\benchmarks\\\\launch-package-slices\.test\.ts"/);
  assert.match(cli, /"package_receipts": \{/);
  assert.match(cli, /"required_packages": FORGE_WWW_TEMPLATE_PACKAGE_IDS/);
  assert.ok(
    (cliProduction.match(/"template_readiness_receipt"/g) || []).length >= 2,
    "expected template readiness receipt metadata in templates JSON and generated manifest",
  );
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE: &str\s*=\s*"\.dx\/forge\/template-readiness\/launch-route\.json"/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE: &str\s*=\s*"\.dx\/forge\/template-readiness\/launch-readiness-bundle\.json"/);
  assert.match(cli, /"file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB/);
  assert.match(cli, /write_next_familiar_launch_receipt/);
  assert.match(cli, /write_next_familiar_launch_readiness_bundle/);
  assert.match(cli, /let materialized_files = (?:vec!)?\[/);
  assert.match(cli, /"summary": \{/);
  assert.match(cli, /"required_package_count": FORGE_WWW_TEMPLATE_PACKAGE_IDS\.len\(\)/);
  assert.match(cli, /"materialized_file_count": materialized_files\.len\(\)/);
  assert.match(cli, /"checks_passed": 1/);
  assert.match(cli, /"checks_pending": 1/);
  assert.match(cli, /"runtime_gate": "pending-governed-runtime-pass"/);
  assert.match(cli, /"runtime_verification_requires_explicit_permission": true/);
  assert.match(cli, /"runtime_verification_request": \{/);
  assert.match(cli, /"approval_status": "requires-explicit-permission"/);
  assert.match(cli, /"automation_default": "skip-runtime-build-preview"/);
  assert.match(cli, /"expected_evidence": \[/);
  assert.match(cli, /"requires_explicit_permission": true/);
  assert.doesNotMatch(cliProduction, /"command": "dx build && dx preview --production-contract"/);
  assert.match(cli, /write_next_familiar_launch_route/);
  assert.match(cli, /NEXT_FAMILIAR_HOME_ROUTE_PAGE_TSX/);
  assert.match(cli, /mod default_template_sources;/);
  assert.match(templateSources, /DEFAULT_TEMPLATE_APP_ROUTE_SOURCES/);
  assert.match(templateSources, /route: "\/"/);
  assert.match(templateSources, /aliases: &\[\]/);
  assert.match(templateSources, /materialized_file: "app\/page\.tsx"/);
  assert.match(templateSources, /role: "primary-www-dashboard"/);
  assert.match(
    templateSources,
    /include_str!\("..\/..\/..\/examples\/template\/app\/page\.tsx"\)/,
  );
  assert.doesNotMatch(
    templateSources,
    /NEXT_FAMILIAR_LAUNCH_ROUTE_PAGE_TSX/,
  );
  assert.doesNotMatch(
    cli,
    /include_str!\("..\/..\/..\/examples\/template\/app\/(?:launch\/)?page\.tsx"\)/,
  );
  assert.match(cli, /"app\/page\.tsx",/);
  assert.doesNotMatch(cli, /"app\/launch\/page\.tsx",\s*(launch_page|NEXT_FAMILIAR_LAUNCH_ROUTE_PAGE_TSX)/);
  assert.match(cli, /"components\/template-app\/template-shell\.tsx",\s*(template_shell|NEXT_FAMILIAR_TEMPLATE_SHELL_TSX)/);
  assert.match(cli, /"components\/template-app\/template-dashboard-nav\.tsx",\s*(template_dashboard_nav|NEXT_FAMILIAR_TEMPLATE_DASHBOARD_NAV_TSX)/);
  assert.match(cli, /"components\/template-app\/template-lead-form\.tsx",\s*(template_lead_form|NEXT_FAMILIAR_TEMPLATE_LEAD_FORM_TSX)/);
  assert.match(cli, /"components\/template-app\/instantdb-status\.tsx",\s*(instant_status|NEXT_FAMILIAR_INSTANT_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/wasm-interop-status\.tsx",\s*(wasm_status|NEXT_FAMILIAR_WASM_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/zod-validation-status\.tsx",\s*(zod_status|NEXT_FAMILIAR_ZOD_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/data-status\.tsx",\s*(data_status|NEXT_FAMILIAR_DATA_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/payments-status\.tsx",\s*(payments_status|NEXT_FAMILIAR_PAYMENTS_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/docs-status\.tsx",\s*(docs_status|NEXT_FAMILIAR_DOCS_STATUS_TSX)/);
  assert.match(cli, /"components\/scene\/launch-scene\.tsx",\s*(launch_scene|NEXT_FAMILIAR_LAUNCH_SCENE_TSX)/);
  assert.match(cli, /"components\/template-app\/icon-status\.tsx",\s*(icon_status|NEXT_FAMILIAR_ICON_STATUS_TSX)/);
  assert.match(cli, /"components\/template-app\/next-intl-status\.tsx",\s*(next_intl_status|NEXT_FAMILIAR_INTL_STATUS_TSX)/);
  for (const materializedFile of [
    "app/page.tsx",
    "components/template-app/template-shell.tsx",
    "components/template-app/template-dashboard-nav.tsx",
    "components/template-app/template-lead-form.tsx",
    "components/template-app/template-route-contract.ts",
    "components/template-app/auth-session-status.tsx",
    "components/template-app/ai-chat-status.tsx",
    "components/template-app/data-status.tsx",
    "components/template-app/payments-status.tsx",
    "components/template-app/docs-status.tsx",
    "components/template-app/instantdb-status.tsx",
    "components/template-app/wasm-interop-status.tsx",
    "components/template-app/zod-validation-status.tsx",
    "components/template-app/icon-status.tsx",
    "components/template-app/next-intl-status.tsx",
    "components/template-app/query-cache-status.tsx",
    "components/template-app/package-catalog.ts",
    "components/template-app/template-surface-registry.ts",
    "components/template-app/state-zustand-counter.tsx",
    "components/template-app/react-markdown-preview.tsx",
    "components/template-app/trpc-launch-health.tsx",
    "components/scene/launch-scene.tsx",
    "lib/scene/types.ts",
    "lib/scene/preset.ts",
    "lib/scene/webgl-runtime.ts",
    "lib/scene/metadata.ts",
    "lib/scene/README.md",
    "server/templateCatalog.ts",
  ]) {
    assert.match(
      cli,
      new RegExp(`"${materializedFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`),
      `expected CLI metadata to include ${materializedFile}`,
    );
  }
  for (const file of [
    {
      kind: "app-router-default-route",
      source: "examples/template/app/page.tsx",
      materialized: "app/page.tsx",
    },
    {
      kind: "route-contract",
      source: "examples/template/template-route-contract.ts",
      materialized: "components/template-app/template-route-contract.ts",
    },
    {
      kind: "template-shell",
      source: "examples/template/template-shell.tsx",
      materialized: "components/template-app/template-shell.tsx",
    },
    {
      kind: "template-dashboard-nav",
      source: "examples/template/template-dashboard-nav.tsx",
      materialized: "components/template-app/template-dashboard-nav.tsx",
    },
    {
      kind: "template-lead-form",
      source: "examples/template/template-lead-form.tsx",
      materialized: "components/template-app/template-lead-form.tsx",
    },
    {
      kind: "auth-session-status",
      source: "examples/template/auth-session-status.tsx",
      materialized: "components/template-app/auth-session-status.tsx",
    },
    {
      kind: "ai-chat-status",
      source: "examples/template/ai-chat-status.tsx",
      materialized: "components/template-app/ai-chat-status.tsx",
    },
    {
      kind: "data-status",
      source: "examples/template/data-status.tsx",
      materialized: "components/template-app/data-status.tsx",
    },
    {
      kind: "payments-status",
      source: "examples/template/payments-status.tsx",
      materialized: "components/template-app/payments-status.tsx",
    },
    {
      kind: "docs-status",
      source: "examples/template/docs-status.tsx",
      materialized: "components/template-app/docs-status.tsx",
    },
    {
      kind: "realtime-data-status",
      source: "examples/template/instantdb-status.tsx",
      materialized: "components/template-app/instantdb-status.tsx",
    },
    {
      kind: "wasm-interop-status",
      source: "examples/template/wasm-interop-status.tsx",
      materialized: "components/template-app/wasm-interop-status.tsx",
    },
    {
      kind: "validation-status",
      source: "examples/template/zod-validation-status.tsx",
      materialized: "components/template-app/zod-validation-status.tsx",
    },
    {
      kind: "icon-status",
      source: "examples/template/icon-status.tsx",
      materialized: "components/template-app/icon-status.tsx",
    },
    {
      kind: "i18n-status",
      source: "examples/template/next-intl-status.tsx",
      materialized: "components/template-app/next-intl-status.tsx",
    },
    {
      kind: "package-catalog",
      source: "examples/template/package-catalog.ts",
      materialized: "components/template-app/package-catalog.ts",
    },
    {
      kind: "state-preview",
      source: "examples/template/state-zustand-counter.tsx",
      materialized: "components/template-app/state-zustand-counter.tsx",
    },
    {
      kind: "content-preview",
      source: "examples/template/react-markdown-preview.tsx",
      materialized: "components/template-app/react-markdown-preview.tsx",
    },
    {
      kind: "typed-api-health",
      source: "examples/template/trpc-launch-health.tsx",
      materialized: "components/template-app/trpc-launch-health.tsx",
    },
    {
      kind: "source-owned-scene",
      source: "examples/template/launch-scene.tsx",
      materialized: "components/scene/launch-scene.tsx",
    },
    {
      kind: "source-owned-scene-types",
      source: "examples/template/scene/types.ts",
      materialized: "lib/scene/types.ts",
    },
    {
      kind: "source-owned-scene-preset",
      source: "examples/template/scene/preset.ts",
      materialized: "lib/scene/preset.ts",
    },
    {
      kind: "source-owned-scene-runtime",
      source: "examples/template/scene/webgl-runtime.ts",
      materialized: "lib/scene/webgl-runtime.ts",
    },
    {
      kind: "source-owned-scene-metadata",
      source: "examples/template/scene/metadata.ts",
      materialized: "lib/scene/metadata.ts",
    },
    {
      kind: "source-owned-scene-readme",
      source: "examples/template/scene/README.md",
      materialized: "lib/scene/README.md",
    },
    {
      kind: "query-cache-status",
      source: "examples/template/query-cache-status.tsx",
      materialized: "components/template-app/query-cache-status.tsx",
    },
    {
      kind: "runtime-console",
      source: null,
      materialized: "components/template-app/template-console.tsx",
    },
    {
      kind: "runtime-catalog-loader",
      source: null,
      materialized: "server/templateCatalog.ts",
    },
    {
      kind: "template-readiness-receipt",
      source: null,
      materialized: ".dx/forge/template-readiness/launch-route.json",
    },
    {
      kind: "launch-readiness-bundle",
      source: null,
      materialized: ".dx/forge/template-readiness/launch-readiness-bundle.json",
    },
  ]) {
    const sourcePattern =
      file.source === null
        ? '"source_file": null,\\s*'
        : `"source_file": "${escapeRegExp(file.source)}",\\s*`;
    const materializedPattern =
      file.materialized === ".dx/forge/template-readiness/launch-route.json"
        ? '(?:"\\.dx/forge/template-readiness/launch-route\\.json"|NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE)'
        : file.materialized === ".dx/forge/template-readiness/launch-readiness-bundle.json"
          ? '(?:"\\.dx/forge/template-readiness/launch-readiness-bundle\\.json"|NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE)'
        : `"${escapeRegExp(file.materialized)}"`;
    assert.match(
      cli,
      new RegExp(
        `\\{\\s*"kind": "${escapeRegExp(file.kind)}",\\s*${sourcePattern}"materialized_file": ${materializedPattern}\\s*\\}`,
      ),
      `expected generated template manifest to include ${file.materialized}`,
    );
  }
  assert.match(cli, /examples\/template\/template-shell\.tsx/);
  assert.match(cli, /examples\/template\/template-dashboard-nav\.tsx/);
  assert.match(cli, /examples\/template\/template-lead-form\.tsx/);
  assert.doesNotMatch(cli, /examples\/template\/app\/launch\/page\.tsx/);
  assert.match(cli, /examples\/template\/template-route-contract\.ts/);
  assert.match(cli, /examples\/template\/data-status\.tsx/);
  assert.match(cli, /examples\/template\/payments-status\.tsx/);
  assert.match(cli, /examples\/template\/docs-status\.tsx/);
  assert.match(cli, /examples\/template\/launch-scene\.tsx/);
  assert.match(cli, /examples\/template\/scene\/types\.ts/);
  assert.match(cli, /examples\/template\/scene\/preset\.ts/);
  assert.match(cli, /examples\/template\/scene\/webgl-runtime\.ts/);
  assert.match(cli, /examples\/template\/scene\/metadata\.ts/);
  assert.match(cli, /examples\/template\/scene\/README\.md/);
  assert.match(cli, /examples\/template\/react-markdown-preview\.tsx/);
  assert.match(cli, /examples\/template\/trpc-launch-health\.tsx/);
  assert.match(cli, /"package_id": "api\/trpc"/);
});
