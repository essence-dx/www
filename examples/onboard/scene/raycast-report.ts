import { createDxSceneInteractionMap, pickDxSceneNode } from "./interaction";
import type { DxLaunchScenePreset, DxSceneVec2 } from "./types";

export type DxSceneRaycastReportStatus = "hit" | "miss" | "empty-scene";

export type DxSceneRaycastReportInput = {
  scene: DxLaunchScenePreset;
  elapsed?: number;
  pointer?: DxSceneVec2;
  selectedNodeIndex?: number | null;
};

export type DxSceneRaycastReport = {
  status: DxSceneRaycastReportStatus;
  nodeId: string | null;
  nodeKind: string | null;
  pointer: DxSceneVec2;
  anchor: DxSceneVec2 | null;
  distance: number | null;
  threshold: number | null;
  summary: string;
  sourceApis: readonly [
    "THREE.Raycaster",
    "THREE.Raycaster.intersectObject",
    "@react-three/fiber state.raycaster",
    "@react-three/drei meshBounds",
  ];
};

const sourceApis = [
  "THREE.Raycaster",
  "THREE.Raycaster.intersectObject",
  "@react-three/fiber state.raycaster",
  "@react-three/drei meshBounds",
] as const;

function roundMetric(value: number) {
  return Math.round(value * 1000) / 1000;
}

function resolvePointer(input: DxSceneRaycastReportInput): DxSceneVec2 {
  if (input.pointer) {
    return [roundMetric(input.pointer[0]), roundMetric(input.pointer[1])];
  }

  const hitMap = createDxSceneInteractionMap(input.scene, input.elapsed ?? 0);
  if (hitMap.length === 0) {
    return [0, 0];
  }

  const index =
    typeof input.selectedNodeIndex === "number"
      ? ((input.selectedNodeIndex % hitMap.length) + hitMap.length) %
        hitMap.length
      : 0;
  const anchor = hitMap[index]?.anchor ?? hitMap[0]?.anchor ?? [0, 0];
  return [roundMetric(anchor[0]), roundMetric(anchor[1])];
}

function createSummary(report: Omit<DxSceneRaycastReport, "summary">) {
  return [
    "Raycaster",
    report.status,
    report.nodeId ?? "none",
    `pointer${report.pointer.join(",")}`,
    report.distance === null ? "distance-none" : `distance${report.distance}`,
    report.threshold === null ? "threshold-none" : `threshold${report.threshold}`,
  ].join(":");
}

export function createDxSceneRaycastReport(
  input: DxSceneRaycastReportInput,
): DxSceneRaycastReport {
  if (input.scene.nodes.length === 0) {
    const report = {
      anchor: null,
      distance: null,
      nodeId: null,
      nodeKind: null,
      pointer: [0, 0] as DxSceneVec2,
      sourceApis,
      status: "empty-scene" as const,
      threshold: null,
    };
    return { ...report, summary: createSummary(report) };
  }

  const pointer = resolvePointer(input);
  const hit = pickDxSceneNode(
    input.scene,
    pointer,
    input.elapsed ?? 0,
  );
  const report = {
    anchor: hit ? [roundMetric(hit.anchor[0]), roundMetric(hit.anchor[1])] as DxSceneVec2 : null,
    distance: hit ? roundMetric(hit.distance) : null,
    nodeId: hit?.nodeId ?? null,
    nodeKind: hit?.node.kind ?? null,
    pointer,
    sourceApis,
    status: hit ? ("hit" as const) : ("miss" as const),
    threshold: hit ? roundMetric(hit.threshold + hit.radius) : null,
  };

  return { ...report, summary: createSummary(report) };
}
