import { resolveDxSceneDashboardCameraRig } from "./dashboard-workflow";
import type { DxSceneDashboardCameraRigId } from "./dashboard-workflow";
import { createDxSceneInteractionMap } from "./interaction";
import type { DxLaunchScenePreset, DxSceneVec3 } from "./types";

export type DxSceneBoundsReportStatus = "ready" | "empty-scene";

export type DxSceneBoundsBox = {
  min: DxSceneVec3;
  max: DxSceneVec3;
  size: DxSceneVec3;
  center: DxSceneVec3;
  radius: number;
  nodeCount: number;
};

export type DxSceneBoundsReportInput = {
  scene: DxLaunchScenePreset;
  cameraRigId?: DxSceneDashboardCameraRigId;
  elapsed?: number;
  margin?: number;
};

export type DxSceneBoundsReport = {
  status: DxSceneBoundsReportStatus;
  box: DxSceneBoundsBox;
  fitTarget: DxSceneVec3;
  cameraDistance: number;
  cameraPosition: DxSceneVec3;
  cameraRigId: DxSceneDashboardCameraRigId;
  summary: string;
  sourceApis: readonly [
    "THREE.Box3",
    "THREE.Box3.setFromObject",
    "@react-three/drei Bounds",
    "@react-three/drei Bounds.fit",
  ];
};

const sourceApis = [
  "THREE.Box3",
  "THREE.Box3.setFromObject",
  "@react-three/drei Bounds",
  "@react-three/drei Bounds.fit",
] as const;

function roundMetric(value: number) {
  return Math.round(value * 1000) / 1000;
}

function roundVec3(value: DxSceneVec3): DxSceneVec3 {
  return [
    roundMetric(value[0]),
    roundMetric(value[1]),
    roundMetric(value[2]),
  ];
}

function normalizeDirection(value: DxSceneVec3): DxSceneVec3 {
  const length = Math.hypot(value[0], value[1], value[2]) || 1;
  return [value[0] / length, value[1] / length, value[2] / length];
}

function createEmptyBounds(): DxSceneBoundsBox {
  return {
    center: [0, 0, 0],
    max: [0, 0, 0],
    min: [0, 0, 0],
    nodeCount: 0,
    radius: 0,
    size: [0, 0, 0],
  };
}

function createSummary(report: Omit<DxSceneBoundsReport, "summary">) {
  const box = report.box;
  return [
    "Box3",
    report.status,
    report.cameraRigId,
    `nodes${box.nodeCount}`,
    `size${box.size.join("x")}`,
    `fitTarget${report.fitTarget.join(",")}`,
    `cameraDistance${report.cameraDistance}`,
  ].join(":");
}

export function createDxSceneBoundsReport(
  input: DxSceneBoundsReportInput,
): DxSceneBoundsReport {
  const nodes = createDxSceneInteractionMap(input.scene, input.elapsed ?? 0);
  const cameraRig = resolveDxSceneDashboardCameraRig(input.cameraRigId);
  const margin = Math.max(1, input.margin ?? 1.24);

  if (nodes.length === 0) {
    const empty = createEmptyBounds();
    const direction = normalizeDirection([
      cameraRig.camera.position[0] - cameraRig.camera.target[0],
      cameraRig.camera.position[1] - cameraRig.camera.target[1],
      cameraRig.camera.position[2] - cameraRig.camera.target[2],
    ]);
    const report = {
      box: empty,
      cameraDistance: roundMetric(3.2 * margin),
      cameraPosition: roundVec3([
        direction[0] * 3.2 * margin,
        direction[1] * 3.2 * margin,
        direction[2] * 3.2 * margin,
      ]),
      cameraRigId: cameraRig.id,
      fitTarget: empty.center,
      sourceApis,
      status: "empty-scene" as const,
    };
    return { ...report, summary: createSummary(report) };
  }

  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;

  nodes.forEach((hit, index) => {
    const z = (index - (nodes.length - 1) / 2) * 0.18;
    const radius = Math.max(hit.radius, hit.node.radius * 0.18);
    minX = Math.min(minX, hit.anchor[0] - radius);
    minY = Math.min(minY, hit.anchor[1] - radius);
    minZ = Math.min(minZ, z - radius * 0.48);
    maxX = Math.max(maxX, hit.anchor[0] + radius);
    maxY = Math.max(maxY, hit.anchor[1] + radius);
    maxZ = Math.max(maxZ, z + radius * 0.48);
  });

  const min: DxSceneVec3 = roundVec3([minX, minY, minZ]);
  const max: DxSceneVec3 = roundVec3([maxX, maxY, maxZ]);
  const size: DxSceneVec3 = roundVec3([
    maxX - minX,
    maxY - minY,
    maxZ - minZ,
  ]);
  const center: DxSceneVec3 = roundVec3([
    minX + (maxX - minX) / 2,
    minY + (maxY - minY) / 2,
    minZ + (maxZ - minZ) / 2,
  ]);
  const radius = roundMetric(Math.hypot(size[0], size[1], size[2]) / 2);
  const cameraDistance = roundMetric(
    Math.max(radius * margin * 2.15, 2.8),
  );
  const direction = normalizeDirection([
    cameraRig.camera.position[0] - cameraRig.camera.target[0],
    cameraRig.camera.position[1] - cameraRig.camera.target[1],
    cameraRig.camera.position[2] - cameraRig.camera.target[2],
  ]);
  const cameraPosition = roundVec3([
    center[0] + direction[0] * cameraDistance,
    center[1] + direction[1] * cameraDistance,
    center[2] + direction[2] * cameraDistance,
  ]);

  const report = {
    box: {
      center,
      max,
      min,
      nodeCount: nodes.length,
      radius,
      size,
    },
    cameraDistance,
    cameraPosition,
    cameraRigId: cameraRig.id,
    fitTarget: center,
    sourceApis,
    status: "ready" as const,
  };

  return { ...report, summary: createSummary(report) };
}
