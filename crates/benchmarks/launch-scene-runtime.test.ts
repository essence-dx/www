const assert = require("assert");
const { execFileSync } = require("node:child_process");
const fs = require("fs");
const os = require("node:os");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("launch scene runtime handles WebGL context loss", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );

  assert.match(runtime, /webglcontextlost/);
  assert.match(runtime, /event\.preventDefault\(\)/);
  assert.match(runtime, /runtime\.onStatusChange\?\.\("fallback"\)/);
  assert.match(runtime, /webglcontextrestored/);
  assert.match(runtime, /releaseResources\(\)/);
  assert.match(runtime, /removeEventListener\("webglcontextlost", onContextLost\)/);
  assert.match(
    runtime,
    /removeEventListener\("webglcontextrestored", onContextRestored\)/,
  );
});

test("launch scene runtime recenters pointer parallax when pointer ownership ends", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );

  assert.match(runtime, /const resetPointer = \(\) => {/);
  assert.match(runtime, /pointer\.x = 0;/);
  assert.match(runtime, /pointer\.y = 0;/);
  assert.match(runtime, /addEventListener\("pointerleave", resetPointer\)/);
  assert.match(runtime, /addEventListener\("lostpointercapture", resetPointer\)/);
  assert.match(runtime, /addEventListener\("blur", resetPointer\)/);
  assert.match(runtime, /removeEventListener\("pointerleave", resetPointer\)/);
  assert.match(
    runtime,
    /removeEventListener\("lostpointercapture", resetPointer\)/,
  );
  assert.match(runtime, /removeEventListener\("blur", resetPointer\)/);
});

test("launch scene runtime falls back to experimental WebGL with preview-safe attributes", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );

  assert.match(runtime, /function getWebGLContext/);
  assert.match(runtime, /WebGLContextAttributes/);
  assert.match(runtime, /powerPreference: budget\.powerPreference/);
  assert.match(runtime, /preserveDrawingBuffer: false/);
  assert.match(runtime, /getContext\("webgl", contextAttributes\)/);
  assert.match(
    runtime,
    /getContext\(\s*"experimental-webgl",\s*contextAttributes,?\s*\)/,
  );
  assert.match(runtime, /const gl = getWebGLContext\(canvas, budget\);/);
});

