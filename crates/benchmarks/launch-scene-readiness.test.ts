const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("launch scene exposes a Web Preview readiness contract", () => {
  const readiness = readRequiredFile(
    "examples/template/scene/preview-readiness.ts",
  );
  const webglRuntime = readRequiredFile(
    "examples/template/scene/webgl-runtime.ts",
  );
  const launchScene = readRequiredFile("examples/template/launch-scene.tsx");
  const sceneIndex = readRequiredFile("examples/template/scene/index.ts");
  const metadata = readRequiredFile("examples/template/scene/metadata.ts");
  const readme = readRequiredFile("examples/template/scene/README.md");
  const routeContract = readRequiredFile(
    "examples/template/template-route-contract.ts",
  );
  const forgeScene = readRequiredFile("core/src/ecosystem/forge_three_scene.rs");
  const registry = readRequiredFile("core/src/ecosystem/forge_registry.rs");
  const cli = readRequiredFile("dx-www/src/cli/mod.rs");

  assert.match(readiness, /export function createDxScenePreviewReadiness/);
  assert.match(readiness, /export function createDxSceneFallbackStyle/);
  assert.match(readiness, /--dx-scene-base/);
  assert.match(readiness, /--dx-scene-horizon/);
  assert.match(readiness, /--dx-scene-accent/);
  assert.match(readiness, /requiresPackageInstall: false/);
  assert.match(readiness, /requiresServer: false/);
  assert.match(readiness, /shaderOwnership: "source-owned"/);
  assert.match(readiness, /warnings: \[\]/);
  assert.match(readiness, /quality: "premium-launch"/);

  assert.match(webglRuntime, /export function createDxSceneRenderBudget/);
  assert.match(webglRuntime, /maxDevicePixelRatio/);
  assert.match(webglRuntime, /reducedMotionFrameInterval/);
  assert.match(webglRuntime, /budget\.maxDevicePixelRatio/);
  assert.match(webglRuntime, /Math\.min\(window\.devicePixelRatio/);
  assert.match(webglRuntime, /deleteBuffer\(positionBuffer\)/);
  assert.match(webglRuntime, /deleteProgram\(program\.program\)/);
  assert.match(webglRuntime, /deleteShader\(program\.vertex\)/);
  assert.match(webglRuntime, /deleteShader\(program\.fragment\)/);

  assert.match(launchScene, /createDxScenePreviewReadiness/);
  assert.match(launchScene, /createDxSceneFallbackStyle/);
  assert.match(launchScene, /createDxSceneRendererHandoff/);
  assert.match(launchScene, /data-dx-scene-preview-readiness/);
  assert.match(launchScene, /data-dx-scene-fallback-layer/);
  assert.match(launchScene, /data-dx-scene-renderer/);
  assert.match(launchScene, /style=\{fallbackStyle\}/);
  assert.match(launchScene, /readiness\.status/);

  assert.match(sceneIndex, /export \{ dxLaunchSceneForgePackage \}/);
  assert.match(sceneIndex, /export \{ createDxLaunchScenePreset, splineLikeLayers \}/);
  assert.match(sceneIndex, /export \{ createDxSceneFallbackStyle, createDxScenePreviewReadiness \}/);
  assert.match(sceneIndex, /export \{ createDxSceneRenderBudget, mountDxWebGLScene \}/);
  assert.match(sceneIndex, /createDxSceneRendererHandoff/);
  assert.match(sceneIndex, /mountDxSceneWithRenderer/);
  assert.match(sceneIndex, /DxSceneRenderBudget/);
  assert.match(sceneIndex, /DxSceneRendererHandoff/);
  assert.match(sceneIndex, /export type \{/);
  assert.doesNotMatch(sceneIndex, /@react-three\/fiber|@react-three\/drei|three/);

  assert.match(metadata, /"lib\/scene\/preview-readiness\.ts"/);
  assert.match(metadata, /"lib\/scene\/index\.ts"/);
  assert.match(readme, /Consumer entrypoint/);
  assert.match(readme, /createDxSceneRendererHandoff/);
  assert.match(readme, /mountDxSceneWithRenderer/);
  assert.match(routeContract, /"lib\/scene\/index\.ts"/);
  assert.match(routeContract, /"lib\/scene\/preview-readiness\.ts"/);
  assert.match(forgeScene, /scene\/preview-readiness\.ts/);
  assert.match(forgeScene, /scene\/index\.ts/);
  assert.match(registry, /lib\/scene\/preview-readiness\.ts/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_INDEX_TS/);
  assert.match(cli, /NEXT_FAMILIAR_SCENE_PREVIEW_READINESS_TS/);
  assert.match(cli, /"lib\/scene\/index\.ts"/);
  assert.match(cli, /"lib\/scene\/preview-readiness\.ts"/);
});
