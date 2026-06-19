import { dxLaunchSceneForgePackage } from "./metadata";
import type {
  DxLaunchScenePreset,
  DxSceneCamera,
  DxSceneMaterialPaletteId,
  DxScenePerformanceBand,
  DxScenePerformanceRegressReason,
  DxSceneQualityProfileId,
} from "./types";

export type DxSceneDashboardWorkflowId =
  | "visual-runtime-health"
  | "render-budget"
  | "web-preview-readiness";

export type DxSceneDashboardWorkflowProfile = {
  id: DxSceneDashboardWorkflowId;
  label: string;
  dashboardMetric: string;
  dashboardTarget: string;
  recommendedAction: string;
};

export type DxSceneDashboardWorkflowNode = DxSceneDashboardWorkflowProfile & {
  nodeId: string;
  nodeKind: string;
  sourceIndex: number;
};

export type DxSceneDashboardCameraRigId =
  | "orbit"
  | "isometric"
  | "inspection";

export type DxSceneDashboardCameraRig = {
  id: DxSceneDashboardCameraRigId;
  label: string;
  camera: DxSceneCamera;
  dashboardMetric: string;
  recommendedAction: string;
};

export type DxSceneDashboardWorkflow = {
  packageId: "3d/launch-scene";
  workflowId: "scene-visual-ops";
  active: DxSceneDashboardWorkflowNode;
  nodes: readonly DxSceneDashboardWorkflowNode[];
  qualityProfileId: DxSceneQualityProfileId;
  materialPaletteId: DxSceneMaterialPaletteId;
  cameraRigId: DxSceneDashboardCameraRigId;
  performanceBand: DxScenePerformanceBand;
  regressionReason: DxScenePerformanceRegressReason;
  noNodeModulesRequired: true;
  receiptPath: string;
  appOwnedBoundaries: readonly string[];
};

export type DxSceneDashboardReceipt = {
  id: string;
  packageId: "3d/launch-scene";
  workflowId: "scene-visual-ops";
  activeNodeId: string;
  activeWorkflowId: DxSceneDashboardWorkflowId;
  qualityProfileId: DxSceneQualityProfileId;
  materialPaletteId: DxSceneMaterialPaletteId;
  cameraRigId: DxSceneDashboardCameraRigId;
  performanceBand: DxScenePerformanceBand;
  regressionReason: DxScenePerformanceRegressReason;
  receiptPath: string;
  status: "prepared-local-preview";
  noNodeModulesRequired: true;
};

export const dxSceneDashboardWorkflowProfiles: readonly DxSceneDashboardWorkflowProfile[] =
  [
    {
      id: "visual-runtime-health",
      label: "Runtime health",
      dashboardMetric: "WebGL active",
      dashboardTarget: "scene-runtime-status",
      recommendedAction: "Focus the node that owns live Web Preview status.",
    },
    {
      id: "render-budget",
      label: "Render budget",
      dashboardMetric: "Preview quality",
      dashboardTarget: "scene-render-budget",
      recommendedAction: "Apply the preview-safe render budget before capture.",
    },
    {
      id: "web-preview-readiness",
      label: "Web Preview readiness",
      dashboardMetric: "Zed selectable",
      dashboardTarget: "scene-preview-marker",
      recommendedAction: "Verify Studio markers before visual QA.",
    },
  ] as const;

export const dxSceneDashboardCameraRigs: readonly DxSceneDashboardCameraRig[] =
  [
    {
      id: "orbit",
      label: "Orbit",
      camera: {
        position: [0, 0, 5.2],
        target: [0, 0, 0],
        focalLength: 42,
      },
      dashboardMetric: "Balanced orbit view",
      recommendedAction: "Use for normal Web Preview review.",
    },
    {
      id: "isometric",
      label: "Isometric",
      camera: {
        position: [3.2, 2.1, 4.6],
        target: [0.12, 0.08, 0],
        focalLength: 38,
      },
      dashboardMetric: "Source graph overview",
      recommendedAction: "Use when inspecting section relationships.",
    },
    {
      id: "inspection",
      label: "Inspection",
      camera: {
        position: [-2.1, 0.55, 3.6],
        target: [0.22, -0.08, 0],
        focalLength: 55,
      },
      dashboardMetric: "Close source marker pass",
      recommendedAction: "Use before capturing a frame sample.",
    },
  ] as const;

