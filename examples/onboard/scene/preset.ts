import type {
  DxLaunchScenePreset,
  DxLaunchScenePresetOptions,
  DxSceneMaterialPalette,
  DxSceneMaterialPaletteId,
  DxSceneNode,
  DxSceneQualityProfile,
  DxSceneQualityProfileId,
} from "./types";

export const dxSceneMaterialPalettes: Record<
  DxSceneMaterialPaletteId,
  DxSceneMaterialPalette
> = {
  aurora: {
    id: "aurora",
    label: "Aurora launch",
    background: {
      base: [0.008, 0.012, 0.024],
      horizon: [0.06, 0.08, 0.14],
    },
    lighting: {
      ambient: [0.02, 0.04, 0.08],
      glow: [0.0, 0.95, 0.62],
      intensity: 1.25,
    },
    nodeColors: [
      [0.0, 0.95, 0.62],
      [0.24, 0.74, 1.0],
      [0.92, 0.28, 0.62],
      [1.0, 0.74, 0.24],
    ],
    material: {
      bloom: 0.72,
      contrast: 1.08,
      sheen: 0.42,
    },
  },
  graphite: {
    id: "graphite",
    label: "Graphite review",
    background: {
      base: [0.012, 0.014, 0.018],
      horizon: [0.05, 0.055, 0.07],
    },
    lighting: {
      ambient: [0.055, 0.065, 0.08],
      glow: [0.58, 0.84, 1.0],
      intensity: 1.05,
    },
    nodeColors: [
      [0.58, 0.84, 1.0],
      [0.74, 0.82, 0.94],
      [0.84, 0.6, 1.0],
      [1.0, 0.8, 0.48],
    ],
    material: {
      bloom: 0.48,
      contrast: 0.96,
      sheen: 0.58,
    },
  },
};

export function resolveDxSceneMaterialPalette(
  palette: DxSceneMaterialPaletteId | DxSceneMaterialPalette = "aurora",
): DxSceneMaterialPalette {
  return typeof palette === "string"
    ? dxSceneMaterialPalettes[palette]
    : palette;
}

export const dxSceneQualityProfiles: Record<
  DxSceneQualityProfileId,
  DxSceneQualityProfile
> = {
  preview: {
    id: "preview",
    label: "Web Preview",
    minimumDevicePixelRatio: 0.85,
    maxDevicePixelRatio: 1.5,
    fullMotionFrameInterval: 16,
    reducedMotionFrameInterval: 80,
    antialias: false,
    powerPreference: "high-performance",
    shaderDetail: 0.78,
    performance: {
      lowerFps: 44,
      upperFps: 58,
      sampleSize: 12,
      step: 0.12,
      minFactor: 0.56,
      maxFactor: 1,
      initialFactor: 1,
    },
  },
  cinematic: {
    id: "cinematic",
    label: "Cinematic",
    minimumDevicePixelRatio: 1,
    maxDevicePixelRatio: 2,
    fullMotionFrameInterval: 16,
    reducedMotionFrameInterval: 64,
    antialias: true,
    powerPreference: "high-performance",
    shaderDetail: 1,
    performance: {
      lowerFps: 50,
      upperFps: 72,
      sampleSize: 16,
      step: 0.08,
      minFactor: 0.62,
      maxFactor: 1,
      initialFactor: 1,
    },
  },
};

export function resolveDxSceneQualityProfile(
  quality: DxSceneQualityProfileId | DxSceneQualityProfile = "preview",
): DxSceneQualityProfile {
  return typeof quality === "string" ? dxSceneQualityProfiles[quality] : quality;
}

export const splineLikeLayers: DxSceneNode[] = [
  {
    id: "forge-core",
    kind: "orb",
    color: [0.0, 0.95, 0.62],
    radius: 0.42,
    orbit: 0.18,
    opacity: 0.86,
  },
  {
    id: "state-graph",
    kind: "ribbon",
    color: [0.24, 0.74, 1.0],
    radius: 0.72,
    orbit: 0.34,
    opacity: 0.62,
  },
  {
    id: "route-grid",
    kind: "grid",
    color: [0.92, 0.28, 0.62],
    radius: 1.0,
    orbit: 0.52,
    opacity: 0.42,
  },
  {
    id: "preview-signal",
    kind: "spark",
    color: [1.0, 0.74, 0.24],
    radius: 0.18,
    orbit: 0.76,
    opacity: 0.58,
  },
];

export function createDxLaunchScenePreset(
  options: DxLaunchScenePresetOptions = {},
): DxLaunchScenePreset {
  const palette = resolveDxSceneMaterialPalette(options.palette);
  const quality = resolveDxSceneQualityProfile(options.quality);

  return {
    id: "dx-www-launch-scene",
    label: "DX WWW launch scene",
    nodes: splineLikeLayers.map((node, index) => ({
      ...node,
      color: palette.nodeColors[index] ?? node.color,
    })),
    camera: {
      position: [0, 0, 5.2],
      target: [0, 0, 0],
      focalLength: 42,
    },
    background: palette.background,
    lighting: palette.lighting,
    material: palette.material,
    quality,
    controls: {
      pointerParallax: 0.18,
      reducedMotionSpeed: 0.08,
      motionSpeed: 1,
    },
  };
}
