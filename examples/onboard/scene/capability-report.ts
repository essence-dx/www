import type {
  DxScenePerformanceBand,
  DxSceneQualityProfileId,
  DxSceneRendererId,
} from "./types";

export type DxSceneCapabilityReportStatus =
  | "ready"
  | "canvas-missing"
  | "webgl-unavailable"
  | "context-lost"
  | "unavailable";

export type DxSceneCapabilityContextKind =
  | "webgl"
  | "experimental-webgl"
  | "unavailable";

export type DxSceneCapabilityReportInput = {
  canvas: HTMLCanvasElement | null;
  qualityProfileId: DxSceneQualityProfileId;
  performanceBand: DxScenePerformanceBand;
  rendererId: DxSceneRendererId;
};

export type DxSceneCapabilityReport = {
  status: DxSceneCapabilityReportStatus;
  rendererId: DxSceneRendererId;
  contextKind: DxSceneCapabilityContextKind;
  qualityProfileId: DxSceneQualityProfileId;
  performanceBand: DxScenePerformanceBand;
  antialias: boolean | null;
  preserveDrawingBuffer: boolean | null;
  maxTextureSize: number | null;
  maxRenderbufferSize: number | null;
  supportedExtensions: readonly string[];
  summary: string;
};

function unavailableReport(
  input: DxSceneCapabilityReportInput,
  status: DxSceneCapabilityReportStatus,
  summary: string,
): DxSceneCapabilityReport {
  return {
    status,
    rendererId: input.rendererId,
    contextKind: "unavailable",
    qualityProfileId: input.qualityProfileId,
    performanceBand: input.performanceBand,
    antialias: null,
    preserveDrawingBuffer: null,
    maxTextureSize: null,
    maxRenderbufferSize: null,
    supportedExtensions: [],
    summary,
  };
}

function getSceneContext(canvas: HTMLCanvasElement) {
  const context = canvas.getContext("webgl");
  if (context) {
    return { context, kind: "webgl" as const };
  }

  const experimentalContext = canvas.getContext(
    "experimental-webgl",
  ) as WebGLRenderingContext | null;
  if (experimentalContext) {
    return { context: experimentalContext, kind: "experimental-webgl" as const };
  }

  return null;
}

export function createDxSceneCapabilityReport(
  input: DxSceneCapabilityReportInput,
): DxSceneCapabilityReport {
  if (!input.canvas) {
    return unavailableReport(input, "canvas-missing", "canvas-missing");
  }

  try {
    const contextMatch = getSceneContext(input.canvas);
    if (!contextMatch) {
      return unavailableReport(input, "webgl-unavailable", "webgl-unavailable");
    }

    const { context, kind } = contextMatch;
    if (context.isContextLost()) {
      return unavailableReport(input, "context-lost", `${kind}:context-lost`);
    }

    const attributes = context.getContextAttributes();
    const supportedExtensions = context.getSupportedExtensions() ?? [];
    const maxTextureSize = Number(context.getParameter(context.MAX_TEXTURE_SIZE));
    const maxRenderbufferSize = Number(
      context.getParameter(context.MAX_RENDERBUFFER_SIZE),
    );
    const summary = [
      kind,
      input.rendererId,
      input.qualityProfileId,
      input.performanceBand,
      `tex${maxTextureSize}`,
      `rb${maxRenderbufferSize}`,
      `ext${supportedExtensions.length}`,
    ].join(":");

    return {
      status: "ready",
      rendererId: input.rendererId,
      contextKind: kind,
      qualityProfileId: input.qualityProfileId,
      performanceBand: input.performanceBand,
      antialias: attributes?.antialias ?? null,
      preserveDrawingBuffer: attributes?.preserveDrawingBuffer ?? null,
      maxTextureSize,
      maxRenderbufferSize,
      supportedExtensions,
      summary,
    };
  } catch {
    return unavailableReport(
      input,
      "unavailable",
      `unavailable:${input.rendererId}:${input.qualityProfileId}:${input.performanceBand}`,
    );
  }
}
