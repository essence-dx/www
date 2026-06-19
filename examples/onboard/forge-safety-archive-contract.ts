import { launchForgePackageStatus } from "./forge-package-status";
import type { LaunchForgeSafetyArchiveStatus } from "./forge-package-status-read-model";

export type ForgeSafetyArchiveContractState = "covered" | "partial" | "missing";

export type ForgeSafetyArchiveContractPackage = {
  readonly name: string;
  readonly rollbackReceiptPath: string | null;
  readonly safetyArchiveReceiptPath: string | null;
  readonly fileCount: number;
  readonly cacheFileCount: number;
  readonly restorePlan: string;
};

export type ForgeSafetyArchiveContract = {
  readonly schema: "dx.forge.safety_archive_contract";
  readonly component: "forge-safety-archive-status";
  readonly statusSurface: "safety-archive-status";
  readonly zedSurface: "safety-archive-status";
  readonly operationSafetySurface: "archive-before-delete";
  readonly sourceFile: "examples/template/forge-safety-archive-contract.ts";
  readonly materializedFile: "components/template-app/forge-safety-archive-contract.ts";
  readonly state: ForgeSafetyArchiveContractState;
  readonly safeForDestructivePackageOperations: boolean;
  readonly archiveDirectory: string;
  readonly packageCount: number;
  readonly rollbackCoveredPackageCount: number;
  readonly rollbackMissingPackageCount: number;
  readonly rollbackCoveragePercent: number;
  readonly archiveReceiptCount: number;
  readonly packages: readonly ForgeSafetyArchiveContractPackage[];
  readonly nextAction: string;
  readonly boundary: string;
};

const missingSafetyArchive: LaunchForgeSafetyArchiveStatus = {
  schema: "dx.www.template.forge_safety_archive_status",
  status: "missing",
  archiveDirectory: ".dx/forge/receipts/safety",
  rollbackCoveredPackageCount: 0,
  rollbackMissingPackageCount: 0,
  rollbackCoveragePercent: 0,
  archiveReceiptCount: 0,
  packages: [],
  boundary:
    "Safety/archive coverage is missing until Forge emits archive-before-delete receipts for tracked packages.",
};

function safetyArchiveState(
  safetyArchive: LaunchForgeSafetyArchiveStatus,
): ForgeSafetyArchiveContractState {
  if (safetyArchive.archiveReceiptCount === 0) {
    return "missing";
  }

  if (
    safetyArchive.rollbackMissingPackageCount === 0 &&
    safetyArchive.rollbackCoveragePercent === 100
  ) {
    return "covered";
  }

  return "partial";
}

function safetyArchiveNextAction(
  state: ForgeSafetyArchiveContractState,
): string {
  if (state === "covered") {
    return "Keep archive-before-delete enabled for Forge remove/update paths and review rollback receipts before restoring files.";
  }

  return "Generate missing safety archive receipts before allowing destructive Forge package operations.";
}

export function createForgeSafetyArchiveContract(
  safetyArchive: LaunchForgeSafetyArchiveStatus =
    launchForgePackageStatus.safetyArchiveStatus ?? missingSafetyArchive,
): ForgeSafetyArchiveContract {
  const state = safetyArchiveState(safetyArchive);

  return {
    schema: "dx.forge.safety_archive_contract",
    component: "forge-safety-archive-status",
    statusSurface: "safety-archive-status",
    zedSurface: "safety-archive-status",
    operationSafetySurface: "archive-before-delete",
    sourceFile: "examples/template/forge-safety-archive-contract.ts",
    materializedFile: "components/template-app/forge-safety-archive-contract.ts",
    state,
    safeForDestructivePackageOperations: state === "covered",
    archiveDirectory: safetyArchive.archiveDirectory,
    packageCount: safetyArchive.packages.length,
    rollbackCoveredPackageCount: safetyArchive.rollbackCoveredPackageCount,
    rollbackMissingPackageCount: safetyArchive.rollbackMissingPackageCount,
    rollbackCoveragePercent: safetyArchive.rollbackCoveragePercent,
    archiveReceiptCount: safetyArchive.archiveReceiptCount,
    packages: safetyArchive.packages,
    nextAction: safetyArchiveNextAction(state),
    boundary: safetyArchive.boundary,
  };
}

export const forgeSafetyArchiveContract = createForgeSafetyArchiveContract();
