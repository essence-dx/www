import type {
  DxLaunchScenePreset,
  DxSceneCameraUniforms,
  DxSceneController,
  DxSceneFrameCallback,
  DxSceneFrameRuntime,
  DxSceneInteractionKind,
  DxSceneMaterialUniforms,
  DxSceneNodeInteraction,
  DxSceneNodeInteractionCallback,
  DxSceneNodeUniform,
  DxSceneNodeUniforms,
  DxScenePerformanceCallback,
  DxScenePerformanceRegression,
  DxScenePerformanceRegressionCallback,
  DxScenePerformanceRegressReason,
  DxScenePerformanceSample,
  DxSceneQualityUniforms,
  DxSceneRenderBudget,
  DxSceneRgb,
  DxSceneVec2,
  DxSceneViewport,
} from "./types";
import {
  createDxSceneInteractionMap,
  createDxSceneKeyboardInteraction,
  createDxSceneNodeInteraction,
  resolveDxSceneKeyboardAction,
} from "./interaction";
import { createDxScenePerformanceMonitor } from "./performance-monitor";

const vertexShaderSource = `
attribute vec2 position;
void main() {
  gl_Position = vec4(position, 0.0, 1.0);
}
`;

const fragmentShaderSource = `
precision mediump float;
uniform vec2 resolution;
uniform vec2 pointer;
uniform float time;
uniform float intensity;
uniform vec3 baseColor;
uniform vec3 horizonColor;
uniform vec3 accentA;
uniform vec3 accentB;
uniform vec3 accentC;
uniform vec3 accentD;
uniform vec3 nodeA;
uniform vec3 nodeB;
uniform vec3 nodeC;
uniform vec3 nodeD;
uniform vec3 cameraPosition;
uniform vec3 cameraTarget;
uniform float cameraFocalLength;
uniform float cameraDepth;
uniform float materialContrast;
uniform float materialBloom;
uniform float materialSheen;
uniform float qualityDetail;

void main() {
  vec2 uv = (gl_FragCoord.xy / resolution.xy) * 2.0 - 1.0;
  uv.x *= resolution.x / max(resolution.y, 1.0);
  float lensScale = clamp(cameraFocalLength / 42.0, 0.64, 1.55);
  float depthScale = clamp(cameraDepth / 5.2, 0.55, 1.7);
  vec2 cameraDrift = (cameraPosition.xy - cameraTarget.xy) * 0.035;
  uv = (uv + pointer * 0.12 + cameraDrift) / lensScale;

  float vignette = 1.0 - smoothstep(0.12, 1.48 * depthScale, length(uv));
  vec2 coreCenter = vec2(
    sin(time * (0.32 + nodeA.y)) * nodeA.x,
    cos(time * (0.24 + nodeA.y)) * nodeA.x * 0.62
  );
  float orbit = length(uv - coreCenter);
  float core = (1.0 - smoothstep(0.0, max(nodeA.x, 0.08), orbit)) * nodeA.z;
  float halo = (
    1.0 - smoothstep(
      0.12,
      max(nodeB.x, 0.18),
      length(uv + vec2(nodeB.y * 0.18, -0.06))
    )
  ) * nodeB.z;
  float ringA = (1.0 - smoothstep(0.0, 0.016, abs(length(uv) - nodeC.x * depthScale))) * nodeC.z;
  float ringB = (
    1.0 - smoothstep(
      0.0,
      0.012,
      abs(length(uv - vec2(nodeB.y * 0.28, -0.08)) - nodeB.x * 0.48 * depthScale)
    )
  ) * nodeB.z;
  float qualityGridDensity = 4.0 + qualityDetail * 6.0 + nodeC.y * 2.0;
  float grid = (
    1.0 - smoothstep(
      0.0,
      0.014,
      abs(fract((uv.x + uv.y + time * 0.08) * qualityGridDensity) - 0.5)
    )
  ) * nodeC.z;
  float sparkDensity = 8.0 + qualityDetail * 10.0 + nodeD.y * 5.0;
  float sparks = (
    1.0 - smoothstep(
      0.0,
      0.028,
      abs(
        sin((uv.x * sparkDensity) + time) *
        cos((uv.y * 13.0) - time * 0.7)
      )
    )
  ) * nodeD.z;

  float targetBias = clamp(cameraTarget.y * 0.12 + 0.5, 0.0, 1.0);
  vec3 color = mix(
    baseColor,
    horizonColor,
    clamp((uv.y + 1.0) * 0.5 + targetBias * 0.08, 0.0, 1.0)
  );
  color += accentA * core * intensity;
  color += accentB * ringA * 0.78;
  color += accentC * ringB * 0.58;
  color += accentC * grid * (0.05 + qualityDetail * 0.025);
  color += accentD * sparks * (0.008 + qualityDetail * 0.006 + materialBloom * 0.018);
  color += accentA * halo * (0.06 + materialBloom * 0.08);
  color += mix(accentB, accentD, 0.35) * ringA * materialSheen * 0.12;
  color = mix(vec3(0.5), color, materialContrast);
  color *= vignette;

  gl_FragColor = vec4(color, 1.0);
}
`;

