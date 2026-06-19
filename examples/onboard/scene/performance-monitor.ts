import type {
  DxLaunchScenePreset,
  DxScenePerformanceBand,
  DxScenePerformanceMonitor,
  DxScenePerformanceProfile,
  DxScenePerformanceRegressReason,
  DxScenePerformanceSample,
} from "./types";

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

function average(values: readonly number[]) {
  if (values.length === 0) {
    return 0;
  }
  return values.reduce((total, value) => total + value, 0) / values.length;
}

function resolvePerformanceBand(
  factor: number,
  averageFps: number,
  profile: DxScenePerformanceProfile,
): DxScenePerformanceBand {
  if (averageFps > 0 && averageFps < profile.lowerFps) {
    return "degraded";
  }
  return factor < profile.maxFactor ? "recovering" : "stable";
}

export function createDxSceneAdaptiveRenderBudget(
  scene: DxLaunchScenePreset,
  factor: number,
  fps = 0,
  averageFps = 0,
  reason: DxScenePerformanceRegressReason = "manual",
): DxScenePerformanceSample {
  const profile = scene.quality.performance;
  const resolvedFactor = clamp(factor, profile.minFactor, profile.maxFactor);
  const devicePixelRange =
    scene.quality.maxDevicePixelRatio - scene.quality.minimumDevicePixelRatio;
  const maxDevicePixelRatio =
    scene.quality.minimumDevicePixelRatio + devicePixelRange * resolvedFactor;

  return {
    fps,
    averageFps,
    factor: resolvedFactor,
    band: resolvePerformanceBand(resolvedFactor, averageFps, profile),
    reason,
    maxDevicePixelRatio,
    shaderDetail: clamp(0.7 + resolvedFactor * 0.3, 0.7, 1),
  };
}

export function createDxScenePerformanceMonitor(
  scene: DxLaunchScenePreset,
): DxScenePerformanceMonitor {
  const profile = scene.quality.performance;
  const frames: number[] = [];
  let factor = clamp(
    profile.initialFactor,
    profile.minFactor,
    profile.maxFactor,
  );
  let lastRegressionReason: DxScenePerformanceRegressReason = "manual";
  let currentSample = createDxSceneAdaptiveRenderBudget(
    scene,
    factor,
    0,
    0,
    lastRegressionReason,
  );

  const updateSample = (fps: number, averageFps: number) => {
    if (averageFps < profile.lowerFps) {
      factor = clamp(factor - profile.step, profile.minFactor, profile.maxFactor);
    } else if (averageFps > profile.upperFps) {
      factor = clamp(factor + profile.step, profile.minFactor, profile.maxFactor);
    }
    currentSample = createDxSceneAdaptiveRenderBudget(
      scene,
      factor,
      fps,
      averageFps,
      lastRegressionReason,
    );
    return currentSample;
  };

  return {
    current: () => currentSample,
    sample(delta: number) {
      if (delta <= 0) {
        return currentSample;
      }

      const fps = Math.round(1 / delta);
      frames.push(fps);
      if (frames.length > profile.sampleSize) {
        frames.shift();
      }

      return updateSample(fps, average(frames));
    },
    regress(reason: DxScenePerformanceRegressReason = "manual") {
      frames.length = 0;
      lastRegressionReason = reason;
      factor = profile.minFactor;
      currentSample = createDxSceneAdaptiveRenderBudget(
        scene,
        factor,
        0,
        profile.lowerFps - 1,
        lastRegressionReason,
      );
      return currentSample;
    },
    reset() {
      frames.length = 0;
      lastRegressionReason = "manual";
      factor = profile.maxFactor;
      currentSample = createDxSceneAdaptiveRenderBudget(
        scene,
        factor,
        0,
        0,
        lastRegressionReason,
      );
      return currentSample;
    },
  };
}
