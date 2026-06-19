import { dxLaunchSceneForgePackage } from "./metadata";
import type {
  DxLaunchScenePreset,
  DxSceneController,
  DxSceneFrameRuntime,
  DxSceneRendererAdapter,
  DxSceneRendererHandoff,
} from "./types";
import { mountDxWebGLScene } from "./webgl-runtime";

export function createDxSceneWebGLRendererAdapter(): DxSceneRendererAdapter {
  return {
    id: "source-owned-webgl",
    label: "Source-owned WebGL",
    source: "source-owned",
    capabilities: {
      webPreviewSafe: true,
      requiresPackageInstall: false,
      supportsFrameCallbacks: true,
      supportsPointerEvents: true,
      supportsKeyboardNavigation: true,
      supportsRendererSwap: true,
    },
    mount(canvas, scene, runtime) {
      return mountDxWebGLScene(canvas, scene, runtime);
    },
  };
}

export function createDxSceneRendererHandoff(
  scene: DxLaunchScenePreset,
  renderer: DxSceneRendererAdapter = createDxSceneWebGLRendererAdapter(),
): DxSceneRendererHandoff {
  return {
    packageId: dxLaunchSceneForgePackage.packageId,
    scene,
    renderer,
    appOwnedBoundaries: dxLaunchSceneForgePackage.appOwnedBoundaries,
  };
}

export function mountDxSceneWithRenderer(
  canvas: HTMLCanvasElement,
  handoff: DxSceneRendererHandoff,
  runtime: DxSceneFrameRuntime,
): DxSceneController {
  return handoff.renderer.mount(canvas, handoff.scene, runtime);
}
