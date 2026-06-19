import type {
  DxLaunchScenePreset,
  DxSceneInteractionKind,
  DxSceneInteractionOptions,
  DxSceneKeyboardAction,
  DxSceneKeyboardBinding,
  DxSceneKeyboardOptions,
  DxSceneNodeHit,
  DxSceneNodeInteraction,
  DxSceneVec2,
} from "./types";

export const defaultDxSceneInteractionOptions: DxSceneInteractionOptions = {
  hitRadius: 0.18,
};

export const defaultDxSceneKeyboardOptions: DxSceneKeyboardOptions = {
  enabled: true,
  loop: true,
};

export const dxSceneKeyboardBindings: readonly DxSceneKeyboardBinding[] = [
  { action: "previous", keys: ["ArrowLeft", "ArrowUp"] },
  { action: "next", keys: ["ArrowRight", "ArrowDown"] },
  { action: "select", keys: ["Enter", " ", "Space"] },
  { action: "clear", keys: ["Escape"] },
];

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

function distance2(a: DxSceneVec2, b: DxSceneVec2) {
  const x = a[0] - b[0];
  const y = a[1] - b[1];
  return Math.sqrt(x * x + y * y);
}

function resolveInteractionOptions(
  options: Partial<DxSceneInteractionOptions> = {},
): DxSceneInteractionOptions {
  return {
    hitRadius: options.hitRadius ?? defaultDxSceneInteractionOptions.hitRadius,
  };
}

function resolveKeyboardOptions(
  options: Partial<DxSceneKeyboardOptions> = {},
): DxSceneKeyboardOptions {
  return {
    enabled: options.enabled ?? defaultDxSceneKeyboardOptions.enabled,
    loop: options.loop ?? defaultDxSceneKeyboardOptions.loop,
  };
}

export function resolveDxSceneKeyboardAction(
  key: string,
  bindings: readonly DxSceneKeyboardBinding[] = dxSceneKeyboardBindings,
): DxSceneKeyboardAction | null {
  return bindings.find((binding) => binding.keys.includes(key))?.action ?? null;
}

export function createDxSceneInteractionMap(
  scene: DxLaunchScenePreset,
  elapsed = 0,
  options: Partial<DxSceneInteractionOptions> = {},
): readonly DxSceneNodeHit[] {
  const resolved = resolveInteractionOptions(options);
  const nodeCount = Math.max(scene.nodes.length, 1);

  return scene.nodes.map((node, index) => {
    const phase =
      (index / nodeCount) * Math.PI * 2 +
      elapsed * (0.08 + node.orbit * 0.12);
    const radial = clamp(node.radius * 0.34 + node.orbit * 0.64, 0.12, 0.92);
    const anchor: DxSceneVec2 =
      index === 0
        ? [
            Math.sin(elapsed * 0.32) * node.radius,
            Math.cos(elapsed * 0.24) * node.radius * 0.32,
          ]
        : [Math.cos(phase) * radial, Math.sin(phase) * radial * 0.72];

    return {
      anchor,
      distance: Number.POSITIVE_INFINITY,
      index,
      node,
      nodeId: node.id,
      radius: clamp(node.radius * 0.18 + node.opacity * 0.06, 0.08, 0.3),
      threshold: resolved.hitRadius,
    };
  });
}

export function pickDxSceneNode(
  scene: DxLaunchScenePreset,
  pointer: DxSceneVec2,
  elapsed = 0,
  options: Partial<DxSceneInteractionOptions> = {},
): DxSceneNodeHit | null {
  const resolved = resolveInteractionOptions(options);
  let picked: DxSceneNodeHit | null = null;

  for (const candidate of createDxSceneInteractionMap(
    scene,
    elapsed,
    resolved,
  )) {
    const distance = distance2(pointer, candidate.anchor);
    const hitRadius = resolved.hitRadius;
    if (distance <= hitRadius + candidate.radius) {
      if (!picked || distance < picked.distance) {
        picked = { ...candidate, distance };
      }
    }
  }

  return picked;
}

function createMissInteraction(
  kind: DxSceneInteractionKind,
  pointer: DxSceneVec2,
): DxSceneNodeInteraction {
  return {
    anchor: null,
    distance: Number.POSITIVE_INFINITY,
    index: null,
    kind,
    node: null,
    nodeId: null,
    pointer,
  };
}

export function createDxSceneNodeInteraction(
  scene: DxLaunchScenePreset,
  kind: DxSceneInteractionKind,
  pointer: DxSceneVec2,
  elapsed = 0,
  options: Partial<DxSceneInteractionOptions> = {},
): DxSceneNodeInteraction {
  if (kind === "miss") {
    return createMissInteraction(kind, pointer);
  }

  const hit = pickDxSceneNode(scene, pointer, elapsed, options);

  return {
    anchor: hit?.anchor ?? null,
    distance: hit?.distance ?? Number.POSITIVE_INFINITY,
    index: hit?.index ?? null,
    kind: hit ? kind : "miss",
    node: hit?.node ?? null,
    nodeId: hit?.nodeId ?? null,
    pointer,
  };
}

export function createDxSceneKeyboardInteraction(
  scene: DxLaunchScenePreset,
  action: DxSceneKeyboardAction,
  currentIndex: number | null,
  elapsed = 0,
  keyboardOptions: Partial<DxSceneKeyboardOptions> = {},
  interactionOptions: Partial<DxSceneInteractionOptions> = {},
): DxSceneNodeInteraction {
  const keyboard = resolveKeyboardOptions(keyboardOptions);
  const nodes = createDxSceneInteractionMap(scene, elapsed, interactionOptions);
  if (!keyboard.enabled || action === "clear" || nodes.length === 0) {
    return createMissInteraction("miss", [0, 0]);
  }

  const maxIndex = nodes.length - 1;
  const baseIndex =
    currentIndex === null ? (action === "previous" ? maxIndex : 0) : currentIndex;
  let nextIndex = clamp(baseIndex, 0, maxIndex);

  if (action === "previous") {
    nextIndex =
      currentIndex === null
        ? maxIndex
        : keyboard.loop
          ? (currentIndex - 1 + nodes.length) % nodes.length
          : Math.max(0, currentIndex - 1);
  }
  if (action === "next") {
    nextIndex =
      currentIndex === null
        ? 0
        : keyboard.loop
          ? (currentIndex + 1) % nodes.length
          : Math.min(maxIndex, currentIndex + 1);
  }

  const hit = nodes[nextIndex];
  return {
    anchor: hit.anchor,
    distance: 0,
    index: hit.index,
    kind: action === "select" ? "select" : "hover",
    node: hit.node,
    nodeId: hit.nodeId,
    pointer: hit.anchor,
  };
}