export function resolveDxSceneDashboardCameraRig(
  rigId: DxSceneDashboardCameraRigId = "orbit",
): DxSceneDashboardCameraRig {
  return (
    dxSceneDashboardCameraRigs.find((rig) => rig.id === rigId) ??
    dxSceneDashboardCameraRigs[0]
  );
}

export function applyDxSceneDashboardCameraRig(
  scene: DxLaunchScenePreset,
  rigId: DxSceneDashboardCameraRigId = "orbit",
): DxLaunchScenePreset {
  const rig = resolveDxSceneDashboardCameraRig(rigId);
  return {
    ...scene,
    camera: rig.camera,
    controls: {
      ...scene.controls,
      pointerParallax: rig.id === "inspection" ? 0.1 : scene.controls.pointerParallax,
      motionSpeed: rig.id === "isometric" ? 0.82 : scene.controls.motionSpeed,
    },
  };
}

export function createDxSceneDashboardWorkflow(
  scene: DxLaunchScenePreset,
  selectedNodeIndex = 0,
  performanceBand: DxScenePerformanceBand = "stable",
  regressionReason: DxScenePerformanceRegressReason = "manual",
  cameraRigId: DxSceneDashboardCameraRigId = "orbit",
  materialPaletteId: DxSceneMaterialPaletteId = "aurora",
): DxSceneDashboardWorkflow {
  const nodes = scene.nodes.map((node, index) => {
    const profile =
      dxSceneDashboardWorkflowProfiles[
        index % dxSceneDashboardWorkflowProfiles.length
      ] ?? dxSceneDashboardWorkflowProfiles[0];
    return {
      ...profile,
      nodeId: node.id,
      nodeKind: node.kind,
      sourceIndex: index,
    };
  });
  const nodeCount = Math.max(nodes.length, 1);
  const active =
    nodes[
      ((selectedNodeIndex % nodeCount) + nodeCount) % nodeCount
    ] ?? {
      ...dxSceneDashboardWorkflowProfiles[0],
      nodeId: "none",
      nodeKind: "none",
      sourceIndex: -1,
    };

  return {
    packageId: dxLaunchSceneForgePackage.packageId,
    workflowId: "scene-visual-ops",
    active,
    nodes,
    qualityProfileId: scene.quality.id,
    materialPaletteId,
    cameraRigId,
    performanceBand,
    regressionReason,
    noNodeModulesRequired: true,
    receiptPath: dxLaunchSceneForgePackage.receiptPaths.dashboardWorkflow,
    appOwnedBoundaries: dxLaunchSceneForgePackage.appOwnedBoundaries,
  };
}

export function createDxSceneDashboardReceipt(
  workflow: DxSceneDashboardWorkflow,
): DxSceneDashboardReceipt {
  return {
    id: `scene-render-budget-${workflow.active.nodeId}-${workflow.performanceBand}-${workflow.qualityProfileId}-${workflow.materialPaletteId}-${workflow.cameraRigId}`,
    packageId: workflow.packageId,
    workflowId: workflow.workflowId,
    activeNodeId: workflow.active.nodeId,
    activeWorkflowId: workflow.active.id,
    qualityProfileId: workflow.qualityProfileId,
    materialPaletteId: workflow.materialPaletteId,
    cameraRigId: workflow.cameraRigId,
    performanceBand: workflow.performanceBand,
    regressionReason: workflow.regressionReason,
    receiptPath: workflow.receiptPath,
    status: "prepared-local-preview",
    noNodeModulesRequired: true,
  };
}