test("launch scene runtime pauses rendering while document is hidden", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );

  assert.match(runtime, /let frameScheduled = false;/);
  assert.match(runtime, /const requestNextFrame = \(\) => {/);
  assert.match(runtime, /document\.hidden/);
  assert.match(runtime, /const cancelScheduledFrame = \(\) => {/);
  assert.match(runtime, /runtime\.cancelFrame\(frame\)/);
  assert.match(runtime, /const onVisibilityChange = \(\) => {/);
  assert.match(runtime, /document\.addEventListener\("visibilitychange", onVisibilityChange\)/);
  assert.match(
    runtime,
    /document\.removeEventListener\("visibilitychange", onVisibilityChange\)/,
  );
  assert.match(runtime, /requestNextFrame\(\);/);
});

test("launch scene runtime enters fallback when WebGL context is already lost", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );

  assert.match(runtime, /let fallbackReported = false;/);
  assert.match(runtime, /const enterFallback = \(\) => {/);
  assert.match(runtime, /if \(fallbackReported\) {/);
  assert.match(runtime, /fallbackReported = true;/);
  assert.match(runtime, /if \(gl\.isContextLost\(\)\) {/);
  assert.match(runtime, /enterFallback\(\);/);
  assert.match(runtime, /const onContextLost = \(event: Event\) => {/);
  assert.match(runtime, /event\.preventDefault\(\);[\s\S]*enterFallback\(\);/);
});

test("launch scene exposes a source-owned frame callback bridge", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneFrameState = {/);
  assert.match(types, /export type DxSceneFrameCallback = \(state: DxSceneFrameState\) => void;/);
  assert.match(types, /onFrame\?: DxSceneFrameCallback \| readonly DxSceneFrameCallback\[\];/);
  assert.match(index, /DxSceneFrameCallback/);
  assert.match(index, /DxSceneFrameState/);

  assert.match(runtime, /function normalizeDxSceneFrameCallbacks/);
  assert.match(runtime, /const frameCallbacks = normalizeDxSceneFrameCallbacks\(runtime\.onFrame\);/);
  assert.match(runtime, /for \(const callback of frameCallbacks\) {/);
  assert.match(runtime, /const motionMode = reduceMotion\.matches \? "reduced" : "full";/);
  assert.match(runtime, /motionMode,/);
  assert.match(runtime, /pointer: \[pointer\.x, pointer\.y\]/);
  assert.match(runtime, /viewport,/);

  assert.match(launchScene, /const \[motionMode, setMotionMode\]/);
  assert.match(launchScene, /onFrame:/);
  assert.match(launchScene, /data-dx-scene-motion-mode=\{motionMode\}/);
  assert.match(readme, /Frame callback bridge/);
  assert.match(readme, /onFrame/);
});

test("launch scene runtime drives shader nodes from preset geometry fields", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneNodeUniform = {/);
  assert.match(types, /export type DxSceneNodeUniforms = {/);
  assert.match(index, /DxSceneNodeUniform/);
  assert.match(index, /DxSceneNodeUniforms/);

  assert.match(runtime, /uniform vec3 nodeA;/);
  assert.match(runtime, /uniform vec3 nodeB;/);
  assert.match(runtime, /uniform vec3 nodeC;/);
  assert.match(runtime, /uniform vec3 nodeD;/);
  assert.match(runtime, /export function createDxSceneNodeUniforms/);
  assert.match(runtime, /scene\.nodes\[index\]\?\.radius/);
  assert.match(runtime, /scene\.nodes\[index\]\?\.orbit/);
  assert.match(runtime, /scene\.nodes\[index\]\?\.opacity/);
  assert.match(runtime, /const nodeUniforms = createDxSceneNodeUniforms\(scene\);/);
  assert.match(runtime, /gl\.getUniformLocation\(program\.program, "nodeA"\)/);
  assert.match(runtime, /gl\.uniform3f\(location, node\.radius, node\.orbit, node\.opacity\);/);
  assert.match(runtime, /nodeA\.x/);
  assert.match(runtime, /nodeB\.y/);
  assert.match(runtime, /nodeC\.z/);
  assert.match(runtime, /nodeD\.z/);
  assert.match(readme, /Node geometry uniforms/);
});

test("launch scene runtime drives shader camera uniforms from the preset camera", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneCameraUniforms = {/);
  assert.match(types, /focalLength: number;/);
  assert.match(types, /depth: number;/);
  assert.match(index, /DxSceneCameraUniforms/);
  assert.match(index, /createDxSceneCameraUniforms/);

  assert.match(runtime, /uniform vec3 cameraPosition;/);
  assert.match(runtime, /uniform vec3 cameraTarget;/);
  assert.match(runtime, /uniform float cameraFocalLength;/);
  assert.match(runtime, /uniform float cameraDepth;/);
  assert.match(runtime, /export function createDxSceneCameraUniforms/);
  assert.match(runtime, /scene\.camera\.position/);
  assert.match(runtime, /scene\.camera\.target/);
  assert.match(runtime, /scene\.camera\.focalLength/);
  assert.match(runtime, /const cameraUniforms = createDxSceneCameraUniforms\(scene\);/);
  assert.match(runtime, /gl\.getUniformLocation\(program\.program, "cameraPosition"\)/);
  assert.match(runtime, /gl\.uniform1f\(cameraFocalLength, cameraUniforms\.focalLength\);/);
  assert.match(runtime, /gl\.uniform1f\(cameraDepth, cameraUniforms\.depth\);/);
  assert.match(runtime, /cameraPosition\.xy/);
  assert.match(runtime, /cameraTarget\.xy/);
  assert.match(runtime, /cameraFocalLength \/ 42\.0/);
  assert.match(runtime, /cameraDepth \/ 5\.2/);
  assert.match(metadata, /"camera-uniforms"/);
  assert.match(readme, /Camera uniforms/);
});

test("launch scene exposes material palettes as real shader input", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const preset = readRequiredFile("examples/template/scene/preset.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneMaterialUniforms = {/);
  assert.match(types, /contrast: number;/);
  assert.match(types, /bloom: number;/);
  assert.match(types, /sheen: number;/);
  assert.match(types, /export type DxSceneMaterialPalette = {/);
  assert.match(types, /export type DxLaunchScenePresetOptions = {/);
  assert.match(types, /material: DxSceneMaterialUniforms;/);

  assert.match(preset, /export const dxSceneMaterialPalettes/);
  assert.match(preset, /aurora/);
  assert.match(preset, /graphite/);
  assert.match(preset, /export function resolveDxSceneMaterialPalette/);
  assert.match(preset, /nodeColors/);
  assert.match(preset, /DxLaunchScenePresetOptions/);
  assert.match(preset, /resolveDxSceneMaterialPalette\(options\.palette\)/);
  assert.match(preset, /palette\.material/);
  assert.match(preset, /palette\.nodeColors\[index\]/);

  assert.match(index, /dxSceneMaterialPalettes/);
  assert.match(index, /resolveDxSceneMaterialPalette/);
  assert.match(index, /createDxSceneMaterialUniforms/);
  assert.match(index, /DxSceneMaterialPalette/);
  assert.match(index, /DxSceneMaterialUniforms/);
  assert.match(index, /DxLaunchScenePresetOptions/);

  assert.match(runtime, /uniform float materialContrast;/);
  assert.match(runtime, /uniform float materialBloom;/);
  assert.match(runtime, /uniform float materialSheen;/);
  assert.match(runtime, /export function createDxSceneMaterialUniforms/);
  assert.match(runtime, /const materialUniforms = createDxSceneMaterialUniforms\(scene\);/);
  assert.match(runtime, /gl\.uniform1f\(materialContrast, materialUniforms\.contrast\);/);
  assert.match(runtime, /gl\.uniform1f\(materialBloom, materialUniforms\.bloom\);/);
  assert.match(runtime, /gl\.uniform1f\(materialSheen, materialUniforms\.sheen\);/);
  assert.match(runtime, /materialContrast/);
  assert.match(runtime, /materialBloom/);
  assert.match(runtime, /materialSheen/);

  assert.match(launchScene, /dxSceneMaterialPalettes\[materialPaletteId\]/);
  assert.match(launchScene, /palette: materialPaletteId/);
  assert.match(launchScene, /data-dx-scene-action="cycle-material-palette"/);
  assert.match(launchScene, /data-dx-scene-material-palette=\{materialPaletteId\}/);
  assert.match(metadata, /"material-palettes"/);
  assert.match(metadata, /"material-palette-dashboard-action"/);
  assert.match(readme, /Material palettes/);
});

test("launch scene exposes quality profiles for Web Preview render budgets", () => {
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const preset = readRequiredFile("examples/template/scene/preset.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneQualityProfileId = "preview" \| "cinematic";/);
  assert.match(types, /export type DxSceneQualityProfile = {/);
  assert.match(types, /maxDevicePixelRatio: number;/);
  assert.match(types, /powerPreference: DxScenePowerPreference;/);
  assert.match(types, /shaderDetail: number;/);
  assert.match(types, /export type DxSceneQualityUniforms = {/);
  assert.match(types, /quality: DxSceneQualityProfile;/);
  assert.match(types, /quality\?: DxSceneQualityProfileId \| DxSceneQualityProfile;/);

  assert.match(preset, /export const dxSceneQualityProfiles/);
  assert.match(preset, /preview/);
  assert.match(preset, /cinematic/);
  assert.match(preset, /export function resolveDxSceneQualityProfile/);
  assert.match(preset, /resolveDxSceneQualityProfile\(options\.quality\)/);
  assert.match(preset, /quality,/);

  assert.match(index, /dxSceneQualityProfiles/);
  assert.match(index, /resolveDxSceneQualityProfile/);
  assert.match(index, /createDxSceneQualityUniforms/);
  assert.match(index, /DxSceneQualityProfile/);
  assert.match(index, /DxSceneQualityUniforms/);

  assert.match(runtime, /uniform float qualityDetail;/);
  assert.match(runtime, /export function createDxSceneQualityUniforms/);
  assert.match(runtime, /detail: scene\.quality\.shaderDetail/);
  assert.match(runtime, /maxDevicePixelRatio: scene\.quality\.maxDevicePixelRatio/);
  assert.match(runtime, /fullMotionFrameInterval: scene\.quality\.fullMotionFrameInterval/);
  assert.match(runtime, /reducedMotionFrameInterval: scene\.quality\.reducedMotionFrameInterval/);
  assert.match(runtime, /antialias: scene\.quality\.antialias/);
  assert.match(runtime, /powerPreference: scene\.quality\.powerPreference/);
  assert.match(runtime, /powerPreference: budget\.powerPreference/);
  assert.match(runtime, /const qualityUniforms = createDxSceneQualityUniforms\(scene\);/);
  assert.match(runtime, /qualityUniforms\.detail \* performanceSample\.shaderDetail/);
  assert.match(runtime, /qualityDetail/);

  assert.match(launchScene, /quality: "preview"/);
  assert.match(launchScene, /quality: qualityProfileId/);
  assert.match(metadata, /"quality-profiles"/);
  assert.match(readme, /Quality profiles/);
});

test("launch scene exposes a source-owned adaptive performance monitor", () => {
  const monitor = readRequiredFile(
    "examples/template/scene/performance-monitor.ts",
  );
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const preset = readRequiredFile("examples/template/scene/preset.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxScenePerformanceBand = "stable" \| "recovering" \| "degraded";/);
  assert.match(types, /export type DxScenePerformanceProfile = {/);
  assert.match(types, /lowerFps: number;/);
  assert.match(types, /upperFps: number;/);
  assert.match(types, /sampleSize: number;/);
  assert.match(types, /initialFactor: number;/);
  assert.match(types, /export type DxScenePerformanceSample = {/);
  assert.match(types, /averageFps: number;/);
  assert.match(types, /maxDevicePixelRatio: number;/);
  assert.match(types, /shaderDetail: number;/);
  assert.match(types, /export type DxScenePerformanceMonitor = {/);
  assert.match(types, /regress: \(reason\?: DxScenePerformanceRegressReason\) => DxScenePerformanceSample;/);
  assert.match(types, /performance: DxScenePerformanceProfile;/);
  assert.match(types, /performance: DxScenePerformanceSample;/);
  assert.match(types, /onPerformanceChange\?: DxScenePerformanceCallback \| readonly DxScenePerformanceCallback\[\];/);
  assert.match(types, /minimumDevicePixelRatio: number;/);

  assert.match(preset, /minimumDevicePixelRatio/);
  assert.match(preset, /performance: {/);
  assert.match(preset, /lowerFps/);
  assert.match(preset, /upperFps/);

  assert.match(monitor, /export function createDxSceneAdaptiveRenderBudget/);
  assert.match(monitor, /export function createDxScenePerformanceMonitor/);
  assert.match(monitor, /scene\.quality\.minimumDevicePixelRatio/);
  assert.match(monitor, /scene\.quality\.maxDevicePixelRatio/);
  assert.match(monitor, /scene\.quality\.performance/);
  assert.match(monitor, /sample\(delta: number\)/);
  assert.match(
    monitor,
    /regress\(reason: DxScenePerformanceRegressReason = "manual"\)/,
  );

  assert.match(index, /createDxSceneAdaptiveRenderBudget/);
  assert.match(index, /createDxScenePerformanceMonitor/);
  assert.match(index, /DxScenePerformanceMonitor/);
  assert.match(index, /DxScenePerformanceSample/);

  assert.match(runtime, /createDxScenePerformanceMonitor/);
  assert.match(runtime, /function normalizeDxScenePerformanceCallbacks/);
  assert.match(runtime, /const performanceMonitor = createDxScenePerformanceMonitor\(scene\);/);
  assert.match(runtime, /let performanceSample = performanceMonitor\.current\(\);/);
  assert.match(runtime, /performanceSample = performanceMonitor\.sample\(delta\);/);
  assert.match(runtime, /const adaptiveMaxDevicePixelRatio = Math\.min\(/);
  assert.match(runtime, /budget\.maxDevicePixelRatio/);
  assert.match(runtime, /performanceSample\.maxDevicePixelRatio/);
  assert.match(runtime, /resizeCanvas\(canvas, adaptiveMaxDevicePixelRatio\)/);
  assert.match(runtime, /qualityUniforms\.detail \* performanceSample\.shaderDetail/);
  assert.match(runtime, /notifyPerformanceChange\(performanceSample\);/);
  assert.match(runtime, /performance: performanceSample,/);

  assert.match(launchScene, /const \[performanceBand, setPerformanceBand\]/);
  assert.match(launchScene, /onPerformanceChange:/);
  assert.match(launchScene, /data-dx-scene-performance-band=\{performanceBand\}/);
  assert.match(metadata, /"lib\/scene\/performance-monitor\.ts"/);
  assert.match(metadata, /"performance-monitor"/);
  assert.match(readme, /Performance monitor/);
});

test("launch scene exposes source-owned performance regression controls", () => {
  const monitor = readRequiredFile(
    "examples/template/scene/performance-monitor.ts",
  );
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(
    types,
    /export type DxScenePerformanceRegressReason = "interaction" \| "route-transition" \| "manual";/,
  );
  assert.match(types, /export type DxScenePerformanceRegression = {/);
  assert.match(types, /reason: DxScenePerformanceRegressReason;/);
  assert.match(types, /sample: DxScenePerformanceSample;/);
  assert.match(types, /export type DxScenePerformanceRegressionCallback = \(/);
  assert.match(types, /regressPerformance: \(reason\?: DxScenePerformanceRegressReason\) => DxScenePerformanceSample;/);
  assert.match(types, /resetPerformance: \(\) => DxScenePerformanceSample;/);
  assert.match(types, /regressOnPointerMove\?: boolean;/);
  assert.match(types, /onPerformanceRegression\?: DxScenePerformanceRegressionCallback \| readonly DxScenePerformanceRegressionCallback\[\];/);

  assert.match(monitor, /regress\(reason: DxScenePerformanceRegressReason = "manual"\)/);
  assert.match(monitor, /lastRegressionReason/);
  assert.match(monitor, /lastRegressionReason = reason/);
  assert.match(monitor, /lastRegressionReason,\s*\)/);
  assert.match(index, /DxScenePerformanceRegressReason/);
  assert.match(index, /DxScenePerformanceRegression/);
  assert.match(index, /DxScenePerformanceRegressionCallback/);

  assert.match(runtime, /function normalizeDxScenePerformanceRegressionCallbacks/);
  assert.match(runtime, /runtime\.onPerformanceRegression/);
  assert.match(runtime, /const notifyPerformanceRegression = \(/);
  assert.match(
    runtime,
    /const regressPerformance = \(\s*reason: DxScenePerformanceRegressReason = "manual",?\s*\) => {/,
  );
  assert.match(runtime, /performanceSample = performanceMonitor\.regress\(reason\);/);
  assert.match(runtime, /notifyPerformanceRegression\(\{ reason, sample: performanceSample \}\);/);
  assert.match(runtime, /const resetPerformance = \(\) => {/);
  assert.match(runtime, /performanceSample = performanceMonitor\.reset\(\);/);
  assert.match(runtime, /if \(runtime\.regressOnPointerMove\) {/);
  assert.match(runtime, /regressPerformance\("interaction"\);/);
  assert.match(runtime, /regressPerformance,/);
  assert.match(runtime, /resetPerformance,/);

  assert.match(launchScene, /DxScenePerformanceRegressReason/);
  assert.match(launchScene, /const \[regressionReason, setRegressionReason\]/);
  assert.match(launchScene, /regressOnPointerMove: true/);
  assert.match(launchScene, /onPerformanceRegression:/);
  assert.match(launchScene, /setRegressionReason\(regression\.reason\)/);
  assert.match(launchScene, /data-dx-scene-regression-reason=\{regressionReason\}/);
  assert.match(metadata, /"performance-regression-controls"/);
  assert.match(readme, /Performance regression controls/);
  assert.match(readme, /route-transition/);
});

test("launch scene exposes a source-owned node interaction bridge", () => {
  const interaction = readRequiredFile(
    "examples/template/scene/interaction.ts",
  );
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneInteractionKind = "hover" \| "select" \| "miss";/);
  assert.match(types, /export type DxSceneInteractionOptions = {/);
  assert.match(types, /hitRadius: number;/);
  assert.match(types, /export type DxSceneNodeHit = {/);
  assert.match(types, /export type DxSceneNodeInteraction = {/);
  assert.match(types, /nodeId: string \| null;/);
  assert.match(types, /export type DxSceneNodeInteractionCallback = \(/);
  assert.match(types, /onNodeHover\?: DxSceneNodeInteractionCallback \| readonly DxSceneNodeInteractionCallback\[\];/);
  assert.match(types, /onNodeSelect\?: DxSceneNodeInteractionCallback \| readonly DxSceneNodeInteractionCallback\[\];/);
  assert.match(types, /pickNode: \(pointer: DxSceneVec2, kind\?: DxSceneInteractionKind\) => DxSceneNodeInteraction;/);

  assert.match(interaction, /export const defaultDxSceneInteractionOptions/);
  assert.match(interaction, /export function createDxSceneInteractionMap/);
  assert.match(interaction, /export function pickDxSceneNode/);
  assert.match(interaction, /export function createDxSceneNodeInteraction/);
  assert.match(interaction, /distance <= hitRadius \+ candidate\.radius/);

  assert.match(index, /defaultDxSceneInteractionOptions/);
  assert.match(index, /createDxSceneInteractionMap/);
  assert.match(index, /pickDxSceneNode/);
  assert.match(index, /DxSceneNodeInteractionCallback/);

  assert.match(runtime, /createDxSceneNodeInteraction/);
  assert.match(runtime, /function normalizeDxSceneNodeInteractionCallbacks/);
  assert.match(runtime, /runtime\.onNodeHover/);
  assert.match(runtime, /runtime\.onNodeSelect/);
  assert.match(runtime, /const pickNode = \(/);
  assert.match(runtime, /const notifyNodeHover = \(/);
  assert.match(runtime, /const notifyNodeSelect = \(/);
  assert.match(runtime, /const onClick = \(\) => {/);
  assert.match(runtime, /const interaction = pickNode\(lastPointer, "select"\);/);
  assert.match(runtime, /focusedNodeIndex = interaction\.index;/);
  assert.match(runtime, /notifyNodeSelect\(interaction\);/);
  assert.match(runtime, /canvas\.addEventListener\("click", onClick\)/);
  assert.match(runtime, /canvas\.removeEventListener\("click", onClick\)/);

  assert.match(launchScene, /const \[activeNodeId, setActiveNodeId\]/);
  assert.match(launchScene, /const syncNodeInteraction = React\.useCallback/);
  assert.match(launchScene, /onNodeHover:/);
  assert.match(launchScene, /onNodeSelect:/);
  assert.match(launchScene, /const nodeId = interaction\.nodeId \?\? "none";/);
  assert.match(launchScene, /setActiveNodeId\(nodeId\)/);
  assert.match(launchScene, /data-dx-scene-active-node=\{activeNodeId\}/);
  assert.match(launchScene, /pointer-events-none/);
  assert.match(metadata, /"node-interaction-bridge"/);
  assert.match(readme, /Node interaction bridge/);
  assert.match(readme, /R3F-style pointer\s+events/);
});

test("launch scene exposes source-owned keyboard node navigation", () => {
  const interaction = readRequiredFile(
    "examples/template/scene/interaction.ts",
  );
  const runtime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneKeyboardAction = "previous" \| "next" \| "select" \| "clear";/);
  assert.match(types, /export type DxSceneKeyboardBinding = {/);
  assert.match(types, /action: DxSceneKeyboardAction;/);
  assert.match(types, /keys: readonly string\[\];/);
  assert.match(types, /export type DxSceneKeyboardOptions = {/);
  assert.match(types, /loop: boolean;/);
  assert.match(types, /keyboard\?: Partial<DxSceneKeyboardOptions>;/);
  assert.match(types, /selectNodeByIndex: \(index: number \| null, kind\?: DxSceneInteractionKind\) => DxSceneNodeInteraction;/);

  assert.match(interaction, /export const dxSceneKeyboardBindings/);
  assert.match(interaction, /action: "previous"/);
  assert.match(interaction, /ArrowLeft/);
  assert.match(interaction, /action: "next"/);
  assert.match(interaction, /ArrowRight/);
  assert.match(interaction, /action: "select"/);
  assert.match(interaction, /Enter/);
  assert.match(interaction, /action: "clear"/);
  assert.match(interaction, /Escape/);
  assert.match(interaction, /export function resolveDxSceneKeyboardAction/);
  assert.match(interaction, /export function createDxSceneKeyboardInteraction/);

  assert.match(index, /dxSceneKeyboardBindings/);
  assert.match(index, /resolveDxSceneKeyboardAction/);
  assert.match(index, /createDxSceneKeyboardInteraction/);
  assert.match(index, /DxSceneKeyboardAction/);

  assert.match(runtime, /resolveDxSceneKeyboardAction/);
  assert.match(runtime, /createDxSceneKeyboardInteraction/);
  assert.match(runtime, /const selectNodeByIndex = \(/);
  assert.match(runtime, /const onKeyDown = \(event: KeyboardEvent\) => {/);
  assert.match(runtime, /const action = resolveDxSceneKeyboardAction\(event\.key\);/);
  assert.match(runtime, /event\.preventDefault\(\);/);
  assert.match(runtime, /notifyNodeHover\(selectNodeByIndex\(focusedNodeIndex, "hover"\)\);/);
  assert.match(runtime, /notifyNodeSelect\(selectNodeByIndex\(focusedNodeIndex, "select"\)\);/);
  assert.match(runtime, /canvas\.addEventListener\("keydown", onKeyDown\)/);
  assert.match(runtime, /canvas\.removeEventListener\("keydown", onKeyDown\)/);
  assert.match(runtime, /selectNodeByIndex,/);

  assert.match(launchScene, /keyboard: \{ enabled: true, loop: true \}/);
  assert.match(launchScene, /tabIndex=\{0\}/);
  assert.match(launchScene, /role="img"/);
  assert.match(launchScene, /data-dx-scene-keyboard="node-navigation"/);
  assert.match(metadata, /"keyboard-node-navigation"/);
  assert.match(readme, /Keyboard node navigation/);
  assert.match(readme, /Drei\s+KeyboardControls/);
});

test("launch scene is visibly demoable from the generated launch route", () => {
  const launchShell = readRequiredFile(
    "examples/template/template-shell.tsx",
  );
  const launchScene = readRequiredFile(
    "examples/template/launch-scene.tsx",
  );

  assert.match(launchShell, /data-dx-media-slot="launch-scene"/);
  assert.match(launchShell, /<LaunchScene \/>/);
  assert.match(launchScene, /data-dx-package="3d\/launch-scene"/);
  assert.match(launchScene, /data-dx-component="launch-scene-webgl-proof"/);
  assert.match(launchScene, /data-dx-scene-canvas/);
  assert.match(launchScene, /data-dx-scene-workflow-controls/);
  assert.match(launchScene, /data-dx-scene-workflow-selected-node=\{activeNodeId\}/);
  assert.match(launchScene, /data-dx-scene-workflow-action="select-node"/);
  assert.match(launchScene, /data-dx-scene-workflow-action="regress-performance"/);
  assert.match(launchScene, /data-dx-scene-workflow-action="reset-performance"/);
  assert.match(launchScene, /controllerRef\.current\?\.selectNodeByIndex/);
  assert.match(launchScene, /controllerRef\.current\?\.regressPerformance\("manual"\)/);
  assert.match(launchScene, /controllerRef\.current\?\.resetPerformance\(\)/);
});

test("launch scene is materialized into the no-node live launch runtime", () => {
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-scene-"));

  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);
  const launchPage = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const runtime = fs.readFileSync(
    path.join(dir, "public", "launch-runtime.js"),
    "utf8",
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.match(launchPage, /data-dx-package="3d\/launch-scene"/);
  assert.match(launchPage, /data-dx-component="launch-scene-webgl-proof"/);
  assert.match(launchPage, /data-dx-media-slot="launch-scene"/);
  assert.match(launchPage, /data-dx-scene-canvas/);
  assert.match(launchPage, /data-dx-scene-workflow-controls/);
  assert.match(launchPage, /data-dx-scene-workflow-selected-node="none"/);
  assert.match(launchPage, /data-dx-scene-workflow-action="select-node"/);
  assert.match(launchPage, /data-dx-scene-workflow-action="regress-performance"/);
  assert.match(launchPage, /data-dx-scene-workflow-action="reset-performance"/);
  assert.match(runtime, /function bindScene\(\)/);
  assert.match(runtime, /sceneNodes/);
  assert.match(runtime, /data-dx-scene-workflow-selected-node/);
  assert.match(runtime, /data-dx-scene-performance-band/);
});

test("launch scene exposes a source-owned renderer handoff contract", () => {
  const handoff = readRequiredFile(
    "examples/template/scene/renderer-handoff.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneRendererId = "source-owned-webgl" \| "three-r3f-drei";/);
  assert.match(types, /export type DxSceneRendererCapabilities = {/);
  assert.match(types, /export type DxSceneRendererAdapter = {/);
  assert.match(types, /mount: \(/);
  assert.match(types, /export type DxSceneRendererHandoff = {/);
  assert.match(types, /capabilities: DxSceneRendererCapabilities;/);

  assert.match(handoff, /export function createDxSceneWebGLRendererAdapter/);
  assert.match(handoff, /mountDxWebGLScene\(canvas, scene, runtime\)/);
  assert.match(handoff, /export function createDxSceneRendererHandoff/);
  assert.match(handoff, /export function mountDxSceneWithRenderer/);
  assert.match(handoff, /dxLaunchSceneForgePackage\.packageId/);
  assert.match(handoff, /appOwnedBoundaries: dxLaunchSceneForgePackage\.appOwnedBoundaries/);

  assert.match(index, /createDxSceneWebGLRendererAdapter/);
  assert.match(index, /createDxSceneRendererHandoff/);
  assert.match(index, /mountDxSceneWithRenderer/);
  assert.match(index, /DxSceneRendererAdapter/);
  assert.match(index, /DxSceneRendererHandoff/);

  assert.match(launchScene, /createDxSceneRendererHandoff/);
  assert.match(launchScene, /mountDxSceneWithRenderer/);
  assert.match(launchScene, /data-dx-scene-renderer=\{handoff\.renderer\.id\}/);
  assert.doesNotMatch(launchScene, /mountDxWebGLScene/);

  assert.match(metadata, /"lib\/scene\/renderer-handoff\.ts"/);
  assert.match(metadata, /"renderer-handoff"/);
  assert.match(readme, /Renderer handoff/);
  assert.match(readme, /Three\/R3F\/Drei/);
});

test("launch scene exposes an injectable Three R3F Drei renderer adapter", () => {
  const adapter = readRequiredFile(
    "examples/template/scene/r3f-renderer-adapter.ts",
  );
  const types = readRequiredFile("examples/template/scene/types.ts");
  const index = readRequiredFile("examples/template/scene/index.ts");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");

  assert.match(types, /export type DxSceneR3FFrameloop = "always" \| "demand" \| "never";/);
  assert.match(types, /export type DxSceneR3FRootConfig = {/);
  assert.match(types, /frameloop\?: DxSceneR3FFrameloop;/);
  assert.match(types, /events\?: unknown;/);
  assert.match(types, /onCreated\?: \(state: unknown\) => void;/);
  assert.match(types, /export type DxSceneR3FRoot = {/);
  assert.match(
    types,
    /configure: \(\s*props\?: DxSceneR3FRootConfig,?\s*\) => DxSceneR3FRoot \| Promise<DxSceneR3FRoot>;/,
  );
  assert.match(types, /render: \(children: unknown\) => unknown;/);
  assert.match(types, /unmount: \(\) => void;/);
  assert.match(types, /export type DxSceneDreiKeyboardMapEntry = {/);
  assert.match(types, /export type DxSceneR3FRendererDependencies = {/);

  assert.match(adapter, /export function createDxSceneR3FRootConfig/);
  assert.match(adapter, /frameloop: options\.frameloop \?\? "demand"/);
  assert.match(adapter, /events: options\.events/);
  assert.match(adapter, /runtime\.onStatusChange\?\.\("ready"\)/);
  assert.match(adapter, /export function createDxSceneDreiKeyboardMap/);
  assert.match(adapter, /dxSceneKeyboardBindings\.map/);
  assert.match(adapter, /name: binding\.action/);
  assert.match(adapter, /keys: binding\.keys/);
  assert.match(adapter, /export function createDxSceneR3FSceneElementInput/);
  assert.match(adapter, /keyboardMap: createDxSceneDreiKeyboardMap\(\)/);
  assert.match(adapter, /export function createDxSceneR3FDreiRendererAdapter/);
  assert.match(adapter, /id: "three-r3f-drei"/);
  assert.match(adapter, /requiresPackageInstall: true/);
  assert.match(adapter, /const root = dependencies\.createRoot\(canvas\)/);
  assert.match(adapter, /Promise\.resolve\(root\.configure\(/);
  assert.match(adapter, /rootForRender\.render\(\s*dependencies\.createSceneElement\(/);
  assert.match(adapter, /rootForRender\.unmount\(\)/);
  assert.doesNotMatch(adapter, /from ["']\@react-three\/fiber|from ["']\@react-three\/drei|from ["']three/);

  assert.match(index, /createDxSceneR3FDreiRendererAdapter/);
  assert.match(index, /createDxSceneR3FRootConfig/);
  assert.match(index, /createDxSceneDreiKeyboardMap/);
  assert.match(index, /DxSceneR3FRendererDependencies/);
  assert.match(metadata, /"lib\/scene\/r3f-renderer-adapter\.ts"/);
  assert.match(metadata, /"r3f-renderer-adapter"/);
  assert.match(readme, /Injected Three\/R3F\/Drei adapter/);
  assert.match(readme, /createRoot\(canvas\)/);
});

test("launch scene package metadata is source-owned and professionally discoverable", () => {
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");
  const registry = readRequiredFile("core/src/ecosystem/forge_three_scene.rs");

  for (const value of [
    "aliases",
    "sourceMirrors",
    "provenance",
    "exportedFiles",
    "requiredEnv",
    "receiptPaths",
    "icon",
    "G:/WWW/inspirations/three.js",
    "G:/WWW/inspirations/react-three-fiber",
    "G:/WWW/inspirations/drei",
    "THREE.WebGLRenderer",
    "@react-three/fiber createRoot(canvas)",
    "@react-three/drei KeyboardControls",
    "<dx-icon name=\"pack:three-scene\" />",
    ".dx/forge/receipts/3d-launch-scene.json",
  ]) {
    assert.match(metadata, new RegExp(value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const file of [
    "interaction.ts",
    "performance-monitor.ts",
    "renderer-handoff.ts",
    "r3f-renderer-adapter.ts",
  ]) {
    assert.match(metadata, new RegExp(`lib/scene/${file}`));
    assert.match(registry, new RegExp(`scene/${file}`));
  }

  assert.match(readme, /Provenance and ownership/);
  assert.match(readme, /WebGLRenderer/);
  assert.match(readme, /Canvas\/createRoot/);
  assert.match(readme, /KeyboardControls/);
});

test("launch scene uses DX token and icon contracts in the dashboard surface", () => {
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");

  assert.match(launchScene, /<dx-icon name="pack:three-scene" aria-hidden="true" \/>/);
  assert.match(launchScene, /data-dx-icon="pack:three-scene"/);
  assert.match(launchScene, /data-dx-style-surface="launch-scene"/);
  assert.match(launchScene, /data-dx-token-scope="3d\/launch-scene"/);
  assert.match(launchScene, /bg-\[var\(--dx-scene-surface\)\]/);
  assert.match(launchScene, /text-\[var\(--dx-scene-text\)\]/);
  assert.match(launchScene, /border-\[var\(--dx-scene-border\)\]/);
  assert.doesNotMatch(
    launchScene,
    /(?:bg-black|bg-white\/|text-(?:cyan|emerald|amber|neutral)|border-(?:cyan|emerald|amber|neutral)|shadow-emerald)/,
  );
});
