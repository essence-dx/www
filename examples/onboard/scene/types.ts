export type DxSceneStatus = "booting" | "ready" | "fallback";

export type DxSceneRgb = readonly [number, number, number];
export type DxSceneVec2 = readonly [number, number];
export type DxSceneVec3 = readonly [number, number, number];

export type DxSceneNode = {
  id: string;
  kind: "orb" | "ribbon" | "grid" | "spark";
  color: DxSceneRgb;
  radius: number;
  orbit: number;
  opacity: number;
};

export type DxSceneNodeUniform = {
  radius: number;
  orbit: number;
  opacity: number;
};

export type DxSceneNodeUniforms = {
  a: DxSceneNodeUniform;
  b: DxSceneNodeUniform;
  c: DxSceneNodeUniform;
  d: DxSceneNodeUniform;
};

export type DxSceneInteractionKind = "hover" | "select" | "miss";

export type DxSceneKeyboardAction = "previous" | "next" | "select" | "clear";

export type DxSceneKeyboardBinding = {
  action: DxSceneKeyboardAction;
  keys: readonly string[];
};

export type DxSceneKeyboardOptions = {
  enabled: boolean;
  loop: boolean;
};

export type DxSceneInteractionOptions = {
  hitRadius: number;
};

export type DxSceneNodeHit = {
  node: DxSceneNode;
  nodeId: string;
  index: number;
  anchor: DxSceneVec2;
  radius: number;
  threshold: number;
  distance: number;
};

export type DxSceneNodeInteraction = {
  kind: DxSceneInteractionKind;
  node: DxSceneNode | null;
  nodeId: string | null;
  index: number | null;
  pointer: DxSceneVec2;
  anchor: DxSceneVec2 | null;
  distance: number;
};

export type DxSceneNodeInteractionCallback = (
  interaction: DxSceneNodeInteraction,
) => void;

export type DxSceneLighting = {
  ambient: DxSceneRgb;
  glow: DxSceneRgb;
  intensity: number;
};

export type DxSceneMaterialUniforms = {
  contrast: number;
  bloom: number;
  sheen: number;
};

export type DxSceneMaterialPaletteId = "aurora" | "graphite";

export type DxSceneMaterialPalette = {
  id: DxSceneMaterialPaletteId;
  label: string;
  background: DxSceneBackground;
  lighting: DxSceneLighting;
  nodeColors: readonly DxSceneRgb[];
  material: DxSceneMaterialUniforms;
};

export type DxScenePowerPreference =
  | "default"
  | "low-power"
  | "high-performance";

export type DxSceneQualityProfileId = "preview" | "cinematic";

export type DxScenePerformanceBand = "stable" | "recovering" | "degraded";

export type DxScenePerformanceRegressReason = "interaction" | "route-transition" | "manual";

export type DxScenePerformanceProfile = {
  lowerFps: number;
  upperFps: number;
  sampleSize: number;
  step: number;
  minFactor: number;
  maxFactor: number;
  initialFactor: number;
};

export type DxScenePerformanceSample = {
  fps: number;
  averageFps: number;
  factor: number;
  band: DxScenePerformanceBand;
  reason: DxScenePerformanceRegressReason;
  maxDevicePixelRatio: number;
  shaderDetail: number;
};

export type DxScenePerformanceRegression = {
  reason: DxScenePerformanceRegressReason;
  sample: DxScenePerformanceSample;
};

export type DxScenePerformanceMonitor = {
  current: () => DxScenePerformanceSample;
  sample: (delta: number) => DxScenePerformanceSample;
  regress: (reason?: DxScenePerformanceRegressReason) => DxScenePerformanceSample;
  reset: () => DxScenePerformanceSample;
};

export type DxScenePerformanceCallback = (
  sample: DxScenePerformanceSample,
) => void;

export type DxScenePerformanceRegressionCallback = (
  regression: DxScenePerformanceRegression,
) => void;

export type DxSceneQualityProfile = {
  id: DxSceneQualityProfileId;
  label: string;
  minimumDevicePixelRatio: number;
  maxDevicePixelRatio: number;
  fullMotionFrameInterval: number;
  reducedMotionFrameInterval: number;
  antialias: boolean;
  powerPreference: DxScenePowerPreference;
  shaderDetail: number;
  performance: DxScenePerformanceProfile;
};

export type DxSceneQualityUniforms = {
  detail: number;
};

export type DxSceneCamera = {
  position: DxSceneVec3;
  target: DxSceneVec3;
  focalLength: number;
};

export type DxSceneCameraUniforms = {
  position: DxSceneVec3;
  target: DxSceneVec3;
  focalLength: number;
  depth: number;
};

export type DxSceneBackground = {
  base: DxSceneRgb;
  horizon: DxSceneRgb;
};

export type DxSceneControls = {
  pointerParallax: number;
  reducedMotionSpeed: number;
  motionSpeed: number;
};

export type DxSceneRenderBudget = {
  maxDevicePixelRatio: number;
  fullMotionFrameInterval: number;
  reducedMotionFrameInterval: number;
  antialias: boolean;
  powerPreference: DxScenePowerPreference;
};

export type DxLaunchScenePreset = {
  id: string;
  label: string;
  nodes: readonly DxSceneNode[];
  camera: DxSceneCamera;
  background: DxSceneBackground;
  lighting: DxSceneLighting;
  material: DxSceneMaterialUniforms;
  quality: DxSceneQualityProfile;
  controls: DxSceneControls;
};

