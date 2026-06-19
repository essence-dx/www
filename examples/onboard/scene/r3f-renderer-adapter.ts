import {
  createDxSceneInteractionMap,
  createDxSceneNodeInteraction,
  dxSceneKeyboardBindings,
} from "./interaction";
import { dxLaunchSceneForgePackage } from "./metadata";
import { createDxScenePerformanceMonitor } from "./performance-monitor";
import type {
  DxLaunchScenePreset,
  DxSceneController,
  DxSceneDreiKeyboardMapEntry,
  DxSceneFrameRuntime,
  DxSceneInteractionKind,
  DxSceneR3FRendererDependencies,
  DxSceneR3FRendererOptions,
  DxSceneR3FRoot,
  DxSceneR3FRootConfig,
  DxSceneR3FSceneElementInput,
  DxSceneRendererAdapter,
  DxSceneVec2,
} from "./types";

export function createDxSceneR3FRootConfig(
  runtime: Pick<DxSceneFrameRuntime, "onStatusChange">,
  options: DxSceneR3FRendererOptions = {},
): DxSceneR3FRootConfig {
  return {
    frameloop: options.frameloop ?? "demand",
    events: options.events,
    onCreated(state: unknown) {
      runtime.onStatusChange?.("ready");
      options.onCreated?.(state);
    },
    onPointerMissed() {
      options.onPointerMissed?.();
    },
  };
}

export function createDxSceneDreiKeyboardMap(): readonly DxSceneDreiKeyboardMapEntry[] {
  return dxSceneKeyboardBindings.map((binding) => ({
    name: binding.action,
    keys: binding.keys,
  }));
}

function createDxSceneR3FController(
  scene: DxLaunchScenePreset,
  runtime: Pick<DxSceneFrameRuntime, "interaction">,
): DxSceneController {
  const performanceMonitor = createDxScenePerformanceMonitor(scene);
  let performanceSample = performanceMonitor.current();

  const pickNode = (
    pointer: DxSceneVec2,
    kind: DxSceneInteractionKind = "hover",
  ) => createDxSceneNodeInteraction(scene, kind, pointer, 0, runtime.interaction);

  return {
    dispose: () => undefined,
    pickNode,
    regressPerformance(reason = "manual") {
      performanceSample = performanceMonitor.regress(reason);
      return performanceSample;
    },
    resetPerformance() {
      performanceSample = performanceMonitor.reset();
      return performanceSample;
    },
    selectNodeByIndex(index, kind = "hover") {
      const hit =
        index === null
          ? undefined
          : createDxSceneInteractionMap(scene, 0, runtime.interaction)[index];
      return hit
        ? {
            anchor: hit.anchor,
            distance: 0,
            index: hit.index,
            kind,
            node: hit.node,
            nodeId: hit.nodeId,
            pointer: hit.anchor,
          }
        : createDxSceneNodeInteraction(scene, "miss", [0, 0]);
    },
  };
}

export function createDxSceneR3FSceneElementInput(
  scene: DxLaunchScenePreset,
  runtime: DxSceneFrameRuntime,
  options: DxSceneR3FRendererOptions,
  controller: DxSceneController,
): DxSceneR3FSceneElementInput {
  return {
    packageId: dxLaunchSceneForgePackage.packageId,
    scene,
    runtime,
    keyboardMap: createDxSceneDreiKeyboardMap(),
    keyboardDomElement: options.keyboardDomElement ?? null,
    controller,
  };
}

export function createDxSceneR3FDreiRendererAdapter(
  dependencies: DxSceneR3FRendererDependencies,
  options: DxSceneR3FRendererOptions = {},
): DxSceneRendererAdapter {
  return {
    id: "three-r3f-drei",
    label: "Injected Three/R3F/Drei",
    source: "app-owned",
    capabilities: {
      webPreviewSafe: false,
      requiresPackageInstall: true,
      supportsFrameCallbacks: true,
      supportsPointerEvents: true,
      supportsKeyboardNavigation: true,
      supportsRendererSwap: true,
    },
    mount(canvas, scene, runtime) {
      const controller = createDxSceneR3FController(scene, runtime);
      const root = dependencies.createRoot(canvas);
      let rootForRender: DxSceneR3FRoot = root;
      let disposed = false;

      Promise.resolve(root.configure(createDxSceneR3FRootConfig(runtime, options)))
        .then((configuredRoot) => {
          rootForRender = configuredRoot;
          if (disposed) {
            rootForRender.unmount();
            return;
          }
          rootForRender.render(dependencies.createSceneElement(
            createDxSceneR3FSceneElementInput(
              scene,
              runtime,
              options,
              controller,
            ),
          ));
        })
        .catch(() => {
          if (!disposed) {
            runtime.onStatusChange?.("fallback");
          }
        });

      return {
        ...controller,
        dispose() {
          disposed = true;
          rootForRender.unmount();
          controller.dispose();
        },
      };
    },
  };
}
