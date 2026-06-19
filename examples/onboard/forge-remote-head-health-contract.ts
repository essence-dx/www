import { launchForgePackageStatus } from "./forge-package-status";
import type { LaunchForgeRemoteObjectHeadHealthStatus } from "./forge-package-status-read-model";

export type ForgeRemoteHeadHealthPanelState = "safe" | "blocked" | "missing";

export type ForgeRemoteHeadHealthPanelContract = {
  readonly schema: "dx.forge.remote_head_health_panel_contract";
  readonly component: "forge-remote-head-health-panel";
  readonly statusSurface: "remote-object-head-health";
  readonly zedSurface: "remote-object-head-health";
  readonly sourceFile: "examples/template/forge-remote-head-health-panel.tsx";
  readonly materializedFile: "components/template-app/forge-remote-head-health-panel.tsx";
  readonly state: ForgeRemoteHeadHealthPanelState;
  readonly rowCount: number;
  readonly packageId: string | null;
  readonly providerKind: string | null;
  readonly safeForRemoteInstall: boolean;
  readonly checkCount: number;
  readonly blockingCheckCount: number;
  readonly missingRequiredCount: number;
  readonly missingOptionalCount: number;
  readonly byteMismatchCount: number;
  readonly sourceReceiptPath: string;
  readonly nextAction: string;
  readonly boundary: string;
};

const missingReceiptPath = ".dx/forge/receipts/remotes";
const missingBoundary =
  "Remote object HEAD health is not measured until an approved dx forge remote-head receipt exists.";
const missingNextAction =
  "Run dx forge remote-head with a real remote manifest before enabling remote install.";

function remoteHeadState(
  row: LaunchForgeRemoteObjectHeadHealthStatus | undefined,
): ForgeRemoteHeadHealthPanelState {
  if (!row) {
    return "missing";
  }
  return row.safeForRemoteInstall ? "safe" : "blocked";
}

export function createForgeRemoteHeadHealthPanelContract(
  rows: readonly LaunchForgeRemoteObjectHeadHealthStatus[] =
    launchForgePackageStatus.remoteObjectHeadHealth,
): ForgeRemoteHeadHealthPanelContract {
  const primary = rows[0];

  return {
    schema: "dx.forge.remote_head_health_panel_contract",
    component: "forge-remote-head-health-panel",
    statusSurface: "remote-object-head-health",
    zedSurface: "remote-object-head-health",
    sourceFile: "examples/template/forge-remote-head-health-panel.tsx",
    materializedFile: "components/template-app/forge-remote-head-health-panel.tsx",
    state: remoteHeadState(primary),
    rowCount: rows.length,
    packageId: primary?.packageId ?? null,
    providerKind: primary?.providerKind ?? null,
    safeForRemoteInstall: primary?.safeForRemoteInstall ?? false,
    checkCount: primary?.checkCount ?? 0,
    blockingCheckCount: primary?.blockingCheckCount ?? 0,
    missingRequiredCount: primary?.missingRequiredCount ?? 0,
    missingOptionalCount: primary?.missingOptionalCount ?? 0,
    byteMismatchCount: primary?.byteMismatchCount ?? 0,
    sourceReceiptPath: primary?.sourceReceiptPath ?? missingReceiptPath,
    nextAction: primary?.nextActions[0] ?? missingNextAction,
    boundary: primary?.boundary ?? missingBoundary,
  };
}

export const forgeRemoteHeadHealthPanelContract =
  createForgeRemoteHeadHealthPanelContract();