export type DxLaunchScenePresetOptions = {
  palette?: DxSceneMaterialPaletteId | DxSceneMaterialPalette;
  quality?: DxSceneQualityProfileId | DxSceneQualityProfile;
};

export type DxScenePreviewReadiness = {
  status: "ready" | "needs-review" | "blocked";
  renderer: "source-owned-webgl";
  quality: "premium-launch";
  requiresPackageInstall: false;
  requiresServer: false;
  shaderOwnership: "source-owned";
  nodeCount: number;
  pointerParallaxEnabled: boolean;
  reducedMotionSupported: boolean;
  warnings: readonly string[];
};

export type DxSceneMotionMode = "full" | "reduced";

export type DxSceneViewport = {
  width: number;
  height: number;
  pixelRatio: number;
};

export type DxSceneFrameState = {
  elapsed: number;
  delta: number;
  motionMode: DxSceneMotionMode;
  performance: DxScenePerformanceSample;
  pointer: DxSceneVec2;
  scene: DxLaunchScenePreset;
  viewport: DxSceneViewport;
};

export type DxSceneFrameCallback = (state: DxSceneFrameState) => void;

export type DxSceneFrameRuntime = {
  requestFrame: (callback: FrameRequestCallback) => number;
  cancelFrame: (id: number) => void;
  onStatusChange?: (status: DxSceneStatus) => void;
  onFrame?: DxSceneFrameCallback | readonly DxSceneFrameCallback[];
  onPerformanceChange?: DxScenePerformanceCallback | readonly DxScenePerformanceCallback[];
  onPerformanceRegression?: DxScenePerformanceRegressionCallback | readonly DxScenePerformanceRegressionCallback[];
  onNodeHover?: DxSceneNodeInteractionCallback | readonly DxSceneNodeInteractionCallback[];
  onNodeSelect?: DxSceneNodeInteractionCallback | readonly DxSceneNodeInteractionCallback[];
  interaction?: Partial<DxSceneInteractionOptions>;
  keyboard?: Partial<DxSceneKeyboardOptions>;
  regressOnPointerMove?: boolean;
};

export type DxSceneController = {
  dispose: () => void;
  regressPerformance: (reason?: DxScenePerformanceRegressReason) => DxScenePerformanceSample;
  resetPerformance: () => DxScenePerformanceSample;
  pickNode: (pointer: DxSceneVec2, kind?: DxSceneInteractionKind) => DxSceneNodeInteraction;
  selectNodeByIndex: (index: number | null, kind?: DxSceneInteractionKind) => DxSceneNodeInteraction;
};

export type DxSceneRendererId = "source-owned-webgl" | "three-r3f-drei";

export type DxSceneRendererSource = "source-owned" | "app-owned";

export type DxSceneRendererCapabilities = {
  webPreviewSafe: boolean;
  requiresPackageInstall: boolean;
  supportsFrameCallbacks: boolean;
  supportsPointerEvents: boolean;
  supportsKeyboardNavigation: boolean;
  supportsRendererSwap: boolean;
};

export type DxSceneRendererAdapter = {
  id: DxSceneRendererId;
  label: string;
  source: DxSceneRendererSource;
  capabilities: DxSceneRendererCapabilities;
  mount: (
    canvas: HTMLCanvasElement,
    scene: DxLaunchScenePreset,
    runtime: DxSceneFrameRuntime,
  ) => DxSceneController;
};

export type DxSceneRendererHandoff = {
  packageId: string;
  scene: DxLaunchScenePreset;
  renderer: DxSceneRendererAdapter;
  appOwnedBoundaries: readonly string[];
};

export type DxSceneR3FFrameloop = "always" | "demand" | "never";

export type DxSceneR3FRootConfig = {
  frameloop?: DxSceneR3FFrameloop;
  events?: unknown;
  onCreated?: (state: unknown) => void;
  onPointerMissed?: () => void;
};

export type DxSceneR3FRoot = {
  configure: (
    props?: DxSceneR3FRootConfig,
  ) => DxSceneR3FRoot | Promise<DxSceneR3FRoot>;
  render: (children: unknown) => unknown;
  unmount: () => void;
};

export type DxSceneR3FRootFactory = (
  canvas: HTMLCanvasElement,
) => DxSceneR3FRoot;

export type DxSceneDreiKeyboardMapEntry = {
  name: DxSceneKeyboardAction;
  keys: readonly string[];
  up?: boolean;
};

export type DxSceneR3FRendererOptions = {
  frameloop?: DxSceneR3FFrameloop;
  events?: unknown;
  onCreated?: (state: unknown) => void;
  onPointerMissed?: () => void;
  keyboardDomElement?: HTMLElement | null;
};

export type DxSceneR3FSceneElementInput = {
  packageId: string;
  scene: DxLaunchScenePreset;
  runtime: DxSceneFrameRuntime;
  keyboardMap: readonly DxSceneDreiKeyboardMapEntry[];
  keyboardDomElement: HTMLElement | null;
  controller: DxSceneController;
};

export type DxSceneR3FSceneElementFactory = (
  input: DxSceneR3FSceneElementInput,
) => unknown;

export type DxSceneR3FRendererDependencies = {
  createRoot: DxSceneR3FRootFactory;
  createSceneElement: DxSceneR3FSceneElementFactory;
};