type DxSceneProgramResources = {
  program: WebGLProgram;
  vertex: WebGLShader;
  fragment: WebGLShader;
};

function compileShader(
  gl: WebGLRenderingContext,
  type: number,
  source: string,
): WebGLShader | null {
  const shader = gl.createShader(type);
  if (!shader) {
    return null;
  }
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  return gl.getShaderParameter(shader, gl.COMPILE_STATUS) ? shader : null;
}

function createProgram(gl: WebGLRenderingContext): DxSceneProgramResources | null {
  const vertex = compileShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
  const fragment = compileShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
  if (!vertex || !fragment) {
    if (vertex) {
      gl.deleteShader(vertex);
    }
    if (fragment) {
      gl.deleteShader(fragment);
    }
    return null;
  }

  const program = gl.createProgram();
  if (!program) {
    gl.deleteShader(vertex);
    gl.deleteShader(fragment);
    return null;
  }
  gl.attachShader(program, vertex);
  gl.attachShader(program, fragment);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    gl.deleteProgram(program);
    gl.deleteShader(vertex);
    gl.deleteShader(fragment);
    return null;
  }

  return { fragment, program, vertex };
}

export function createDxSceneRenderBudget(
  scene: DxLaunchScenePreset,
): DxSceneRenderBudget {
  return {
    maxDevicePixelRatio: scene.quality.maxDevicePixelRatio,
    fullMotionFrameInterval: scene.quality.fullMotionFrameInterval,
    reducedMotionFrameInterval: scene.quality.reducedMotionFrameInterval,
    antialias: scene.quality.antialias,
    powerPreference: scene.quality.powerPreference,
  };
}

function resizeCanvas(
  canvas: HTMLCanvasElement,
  maxDevicePixelRatio: number,
): DxSceneViewport {
  const rect = canvas.getBoundingClientRect();
  const pixelRatio = Math.min(window.devicePixelRatio || 1, maxDevicePixelRatio);
  const width = Math.max(1, Math.floor(rect.width * pixelRatio));
  const height = Math.max(1, Math.floor(rect.height * pixelRatio));
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
  }
  return { height, pixelRatio, width };
}

function nodeColor(scene: DxLaunchScenePreset, index: number, fallback: DxSceneRgb) {
  return scene.nodes[index]?.color ?? fallback;
}

function nodeUniform(
  scene: DxLaunchScenePreset,
  index: number,
  fallback: DxSceneNodeUniform,
): DxSceneNodeUniform {
  return {
    radius: scene.nodes[index]?.radius ?? fallback.radius,
    orbit: scene.nodes[index]?.orbit ?? fallback.orbit,
    opacity: scene.nodes[index]?.opacity ?? fallback.opacity,
  };
}

