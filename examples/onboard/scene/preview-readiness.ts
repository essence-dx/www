import type {
  DxLaunchScenePreset,
  DxScenePreviewReadiness,
  DxSceneRgb,
} from "./types";

export type DxSceneFallbackStyle = {
  "--dx-scene-base": string;
  "--dx-scene-horizon": string;
  "--dx-scene-accent": string;
  "--dx-scene-secondary": string;
  "--dx-scene-surface": string;
  "--dx-scene-layer": string;
  "--dx-scene-text": string;
  "--dx-scene-muted": string;
  "--dx-scene-accent-text": string;
  "--dx-scene-success": string;
  "--dx-scene-border": string;
  "--dx-scene-control": string;
  "--dx-scene-control-border": string;
  "--dx-scene-control-hover": string;
  "--dx-scene-control-hover-border": string;
  "--dx-scene-chip": string;
  "--dx-scene-shadow": string;
  background: string;
};

function readinessBase(scene: DxLaunchScenePreset) {
  return {
    renderer: "source-owned-webgl",
    quality: "premium-launch",
    requiresPackageInstall: false,
    requiresServer: false,
    shaderOwnership: "source-owned",
    nodeCount: scene.nodes.length,
    pointerParallaxEnabled: scene.controls.pointerParallax > 0,
    reducedMotionSupported: scene.controls.reducedMotionSpeed < scene.controls.motionSpeed,
  } as const;
}

export function createDxScenePreviewReadiness(
  scene: DxLaunchScenePreset,
): DxScenePreviewReadiness {
  const warnings: string[] = [];

  if (scene.nodes.length < 3) {
    warnings.push("Scene should expose at least three visual layers.");
  }

  if (scene.lighting.intensity <= 0) {
    warnings.push("Scene lighting intensity must be positive.");
  }

  if (scene.controls.reducedMotionSpeed >= scene.controls.motionSpeed) {
    warnings.push("Reduced-motion speed should be lower than standard motion speed.");
  }

  if (warnings.length === 0) {
    return {
      ...readinessBase(scene),
      status: "ready",
      warnings: [],
    };
  }

  return {
    ...readinessBase(scene),
    status: "needs-review",
    warnings,
  };
}

export function createDxSceneFallbackStyle(
  scene: DxLaunchScenePreset,
): DxSceneFallbackStyle {
  const accent = scene.nodes[0]?.color ?? scene.lighting.glow;
  const secondary = scene.nodes[1]?.color ?? scene.background.horizon;

  return {
    "--dx-scene-base": rgb(scene.background.base),
    "--dx-scene-horizon": rgb(scene.background.horizon),
    "--dx-scene-accent": rgb(accent),
    "--dx-scene-secondary": rgb(secondary),
    "--dx-scene-surface": rgb(scene.background.base),
    "--dx-scene-layer": [
      `radial-gradient(circle at 78% 12%, ${rgba(accent, 0.2)}, transparent 26%)`,
      `linear-gradient(180deg, ${rgba(scene.background.base, 0.08)}, ${rgba(scene.background.base, 0.9)})`,
    ].join(", "),
    "--dx-scene-text": rgba(scene.lighting.glow, 0.92),
    "--dx-scene-muted": rgba(scene.lighting.glow, 0.64),
    "--dx-scene-accent-text": rgba(accent, 0.9),
    "--dx-scene-success": rgba(secondary, 0.82),
    "--dx-scene-border": rgba(accent, 0.28),
    "--dx-scene-control": rgba(scene.background.base, 0.76),
    "--dx-scene-control-border": rgba(accent, 0.42),
    "--dx-scene-control-hover": rgba(accent, 0.12),
    "--dx-scene-control-hover-border": rgba(secondary, 0.58),
    "--dx-scene-chip": rgba(scene.background.horizon, 0.52),
    "--dx-scene-shadow": `0 24px 80px ${rgba(scene.background.base, 0.42)}`,
    background: [
      `radial-gradient(circle at 32% 24%, ${rgba(accent, 0.38)}, transparent 30%)`,
      `radial-gradient(circle at 78% 16%, ${rgba(secondary, 0.32)}, transparent 28%)`,
      `linear-gradient(135deg, ${rgb(scene.background.base)}, ${rgb(scene.background.horizon)})`,
    ].join(", "),
  };
}

function rgb(color: DxSceneRgb) {
  return `rgb(${channel(color[0])} ${channel(color[1])} ${channel(color[2])})`;
}

function rgba(color: DxSceneRgb, alpha: number) {
  return `rgb(${channel(color[0])} ${channel(color[1])} ${channel(color[2])} / ${alpha})`;
}

function channel(value: number) {
  return Math.max(0, Math.min(255, Math.round(value * 255)));
}
