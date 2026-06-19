import {
  dxSceneDashboardCameraRigs,
  type DxSceneDashboardCameraRigId,
  type DxSceneDashboardReceipt,
} from "./dashboard-workflow";
import type {
  DxSceneMaterialPaletteId,
  DxSceneQualityProfileId,
} from "./types";

export const dxSceneDashboardIdleReceiptMessage =
  "Render budget receipt not prepared yet.";

export function cycleDxSceneQualityProfile(
  current: DxSceneQualityProfileId,
): DxSceneQualityProfileId {
  return current === "preview" ? "cinematic" : "preview";
}

export function cycleDxSceneMaterialPalette(
  current: DxSceneMaterialPaletteId,
): DxSceneMaterialPaletteId {
  return current === "aurora" ? "graphite" : "aurora";
}

export function cycleDxSceneCameraRig(
  current: DxSceneDashboardCameraRigId,
): DxSceneDashboardCameraRigId {
  const currentIndex = dxSceneDashboardCameraRigs.findIndex(
    (rig) => rig.id === current,
  );
  const nextIndex =
    currentIndex < 0
      ? 0
      : (currentIndex + 1) % dxSceneDashboardCameraRigs.length;

  return dxSceneDashboardCameraRigs[nextIndex]?.id ?? "orbit";
}

export function formatDxSceneDashboardReceiptSummary(
  receipt: DxSceneDashboardReceipt,
  workflowLabel: string,
) {
  return `${receipt.id} prepared for ${workflowLabel} in ${receipt.qualityProfileId}/${receipt.materialPaletteId}/${receipt.cameraRigId}; ${receipt.receiptPath}`;
}