export function createDxSceneNodeUniforms(
  scene: DxLaunchScenePreset,
): DxSceneNodeUniforms {
  return {
    a: nodeUniform(scene, 0, { opacity: 0.88, orbit: 0.18, radius: 0.42 }),
    b: nodeUniform(scene, 1, { opacity: 0.68, orbit: 0.36, radius: 0.72 }),
    c: nodeUniform(scene, 2, { opacity: 0.46, orbit: 0.52, radius: 0.92 }),
    d: nodeUniform(scene, 3, { opacity: 0.58, orbit: 0.72, radius: 0.2 }),
  };
}

function distance3(
  a: readonly [number, number, number],
  b: readonly [number, number, number],
) {
  const x = a[0] - b[0];
  const y = a[1] - b[1];
  const z = a[2] - b[2];
  return Math.sqrt(x * x + y * y + z * z);
}

export function createDxSceneCameraUniforms(
  scene: DxLaunchScenePreset,
): DxSceneCameraUniforms {
  return {
    depth: Math.max(0.1, distance3(scene.camera.position, scene.camera.target)),
    focalLength: scene.camera.focalLength,
    position: scene.camera.position,
    target: scene.camera.target,
  };
}

export function createDxSceneMaterialUniforms(
  scene: DxLaunchScenePreset,
): DxSceneMaterialUniforms {
  return scene.material;
}

export function createDxSceneQualityUniforms(
  scene: DxLaunchScenePreset,
): DxSceneQualityUniforms {
  return {
    detail: scene.quality.shaderDetail,
  };
}

function setRgb(
  gl: WebGLRenderingContext,
  location: WebGLUniformLocation | null,
  color: DxSceneRgb,
) {
  gl.uniform3f(location, color[0], color[1], color[2]);
}

function setVec3(
  gl: WebGLRenderingContext,
  location: WebGLUniformLocation | null,
  vector: readonly [number, number, number],
) {
  gl.uniform3f(location, vector[0], vector[1], vector[2]);
}

function setNodeUniform(
  gl: WebGLRenderingContext,
  location: WebGLUniformLocation | null,
  node: DxSceneNodeUniform,
) {
  gl.uniform3f(location, node.radius, node.orbit, node.opacity);
}

function getWebGLContext(
  canvas: HTMLCanvasElement,
  budget: DxSceneRenderBudget,
): WebGLRenderingContext | null {
  const contextAttributes: WebGLContextAttributes = {
    alpha: false,
    antialias: budget.antialias,
    powerPreference: budget.powerPreference,
    preserveDrawingBuffer: false,
  };

  return (
    canvas.getContext("webgl", contextAttributes) ??
    (canvas.getContext(
      "experimental-webgl",
      contextAttributes,
    ) as WebGLRenderingContext | null)
  );
}

function normalizeDxSceneFrameCallbacks(
  onFrame: DxSceneFrameRuntime["onFrame"],
): readonly DxSceneFrameCallback[] {
  if (!onFrame) {
    return [];
  }
  return Array.isArray(onFrame) ? onFrame : [onFrame];
}

function normalizeDxScenePerformanceCallbacks(
  onPerformanceChange: DxSceneFrameRuntime["onPerformanceChange"],
): readonly DxScenePerformanceCallback[] {
  if (!onPerformanceChange) {
    return [];
  }
  return Array.isArray(onPerformanceChange)
    ? onPerformanceChange
    : [onPerformanceChange];
}

function normalizeDxScenePerformanceRegressionCallbacks(
  onPerformanceRegression: DxSceneFrameRuntime["onPerformanceRegression"],
): readonly DxScenePerformanceRegressionCallback[] {
  if (!onPerformanceRegression) {
    return [];
  }
  return Array.isArray(onPerformanceRegression)
    ? onPerformanceRegression
    : [onPerformanceRegression];
}

