import type {
  DxScenePerformanceBand,
  DxSceneQualityProfileId,
  DxSceneStatus,
} from "./types";

export type DxSceneFrameSampleSource =
  | "webgl"
  | "canvas-2d"
  | "canvas-missing"
  | "unavailable"
  | "status";

export type DxSceneFrameSampleStatus =
  | "captured"
  | "canvas-missing"
  | "unavailable"
  | "fallback";

export type DxSceneFrameSampleInput = {
  canvas: HTMLCanvasElement | null;
  qualityProfileId: DxSceneQualityProfileId;
  status: DxSceneStatus;
  performanceBand: DxScenePerformanceBand;
};

export type DxSceneFrameSampleResult = {
  status: DxSceneFrameSampleStatus;
  source: DxSceneFrameSampleSource;
  value: string;
};

function sampleCoordinate(size: number) {
  return Math.max(0, Math.floor(size / 2));
}

function formatRgbaFrameSample(
  pixel: Uint8Array | Uint8ClampedArray,
  qualityProfileId: DxSceneQualityProfileId,
) {
  return `rgba-${pixel[0]}-${pixel[1]}-${pixel[2]}-${pixel[3]}-${qualityProfileId}`;
}

function readWebGLFrameSample(
  canvas: HTMLCanvasElement,
  qualityProfileId: DxSceneQualityProfileId,
): DxSceneFrameSampleResult | null {
  const gl =
    canvas.getContext("webgl") ??
    (canvas.getContext("experimental-webgl") as WebGLRenderingContext | null);

  if (!gl || gl.isContextLost()) {
    return null;
  }

  const pixel = new Uint8Array(4);
  gl.readPixels(
    sampleCoordinate(canvas.width),
    sampleCoordinate(canvas.height),
    1,
    1,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    pixel,
  );

  return {
    status: "captured",
    source: "webgl",
    value: formatRgbaFrameSample(pixel, qualityProfileId),
  };
}

function readCanvas2DFrameSample(
  canvas: HTMLCanvasElement,
  qualityProfileId: DxSceneQualityProfileId,
): DxSceneFrameSampleResult | null {
  const context = canvas.getContext("2d");

  if (!context) {
    return null;
  }

  const sample = context.getImageData(
    sampleCoordinate(canvas.width),
    sampleCoordinate(canvas.height),
    1,
    1,
  ).data;

  return {
    status: "captured",
    source: "canvas-2d",
    value: formatRgbaFrameSample(sample, qualityProfileId),
  };
}

export function captureDxSceneFrameSample(
  input: DxSceneFrameSampleInput,
): DxSceneFrameSampleResult {
  const { canvas, performanceBand, qualityProfileId, status } = input;

  if (!canvas) {
    return {
      status: "canvas-missing",
      source: "canvas-missing",
      value: "canvas-missing",
    };
  }

  try {
    const webglSample = readWebGLFrameSample(canvas, qualityProfileId);
    if (webglSample) {
      return webglSample;
    }

    const canvas2DSample = readCanvas2DFrameSample(canvas, qualityProfileId);
    if (canvas2DSample) {
      return canvas2DSample;
    }
  } catch {
    return {
      status: "unavailable",
      source: "unavailable",
      value: `unavailable-${qualityProfileId}`,
    };
  }

  return {
    status: "fallback",
    source: "status",
    value: `${status}-${performanceBand}-${qualityProfileId}`,
  };
}
