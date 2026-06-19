import { resolveDxSceneQualityProfile } from "./preset";
import type {
  DxScenePerformanceBand,
  DxSceneQualityProfileId,
  DxSceneRendererId,
} from "./types";

export type DxSceneViewportReportStatus =
  | "ready"
  | "canvas-missing"
  | "zero-size";

export type DxSceneViewportReportInput = {
  canvas: HTMLCanvasElement | null;
  qualityProfileId: DxSceneQualityProfileId;
  performanceBand: DxScenePerformanceBand;
  rendererId: DxSceneRendererId;
  devicePixelRatio?: number;
};

export type DxSceneViewportReport = {
  status: DxSceneViewportReportStatus;
  rendererId: DxSceneRendererId;
  qualityProfileId: DxSceneQualityProfileId;
  performanceBand: DxScenePerformanceBand;
  cssWidth: number;
  cssHeight: number;
  drawingBufferWidth: number;
  drawingBufferHeight: number;
  devicePixelRatio: number;
  budgetPixelRatio: number;
  effectivePixelRatio: number;
  estimatedPixelCount: number;
  summary: string;
};

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function roundMetric(value: number) {
  return Math.round(value * 100) / 100;
}

function resolveDevicePixelRatio(input: DxSceneViewportReportInput) {
  const fromInput = input.devicePixelRatio;
  const fromWindow =
    input.canvas?.ownerDocument?.defaultView?.devicePixelRatio ?? undefined;
  const value = fromInput ?? fromWindow ?? 1;

  return Number.isFinite(value) && value > 0 ? value : 1;
}

function resolveBudgetPixelRatio(input: DxSceneViewportReportInput) {
  const quality = resolveDxSceneQualityProfile(input.qualityProfileId);
  if (input.performanceBand === "degraded") {
    return quality.minimumDevicePixelRatio;
  }
  if (input.performanceBand === "recovering") {
    return roundMetric(
      (quality.minimumDevicePixelRatio + quality.maxDevicePixelRatio) / 2,
    );
  }
  return quality.maxDevicePixelRatio;
}

export function createDxSceneViewportReport(
  input: DxSceneViewportReportInput,
): DxSceneViewportReport {
  const quality = resolveDxSceneQualityProfile(input.qualityProfileId);
  const devicePixelRatio = resolveDevicePixelRatio(input);
  const budgetPixelRatio = resolveBudgetPixelRatio(input);
  const effectivePixelRatio = roundMetric(
    clamp(devicePixelRatio, quality.minimumDevicePixelRatio, budgetPixelRatio),
  );

  if (!input.canvas) {
    return {
      status: "canvas-missing",
      rendererId: input.rendererId,
      qualityProfileId: input.qualityProfileId,
      performanceBand: input.performanceBand,
      cssWidth: 0,
      cssHeight: 0,
      drawingBufferWidth: 0,
      drawingBufferHeight: 0,
      devicePixelRatio,
      budgetPixelRatio,
      effectivePixelRatio,
      estimatedPixelCount: 0,
      summary: "canvas-missing",
    };
  }

  const cssWidth = Math.round(input.canvas.clientWidth || input.canvas.width);
  const cssHeight = Math.round(input.canvas.clientHeight || input.canvas.height);
  const drawingBufferWidth = input.canvas.width;
  const drawingBufferHeight = input.canvas.height;
  const estimatedPixelCount = Math.round(
    Math.max(0, cssWidth) *
      Math.max(0, cssHeight) *
      effectivePixelRatio *
      effectivePixelRatio,
  );
  const status = cssWidth > 0 && cssHeight > 0 ? "ready" : "zero-size";
  const summary = [
    input.rendererId,
    input.qualityProfileId,
    input.performanceBand,
    `${cssWidth}x${cssHeight}`,
    `dpr${roundMetric(devicePixelRatio)}`,
    `effectivePixelRatio${effectivePixelRatio}`,
    `buffer${drawingBufferWidth}x${drawingBufferHeight}`,
  ].join(":");

  return {
    status,
    rendererId: input.rendererId,
    qualityProfileId: input.qualityProfileId,
    performanceBand: input.performanceBand,
    cssWidth,
    cssHeight,
    drawingBufferWidth,
    drawingBufferHeight,
    devicePixelRatio: roundMetric(devicePixelRatio),
    budgetPixelRatio,
    effectivePixelRatio,
    estimatedPixelCount,
    summary,
  };
}