function normalizeDxSceneNodeInteractionCallbacks(
  onInteraction: DxSceneFrameRuntime["onNodeHover"],
): readonly DxSceneNodeInteractionCallback[] {
  if (!onInteraction) {
    return [];
  }
  return Array.isArray(onInteraction) ? onInteraction : [onInteraction];
}

export function mountDxWebGLScene(
  canvas: HTMLCanvasElement,
  scene: DxLaunchScenePreset,
  runtime: DxSceneFrameRuntime,
): DxSceneController {
  const budget = createDxSceneRenderBudget(scene);
  const gl = getWebGLContext(canvas, budget);
  const frameCallbacks = normalizeDxSceneFrameCallbacks(runtime.onFrame);
  const performanceCallbacks = normalizeDxScenePerformanceCallbacks(
    runtime.onPerformanceChange,
  );
  const performanceRegressionCallbacks =
    normalizeDxScenePerformanceRegressionCallbacks(
      runtime.onPerformanceRegression,
    );
  const nodeHoverCallbacks = normalizeDxSceneNodeInteractionCallbacks(
    runtime.onNodeHover,
  );
  const nodeSelectCallbacks = normalizeDxSceneNodeInteractionCallbacks(
    runtime.onNodeSelect,
  );
  const performanceMonitor = createDxScenePerformanceMonitor(scene);
  let performanceSample = performanceMonitor.current();
  let lastElapsed = 0;
  let lastHoverNodeId: string | null = null;
  let lastPointer: DxSceneVec2 = [0, 0];
  let focusedNodeIndex: number | null = null;
  const createFallbackController = (): DxSceneController => ({
    dispose: () => undefined,
    pickNode(pointer: DxSceneVec2, kind: DxSceneInteractionKind = "hover") {
      return createDxSceneNodeInteraction(
        scene,
        kind,
        pointer,
        0,
        runtime.interaction,
      );
    },
    selectNodeByIndex(
      index: number | null,
      kind: DxSceneInteractionKind = "hover",
    ) {
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
    regressPerformance(reason: DxScenePerformanceRegressReason = "manual") {
      performanceSample = performanceMonitor.regress(reason);
      return performanceSample;
    },
    resetPerformance() {
      performanceSample = performanceMonitor.reset();
      return performanceSample;
    },
  });
  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  if (!gl) {
    runtime.onStatusChange?.("fallback");
    return createFallbackController();
  }

  const program = createProgram(gl);
  const positionBuffer = gl.createBuffer();
  if (!program || !positionBuffer) {
    if (positionBuffer) {
      gl.deleteBuffer(positionBuffer);
    }
    if (program) {
      gl.deleteProgram(program.program);
      gl.deleteShader(program.vertex);
      gl.deleteShader(program.fragment);
    }
    runtime.onStatusChange?.("fallback");
    return createFallbackController();
  }

  const position = gl.getAttribLocation(program.program, "position");
  const resolution = gl.getUniformLocation(program.program, "resolution");
  const pointerUniform = gl.getUniformLocation(program.program, "pointer");
  const time = gl.getUniformLocation(program.program, "time");
  const intensity = gl.getUniformLocation(program.program, "intensity");
  const baseColor = gl.getUniformLocation(program.program, "baseColor");
  const horizonColor = gl.getUniformLocation(program.program, "horizonColor");
  const accentA = gl.getUniformLocation(program.program, "accentA");
  const accentB = gl.getUniformLocation(program.program, "accentB");
  const accentC = gl.getUniformLocation(program.program, "accentC");
  const accentD = gl.getUniformLocation(program.program, "accentD");
  const nodeA = gl.getUniformLocation(program.program, "nodeA");
  const nodeB = gl.getUniformLocation(program.program, "nodeB");
  const nodeC = gl.getUniformLocation(program.program, "nodeC");
  const nodeD = gl.getUniformLocation(program.program, "nodeD");
  const cameraPosition = gl.getUniformLocation(program.program, "cameraPosition");
  const cameraTarget = gl.getUniformLocation(program.program, "cameraTarget");
  const cameraFocalLength = gl.getUniformLocation(
    program.program,
    "cameraFocalLength",
  );
  const cameraDepth = gl.getUniformLocation(program.program, "cameraDepth");
  const materialContrast = gl.getUniformLocation(
    program.program,
    "materialContrast",
  );
  const materialBloom = gl.getUniformLocation(program.program, "materialBloom");
  const materialSheen = gl.getUniformLocation(program.program, "materialSheen");
  const qualityDetail = gl.getUniformLocation(program.program, "qualityDetail");
  const cameraUniforms = createDxSceneCameraUniforms(scene);
  const materialUniforms = createDxSceneMaterialUniforms(scene);
  const nodeUniforms = createDxSceneNodeUniforms(scene);
  const qualityUniforms = createDxSceneQualityUniforms(scene);
  let frame = 0;
  let frameScheduled = false;
  let disposed = false;
  let lastPaint = 0;
  let lastPerformanceKey = "";
  let reportedReady = false;
  let resourcesReleased = false;
  let fallbackReported = false;
  const pointer = { x: 0, y: 0 };

  const releaseResources = () => {
    if (resourcesReleased) {
      return;
    }
    resourcesReleased = true;
    gl.deleteBuffer(positionBuffer);
    gl.deleteProgram(program.program);
    gl.deleteShader(program.vertex);
    gl.deleteShader(program.fragment);
  };

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
    gl.STATIC_DRAW,
  );

  const requestNextFrame = () => {
    if (disposed || frameScheduled || document.hidden) {
      return;
    }
    frameScheduled = true;
    frame = runtime.requestFrame(render);
  };

  const cancelScheduledFrame = () => {
    if (!frameScheduled) {
      return;
    }
    runtime.cancelFrame(frame);
    frame = 0;
    frameScheduled = false;
  };

  const notifyPerformanceChange = (sample: DxScenePerformanceSample) => {
    if (performanceCallbacks.length === 0) {
      return;
    }

    const key = [
      sample.band,
      sample.factor.toFixed(3),
      sample.maxDevicePixelRatio.toFixed(3),
      sample.shaderDetail.toFixed(3),
    ].join(":");
    if (key === lastPerformanceKey) {
      return;
    }
    lastPerformanceKey = key;
    for (const callback of performanceCallbacks) {
      callback(sample);
    }
  };

  const notifyPerformanceRegression = (
    regression: DxScenePerformanceRegression,
  ) => {
    if (performanceRegressionCallbacks.length === 0) {
      return;
    }
    for (const callback of performanceRegressionCallbacks) {
      callback(regression);
    }
  };

  const regressPerformance = (
    reason: DxScenePerformanceRegressReason = "manual",
  ) => {
    if (disposed) {
      return performanceSample;
    }
    performanceSample = performanceMonitor.regress(reason);
    notifyPerformanceChange(performanceSample);
    notifyPerformanceRegression({ reason, sample: performanceSample });
    requestNextFrame();
    return performanceSample;
  };

  const resetPerformance = () => {
    performanceSample = performanceMonitor.reset();
    notifyPerformanceChange(performanceSample);
    return performanceSample;
  };

  const selectNodeByIndex = (
    index: number | null,
    kind: DxSceneInteractionKind = "hover",
  ) => {
    const hit =
      index === null
        ? undefined
        : createDxSceneInteractionMap(
            scene,
            lastElapsed,
            runtime.interaction,
          )[index];
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
  };

  const pickNode = (
    pointerPosition: DxSceneVec2,
    kind: DxSceneInteractionKind = "hover",
  ) =>
    createDxSceneNodeInteraction(
      scene,
      kind,
      pointerPosition,
      lastElapsed,
      runtime.interaction,
    );

  const notifyNodeHover = (interaction: DxSceneNodeInteraction) => {
    if (interaction.nodeId === lastHoverNodeId) {
      return;
    }
    lastHoverNodeId = interaction.nodeId;
    canvas.style.cursor = interaction.nodeId ? "pointer" : "";
    for (const callback of nodeHoverCallbacks) {
      callback(interaction);
    }
  };

  const notifyNodeSelect = (interaction: DxSceneNodeInteraction) => {
    for (const callback of nodeSelectCallbacks) {
      callback(interaction);
    }
  };

  const resetPointer = () => {
    pointer.x = 0;
    pointer.y = 0;
    lastPointer = [0, 0];
    focusedNodeIndex = null;
    notifyNodeHover(
      createDxSceneNodeInteraction(
        scene,
        "miss",
        lastPointer,
        lastElapsed,
        runtime.interaction,
      ),
    );
  };

  const enterFallback = () => {
    if (fallbackReported) {
      return;
    }
    fallbackReported = true;
    disposed = true;
    cancelScheduledFrame();
    resetPointer();
    releaseResources();
    runtime.onStatusChange?.("fallback");
  };

  function render(now: number) {
    frameScheduled = false;
    if (disposed || document.hidden) {
      return;
    }

    if (gl.isContextLost()) {
      enterFallback();
      return;
    }

    const frameInterval = reduceMotion.matches
      ? budget.reducedMotionFrameInterval
      : budget.fullMotionFrameInterval;
    if (lastPaint > 0 && now - lastPaint < frameInterval) {
      requestNextFrame();
      return;
    }
    const previousPaint = lastPaint;
    lastPaint = now;
    const delta = previousPaint > 0 ? (now - previousPaint) / 1000 : 0;
    performanceSample = performanceMonitor.sample(delta);
    const adaptiveMaxDevicePixelRatio = Math.min(
      budget.maxDevicePixelRatio,
      performanceSample.maxDevicePixelRatio,
    );
    const viewport = resizeCanvas(canvas, adaptiveMaxDevicePixelRatio);
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.useProgram(program.program);
    gl.enableVertexAttribArray(position);
    gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
    gl.uniform2f(resolution, canvas.width, canvas.height);
    gl.uniform2f(pointerUniform, pointer.x, pointer.y);
    const elapsed = now / 1000;
    lastElapsed = elapsed;
    const motionMode = reduceMotion.matches ? "reduced" : "full";
    gl.uniform1f(
      time,
      elapsed *
        (motionMode === "reduced"
          ? scene.controls.reducedMotionSpeed
          : scene.controls.motionSpeed),
    );
    gl.uniform1f(intensity, scene.lighting.intensity);
    setRgb(gl, baseColor, scene.background.base);
    setRgb(gl, horizonColor, scene.background.horizon);
    setRgb(gl, accentA, nodeColor(scene, 0, scene.lighting.glow));
    setRgb(gl, accentB, nodeColor(scene, 1, scene.lighting.ambient));
    setRgb(gl, accentC, nodeColor(scene, 2, scene.lighting.glow));
    setRgb(gl, accentD, nodeColor(scene, 3, [1, 0.74, 0.24]));
    setVec3(gl, cameraPosition, cameraUniforms.position);
    setVec3(gl, cameraTarget, cameraUniforms.target);
    gl.uniform1f(cameraFocalLength, cameraUniforms.focalLength);
    gl.uniform1f(cameraDepth, cameraUniforms.depth);
    gl.uniform1f(materialContrast, materialUniforms.contrast);
    gl.uniform1f(materialBloom, materialUniforms.bloom);
    gl.uniform1f(materialSheen, materialUniforms.sheen);
    gl.uniform1f(
      qualityDetail,
      qualityUniforms.detail * performanceSample.shaderDetail,
    );
    setNodeUniform(gl, nodeA, nodeUniforms.a);
    setNodeUniform(gl, nodeB, nodeUniforms.b);
    setNodeUniform(gl, nodeC, nodeUniforms.c);
    setNodeUniform(gl, nodeD, nodeUniforms.d);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
    if (!reportedReady) {
      reportedReady = true;
      runtime.onStatusChange?.("ready");
    }
    notifyPerformanceChange(performanceSample);
    for (const callback of frameCallbacks) {
      callback({
        delta,
        elapsed,
        motionMode,
        performance: performanceSample,
        pointer: [pointer.x, pointer.y],
        scene,
        viewport,
      });
    }
    requestNextFrame();
  }

  const onPointerMove = (event: PointerEvent) => {
    const rect = canvas.getBoundingClientRect();
    const x = ((event.clientX - rect.left) / Math.max(rect.width, 1)) * 2 - 1;
    const y = ((event.clientY - rect.top) / Math.max(rect.height, 1)) * 2 - 1;
    lastPointer = [x, y];
    pointer.x = x * scene.controls.pointerParallax;
    pointer.y = -y * scene.controls.pointerParallax;
    const interaction = pickNode(lastPointer, "hover");
    focusedNodeIndex = interaction.index;
    notifyNodeHover(interaction);
    if (runtime.regressOnPointerMove) {
      regressPerformance("interaction");
    }
  };

  const onClick = () => {
    const interaction = pickNode(lastPointer, "select");
    focusedNodeIndex = interaction.index;
    notifyNodeSelect(interaction);
  };

  const onKeyDown = (event: KeyboardEvent) => {
    const action = resolveDxSceneKeyboardAction(event.key);
    if (!action || runtime.keyboard?.enabled === false) {
      return;
    }
    event.preventDefault();
    const interaction = createDxSceneKeyboardInteraction(
      scene,
      action,
      focusedNodeIndex,
      lastElapsed,
      runtime.keyboard,
      runtime.interaction,
    );
    focusedNodeIndex = interaction.index;
    if (action === "clear") {
      notifyNodeHover(selectNodeByIndex(focusedNodeIndex, "miss"));
      notifyNodeSelect(selectNodeByIndex(focusedNodeIndex, "miss"));
      return;
    }
    if (action === "select") {
      notifyNodeSelect(selectNodeByIndex(focusedNodeIndex, "select"));
      return;
    }
    notifyNodeHover(selectNodeByIndex(focusedNodeIndex, "hover"));
    regressPerformance("interaction");
  };

  const onContextLost = (event: Event) => {
    event.preventDefault();
    enterFallback();
  };

  const onContextRestored = () => {
    runtime.onStatusChange?.("fallback");
  };

  const onVisibilityChange = () => {
    if (document.hidden) {
      cancelScheduledFrame();
      resetPointer();
      return;
    }
    requestNextFrame();
  };

  canvas.addEventListener("pointermove", onPointerMove);
  canvas.addEventListener("click", onClick);
  canvas.addEventListener("keydown", onKeyDown);
  canvas.addEventListener("pointerleave", resetPointer);
  canvas.addEventListener("lostpointercapture", resetPointer);
  canvas.addEventListener("webglcontextlost", onContextLost);
  canvas.addEventListener("webglcontextrestored", onContextRestored);
  window.addEventListener("blur", resetPointer);
  document.addEventListener("visibilitychange", onVisibilityChange);
  requestNextFrame();

  return {
    pickNode,
    regressPerformance,
    resetPerformance,
    selectNodeByIndex,
    dispose() {
      disposed = true;
      canvas.removeEventListener("pointermove", onPointerMove);
      canvas.removeEventListener("click", onClick);
      canvas.removeEventListener("keydown", onKeyDown);
      canvas.removeEventListener("pointerleave", resetPointer);
      canvas.removeEventListener("lostpointercapture", resetPointer);
      canvas.removeEventListener("webglcontextlost", onContextLost);
      canvas.removeEventListener("webglcontextrestored", onContextRestored);
      window.removeEventListener("blur", resetPointer);
      document.removeEventListener("visibilitychange", onVisibilityChange);
      cancelScheduledFrame();
      releaseResources();
    },
  };
}
