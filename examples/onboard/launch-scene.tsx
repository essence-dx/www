"use client";

import * as React from "react";

import {
  cycleDxSceneCameraRig,
  cycleDxSceneMaterialPalette,
  cycleDxSceneQualityProfile,
  dxSceneDashboardIdleReceiptMessage,
  formatDxSceneDashboardReceiptSummary,
} from "@/lib/scene/dashboard-controls";
import {
  applyDxSceneDashboardCameraRig,
  createDxSceneDashboardReceipt,
  createDxSceneDashboardWorkflow,
  resolveDxSceneDashboardCameraRig,
} from "@/lib/scene/dashboard-workflow";
import type { DxSceneDashboardCameraRigId } from "@/lib/scene/dashboard-workflow";
import { createDxSceneBoundsReport } from "@/lib/scene/bounds-report";
import { createDxSceneCapabilityReport } from "@/lib/scene/capability-report";
import { captureDxSceneFrameSample } from "@/lib/scene/frame-sample";
import { dxLaunchSceneForgePackage } from "@/lib/scene/metadata";
import {
  createDxLaunchScenePreset,
  dxSceneMaterialPalettes,
  dxSceneQualityProfiles,
} from "@/lib/scene/preset";
import {
  createDxSceneFallbackStyle,
  createDxScenePreviewReadiness,
} from "@/lib/scene/preview-readiness";
import {
  createDxSceneRendererHandoff,
  mountDxSceneWithRenderer,
} from "@/lib/scene/renderer-handoff";
import { createDxSceneRaycastReport } from "@/lib/scene/raycast-report";
import { createDxSceneViewportReport } from "@/lib/scene/viewport-report";
import type {
  DxSceneController,
  DxSceneNodeInteraction,
  DxSceneMaterialPaletteId,
  DxScenePerformanceBand,
  DxScenePerformanceRegressReason,
  DxScenePerformanceSample,
  DxSceneQualityProfileId,
} from "@/lib/scene/types";

type SceneStatus = "booting" | "ready" | "fallback";

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "dx-icon": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement>,
        HTMLElement
      > & {
        name: string;
      };
    }
  }
}

const sceneActionButtonClassName =
  "rounded-md border border-[var(--dx-scene-control-border)] px-2.5 py-1.5 text-[var(--dx-scene-text)] transition hover:border-[var(--dx-scene-control-hover-border)] hover:bg-[var(--dx-scene-control-hover)]";

export function LaunchScene() {
  const canvasRef = React.useRef<HTMLCanvasElement | null>(null);
  const controllerRef = React.useRef<DxSceneController | null>(null);
  const [status, setStatus] = React.useState<SceneStatus>("booting");
  const [motionMode, setMotionMode] = React.useState<"full" | "reduced">(
    "full",
  );
  const [performanceBand, setPerformanceBand] =
    React.useState<DxScenePerformanceBand>("stable");
  const [regressionReason, setRegressionReason] =
    React.useState<DxScenePerformanceRegressReason>("manual");
  const [qualityProfileId, setQualityProfileId] =
    React.useState<DxSceneQualityProfileId>("preview");
  const [cameraRigId, setCameraRigId] =
    React.useState<DxSceneDashboardCameraRigId>("orbit");
  const [materialPaletteId, setMaterialPaletteId] =
    React.useState<DxSceneMaterialPaletteId>("aurora");
  const [activeNodeId, setActiveNodeId] = React.useState("none");
  const [selectedNodeIndex, setSelectedNodeIndex] = React.useState(0);
  const [receiptState, setReceiptState] = React.useState<
    "idle" | "prepared"
  >("idle");
  const [receiptOutput, setReceiptOutput] = React.useState(
    dxSceneDashboardIdleReceiptMessage,
  );
  const [frameSample, setFrameSample] = React.useState("pending");
  const [capabilityReport, setCapabilityReport] = React.useState(() =>
    createDxSceneCapabilityReport({
      canvas: null,
      performanceBand: "stable",
      qualityProfileId: "preview",
      rendererId: "source-owned-webgl",
    }),
  );
  const [viewportReport, setViewportReport] = React.useState(() =>
    createDxSceneViewportReport({
      canvas: null,
      performanceBand: "stable",
      qualityProfileId: "preview",
      rendererId: "source-owned-webgl",
    }),
  );
  const [boundsReport, setBoundsReport] = React.useState(() =>
    createDxSceneBoundsReport({
      cameraRigId: "orbit",
      scene: createDxLaunchScenePreset({
        palette: "aurora",
        quality: "preview",
      }),
    }),
  );
  const [raycastReport, setRaycastReport] = React.useState(() =>
    createDxSceneRaycastReport({
      scene: createDxLaunchScenePreset({
        palette: "aurora",
        quality: "preview",
      }),
      selectedNodeIndex: 0,
    }),
  );
  const motionModeRef = React.useRef(motionMode);
  const performanceBandRef = React.useRef(performanceBand);
  const regressionReasonRef = React.useRef(regressionReason);
  const activeNodeIdRef = React.useRef(activeNodeId);
  const selectedNodeIndexRef = React.useRef(selectedNodeIndex);
  const scene = React.useMemo(() => {
    const baseScene =
      qualityProfileId === "preview"
        ? createDxLaunchScenePreset({
            palette: materialPaletteId,
            quality: "preview",
          })
        : createDxLaunchScenePreset({
            palette: materialPaletteId,
            quality: qualityProfileId,
          });
    return applyDxSceneDashboardCameraRig(baseScene, cameraRigId);
  }, [cameraRigId, materialPaletteId, qualityProfileId]);
  const activeQualityProfile = dxSceneQualityProfiles[qualityProfileId];
  const activeMaterialPalette = dxSceneMaterialPalettes[materialPaletteId];
  const activeCameraRig = resolveDxSceneDashboardCameraRig(cameraRigId);
  const selectedNode = scene.nodes[selectedNodeIndex] ?? scene.nodes[0] ?? null;
  const handoff = React.useMemo(
    () => createDxSceneRendererHandoff(scene),
    [scene],
  );
  const readiness = React.useMemo(
    () => createDxScenePreviewReadiness(handoff.scene),
    [handoff],
  );
  const fallbackStyle = React.useMemo(
    () => createDxSceneFallbackStyle(handoff.scene) as React.CSSProperties,
    [handoff],
  );
  const dashboardWorkflow = React.useMemo(
    () =>
      createDxSceneDashboardWorkflow(
        scene,
        selectedNodeIndex,
        performanceBand,
        regressionReason,
        cameraRigId,
        materialPaletteId,
      ),
    [
      cameraRigId,
      materialPaletteId,
      performanceBand,
      regressionReason,
      scene,
      selectedNodeIndex,
    ],
  );
  const activeWorkflow = dashboardWorkflow.active;

  const syncNodeInteraction = React.useCallback(
    (interaction: DxSceneNodeInteraction) => {
      const nodeId = interaction.nodeId ?? "none";
      if (activeNodeIdRef.current !== nodeId) {
        activeNodeIdRef.current = nodeId;
        setActiveNodeId(nodeId);
      }
      if (
        typeof interaction.index === "number" &&
        selectedNodeIndexRef.current !== interaction.index
      ) {
        selectedNodeIndexRef.current = interaction.index;
        setSelectedNodeIndex(interaction.index);
      }
    },
    [],
  );

  const syncPerformanceSample = React.useCallback(
    (sample: DxScenePerformanceSample) => {
      if (performanceBandRef.current !== sample.band) {
        performanceBandRef.current = sample.band;
        setPerformanceBand(sample.band);
      }
      if (regressionReasonRef.current !== sample.reason) {
        regressionReasonRef.current = sample.reason;
        setRegressionReason(sample.reason);
      }
    },
    [],
  );

  const selectSceneNode = React.useCallback(
    (index: number) => {
      const nodeCount = scene.nodes.length;
      if (nodeCount === 0) {
        activeNodeIdRef.current = "none";
        setActiveNodeId("none");
        return;
      }

      const nextIndex = ((index % nodeCount) + nodeCount) % nodeCount;
      selectedNodeIndexRef.current = nextIndex;
      setSelectedNodeIndex(nextIndex);
      setReceiptState("idle");
      setReceiptOutput(dxSceneDashboardIdleReceiptMessage);

      const interaction = controllerRef.current?.selectNodeByIndex(
        nextIndex,
        "select",
      );
      if (interaction) {
        syncNodeInteraction(interaction);
        return;
      }

      const nodeId = scene.nodes[nextIndex]?.id ?? "none";
      activeNodeIdRef.current = nodeId;
      setActiveNodeId(nodeId);
    },
    [scene.nodes, syncNodeInteraction],
  );

  const selectNextSceneNode = React.useCallback(() => {
    selectSceneNode(selectedNodeIndexRef.current + 1);
  }, [selectSceneNode]);

  const regressScenePerformance = React.useCallback(() => {
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
    const sample = controllerRef.current?.regressPerformance("manual");
    regressionReasonRef.current = "manual";
    setRegressionReason("manual");
    if (sample) {
      syncPerformanceSample(sample);
    }
  }, [syncPerformanceSample]);

  const resetScenePerformance = React.useCallback(() => {
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
    const sample = controllerRef.current?.resetPerformance();
    if (sample) {
      syncPerformanceSample(sample);
      return;
    }
    performanceBandRef.current = "stable";
    regressionReasonRef.current = "manual";
    setPerformanceBand("stable");
    setRegressionReason("manual");
    setReceiptState("idle");
  }, [syncPerformanceSample]);

  const prepareSceneWorkflowReceipt = React.useCallback(() => {
    const receipt = createDxSceneDashboardReceipt(dashboardWorkflow);
    setReceiptState("prepared");
    setReceiptOutput(
      formatDxSceneDashboardReceiptSummary(receipt, activeWorkflow.label),
    );
  }, [activeWorkflow.label, dashboardWorkflow]);

  const toggleSceneQualityProfile = React.useCallback(() => {
    setQualityProfileId(cycleDxSceneQualityProfile);
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
    setFrameSample("pending");
  }, []);

  const cycleSceneMaterialPalette = React.useCallback(() => {
    setMaterialPaletteId(cycleDxSceneMaterialPalette);
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
    setFrameSample("pending");
  }, []);

  const cycleSceneCameraRig = React.useCallback(() => {
    setCameraRigId(cycleDxSceneCameraRig);
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
    setFrameSample("pending");
  }, []);

  const captureSceneFrameSample = React.useCallback(() => {
    const sample = captureDxSceneFrameSample({
      canvas: canvasRef.current,
      performanceBand,
      qualityProfileId,
      status,
    });
    setFrameSample(sample.value);
  }, [performanceBand, qualityProfileId, status]);

  const inspectRendererCapabilities = React.useCallback(() => {
    setCapabilityReport(
      createDxSceneCapabilityReport({
        canvas: canvasRef.current,
        performanceBand,
        qualityProfileId,
        rendererId: handoff.renderer.id,
      }),
    );
  }, [handoff.renderer.id, performanceBand, qualityProfileId]);

  const measureSceneViewport = React.useCallback(() => {
    setViewportReport(
      createDxSceneViewportReport({
        canvas: canvasRef.current,
        devicePixelRatio: window.devicePixelRatio,
        performanceBand,
        qualityProfileId,
        rendererId: handoff.renderer.id,
      }),
    );
  }, [handoff.renderer.id, performanceBand, qualityProfileId]);

  const fitSceneBounds = React.useCallback(() => {
    setBoundsReport(
      createDxSceneBoundsReport({
        cameraRigId,
        margin: qualityProfileId === "cinematic" ? 1.18 : 1.24,
        scene,
      }),
    );
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
  }, [cameraRigId, qualityProfileId, scene]);

  const inspectSceneRaycastHit = React.useCallback(() => {
    setRaycastReport(
      createDxSceneRaycastReport({
        scene,
        selectedNodeIndex,
      }),
    );
    setReceiptState("idle");
    setReceiptOutput(dxSceneDashboardIdleReceiptMessage);
  }, [scene, selectedNodeIndex]);

  React.useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      setStatus("fallback");
      return;
    }

    const controller = mountDxSceneWithRenderer(canvas, handoff, {
      requestFrame: window.requestAnimationFrame.bind(window),
      cancelFrame: window.cancelAnimationFrame.bind(window),
      regressOnPointerMove: true,
      keyboard: { enabled: true, loop: true },
      onStatusChange: setStatus,
      onFrame: (frame) => {
        if (motionModeRef.current === frame.motionMode) {
          return;
        }
        motionModeRef.current = frame.motionMode;
        setMotionMode(frame.motionMode);
      },
      onPerformanceChange: (sample) => {
        syncPerformanceSample(sample);
      },
      onPerformanceRegression: (regression) => {
        if (regressionReasonRef.current === regression.reason) {
          return;
        }
        regressionReasonRef.current = regression.reason;
        setRegressionReason(regression.reason);
      },
      onNodeHover: syncNodeInteraction,
      onNodeSelect: syncNodeInteraction,
    });
    controllerRef.current = controller;

    return () => {
      controllerRef.current = null;
      controller.dispose();
    };
  }, [handoff, syncNodeInteraction, syncPerformanceSample]);

  return (
    <div
      className="relative min-h-64 overflow-hidden rounded-md border border-[var(--dx-scene-border)] bg-[var(--dx-scene-surface)] shadow-2xl shadow-[var(--dx-scene-shadow)]"
      style={fallbackStyle}
      data-dx-scene-motion-mode={motionMode}
      data-dx-scene-performance-band={performanceBand}
      data-dx-scene-preview-readiness={readiness.status}
      data-dx-scene-active-node={activeNodeId}
      data-dx-scene-regression-reason={regressionReason}
      data-dx-scene-renderer={handoff.renderer.id}
      data-dx-scene-status={status}
      data-dx-scene-quality-profile={qualityProfileId}
      data-dx-scene-material-palette={materialPaletteId}
      data-dx-scene-camera-rig={cameraRigId}
      data-dx-scene-frame-sample={frameSample}
      data-dx-scene-capability-report={capabilityReport.summary}
      data-dx-scene-capability-status={capabilityReport.status}
      data-dx-scene-viewport-report={viewportReport.summary}
      data-dx-scene-viewport-status={viewportReport.status}
      data-dx-scene-bounds-report={boundsReport.summary}
      data-dx-scene-bounds-status={boundsReport.status}
      data-dx-scene-raycast-report={raycastReport.summary}
      data-dx-scene-raycast-status={raycastReport.status}
      data-dx-dashboard-workflow="scene-visual-ops"
      data-dx-component="launch-scene-webgl-proof"
      data-dx-editable="design-token insert_icon_media"
      data-dx-edit-id="launch.scene"
      data-dx-edit-kind="visual-scene"
      data-dx-edit-ops="design-token insert_icon_media scene-preview"
      data-dx-icon="pack:three-scene"
      data-dx-package="3d/launch-scene"
      data-dx-product-surface="visual-operations-dashboard"
      data-dx-scene-workflow-active={activeWorkflow.id}
      data-dx-scene-workflow-receipt-state={receiptState}
      data-dx-style-surface="launch-scene"
      data-dx-token-scope="3d/launch-scene"
    >
      <div
        className="absolute inset-0"
        data-dx-scene-fallback-layer
        style={fallbackStyle}
      />
      <canvas
        ref={canvasRef}
        className="absolute inset-0 h-full w-full"
        aria-label="DX launch WebGL scene"
        data-dx-scene-canvas
        data-dx-scene-keyboard="node-navigation"
        role="img"
        tabIndex={0}
      />
      <div className="pointer-events-none relative z-10 flex min-h-64 flex-col justify-between bg-[var(--dx-scene-layer)] p-5 text-[var(--dx-scene-text)]">
        <div className="flex items-center justify-between gap-3 text-xs text-[var(--dx-scene-muted)]">
          <span className="inline-flex items-center gap-2">
            <dx-icon name="pack:three-scene" aria-hidden="true" />
            {dxLaunchSceneForgePackage.packageId}
          </span>
          <span className="rounded-md border border-[var(--dx-scene-border)] bg-[var(--dx-scene-chip)] px-2 py-1 text-[11px] text-[var(--dx-scene-accent-text)]">
            {status} / {activeQualityProfile.label} / {motionMode} /{" "}
            {performanceBand} / {activeMaterialPalette.label} /{" "}
            {activeCameraRig.label} / {activeWorkflow.id}
          </span>
        </div>
        <div>
          <p className="text-xs font-medium uppercase text-[var(--dx-scene-accent-text)]">
            Visual operations
          </p>
          <p className="mt-2 max-w-sm text-sm leading-6 text-[var(--dx-scene-text)]">
            Web Preview scene control for runtime health, render budget, and
            Studio-selectable camera state.
          </p>
          <div
            className="mt-3 grid gap-1 text-xs text-[var(--dx-scene-success)]"
            data-dx-scene-workflow-card={activeWorkflow.id}
          >
            <span id="scene-workflow-status">
              {activeWorkflow.label}: {activeWorkflow.dashboardMetric}
            </span>
            <span>{activeWorkflow.recommendedAction}</span>
          </div>
        </div>
      </div>
      <div
        className="pointer-events-auto absolute inset-x-4 bottom-4 z-20 flex flex-wrap items-center gap-2 rounded-md border border-[var(--dx-scene-border)] bg-[var(--dx-scene-control)] p-2 text-xs text-[var(--dx-scene-text)] shadow-lg shadow-[var(--dx-scene-shadow)] backdrop-blur-md"
        data-dx-scene-workflow-controls
        data-dx-scene-workflow-selected-node={activeNodeId}
      >
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="focus-dashboard-node"
          data-dx-scene-workflow-action="select-node"
          onClick={selectNextSceneNode}
        >
          Focus {selectedNode?.id ?? "node"}
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="apply-render-budget"
          onClick={prepareSceneWorkflowReceipt}
        >
          Apply render budget
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          aria-pressed={qualityProfileId === "cinematic"}
          data-dx-scene-action="toggle-quality-profile"
          onClick={toggleSceneQualityProfile}
        >
          Quality: {activeQualityProfile.label}
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="cycle-material-palette"
          onClick={cycleSceneMaterialPalette}
        >
          Material: {activeMaterialPalette.label}
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="cycle-camera-rig"
          onClick={cycleSceneCameraRig}
        >
          Camera: {activeCameraRig.label}
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="capture-frame-sample"
          onClick={captureSceneFrameSample}
        >
          Capture frame sample
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="inspect-renderer-capabilities"
          onClick={inspectRendererCapabilities}
        >
          Inspect renderer
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="measure-viewport-dpr"
          onClick={measureSceneViewport}
        >
          Measure viewport
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="fit-scene-bounds"
          onClick={fitSceneBounds}
        >
          Fit bounds
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="inspect-raycast-hit"
          onClick={inspectSceneRaycastHit}
        >
          Inspect hit map
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="regress-performance"
          data-dx-scene-workflow-action="regress-performance"
          onClick={regressScenePerformance}
        >
          Regress performance
        </button>
        <button
          type="button"
          className={sceneActionButtonClassName}
          data-dx-scene-action="reset-render-budget"
          data-dx-scene-workflow-action="reset-performance"
          onClick={resetScenePerformance}
        >
          Reset
        </button>
        <span className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]">
          Active node: {activeNodeId}
        </span>
        <span
          id="scene-material-palette"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-material-palette={materialPaletteId}
        >
          Material: {activeMaterialPalette.label}
        </span>
        <span
          id="scene-camera-rig"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-camera-rig={cameraRigId}
        >
          Camera rig: {activeCameraRig.label}
        </span>
        <span
          id="scene-frame-sample"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-frame-sample={frameSample}
        >
          Frame sample: {frameSample}
        </span>
        <span
          id="scene-capability-report"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-capability-report={capabilityReport.summary}
          data-dx-scene-capability-status={capabilityReport.status}
        >
          Renderer: {capabilityReport.summary}
        </span>
        <span
          id="scene-viewport-report"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-viewport-report={viewportReport.summary}
          data-dx-scene-viewport-status={viewportReport.status}
        >
          Viewport: {viewportReport.summary}
        </span>
        <span
          id="scene-bounds-report"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-bounds-report={boundsReport.summary}
          data-dx-scene-bounds-status={boundsReport.status}
        >
          Bounds: {boundsReport.summary}
        </span>
        <span
          id="scene-raycast-report"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-raycast-report={raycastReport.summary}
          data-dx-scene-raycast-status={raycastReport.status}
        >
          Raycast: {raycastReport.summary}
        </span>
        <span
          id="scene-receipt-output"
          className="rounded-md bg-[var(--dx-scene-chip)] px-2.5 py-1.5 text-[var(--dx-scene-muted)]"
          data-dx-scene-dashboard-receipt={receiptState}
        >
          {receiptOutput}
        </span>
      </div>
    </div>
  );
}
