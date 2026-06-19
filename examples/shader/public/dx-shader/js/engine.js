var Engine = (function () {
  var canvas, gl, program, uniforms = {};
  var uniformCache = {};
  var playing = true;
  var suspensionReasons = {};
  var suspensionCount = 0;
  var initialized = false;
  var contextLost = false;
  var frameHandle = 0;
  var frameTimer = 0;
  var targetFrameIntervalMs = 0;
  var lastDrawAt = 0;
  var dirty = true;
  var loopT = 0;            // seconds into current loop
  var lastTick = 0;
  var fps = 60, fpsAcc = 0, fpsN = 0, fpsCb = null;
  var contextCb = null;
  var getParams = null;     // injected: () => P
  var MAX_RENDER_DIMENSION = 8192;

  var CONTEXT_OPTIONS = {
    antialias: false,
    preserveDrawingBuffer: false,
    powerPreference: "low-power",
    failIfMajorPerformanceCaveat: true
  };

  var UNIFORM_NAMES = (
    "u_res u_phase u_seed u_mode u_c1 u_c2 u_c3 u_c4 u_bg u_hue u_sat " +
    "u_exposure u_contrast u_scale u_complex u_warp u_flow u_stretch " +
    "u_light u_gloss u_lightAngle u_irid u_glow u_grain u_cell u_lines " +
    "u_ca u_vig u_soft u_travel u_synth u_modeB u_mixOp u_blend " +
    "u_genome u_g1 u_g2 u_g3"
  ).split(" ");

  function compile(type, src) {
    var sh = gl.createShader(type);
    gl.shaderSource(sh, src);
    gl.compileShader(sh);
    if (!gl.getShaderParameter(sh, gl.COMPILE_STATUS)) {
      throw new Error("Shader compile error:\n" + gl.getShaderInfoLog(sh));
    }
    return sh;
  }

  function createContext() {
    gl = canvas.getContext("webgl2", CONTEXT_OPTIONS);
    if (!gl) throw new Error("WebGL2 not available");
  }

  function safeRenderDimension(value) {
    var next = Number(value);
    if (!isFinite(next) || next <= 0) return 0;
    return Math.max(2, Math.min(MAX_RENDER_DIMENSION, 2 * Math.round(next / 2)));
  }

  function resetUniformCache() { uniformCache = {}; }

  function uploadUniform1f(name, value) {
    if (uniformCache[name] === value) return;
    uniformCache[name] = value;
    gl.uniform1f(uniforms[name], value);
  }

  function uploadUniform1i(name, value) {
    if (uniformCache[name] === value) return;
    uniformCache[name] = value;
    gl.uniform1i(uniforms[name], value);
  }

  function uploadUniform2f(name, x, y) {
    var c = uniformCache[name];
    if (c && c[0] === x && c[1] === y) return;
    if (!c) c = uniformCache[name] = new Array(2);
    c[0] = x;
    c[1] = y;
    gl.uniform2f(uniforms[name], x, y);
  }

  function uploadUniform3fv(name, value) {
    var c = uniformCache[name];
    if (c && c[0] === value[0] && c[1] === value[1] && c[2] === value[2]) return;
    if (!c) c = uniformCache[name] = new Array(3);
    c[0] = value[0];
    c[1] = value[1];
    c[2] = value[2];
    gl.uniform3fv(uniforms[name], value);
  }

  function uploadUniform4f(name, x, y, z, w) {
    var c = uniformCache[name];
    if (c && c[0] === x && c[1] === y && c[2] === z && c[3] === w) return;
    if (!c) c = uniformCache[name] = new Array(4);
    c[0] = x;
    c[1] = y;
    c[2] = z;
    c[3] = w;
    gl.uniform4f(uniforms[name], x, y, z, w);
  }

  function buildPipeline() {
    uniforms = {};
    resetUniformCache();
    program = gl.createProgram();
    var vertexShader = compile(gl.VERTEX_SHADER, VERT_SRC);
    var fragmentShader = compile(gl.FRAGMENT_SHADER, FRAG_SRC);
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    gl.deleteShader(vertexShader);
    gl.deleteShader(fragmentShader);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      throw new Error("Program link error:\n" + gl.getProgramInfoLog(program));
    }
    gl.useProgram(program);

    var buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
    gl.enableVertexAttribArray(0);
    gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);

    UNIFORM_NAMES.forEach(function (n) {
      uniforms[n] = gl.getUniformLocation(program, n);
    });
  }

  function notifyContext(state, error) {
    if (contextCb) contextCb(state, error || null);
  }

  function handleContextLost(event) {
    if (event && event.preventDefault) event.preventDefault();
    if (contextLost) return;
    contextLost = true;
    suspendFor("context");
    notifyContext("lost");
  }

  function handleContextRestored() {
    try {
      createContext();
      buildPipeline();
      contextLost = false;
      initialized = true;
      if (canvas && canvas.width && canvas.height) gl.viewport(0, 0, canvas.width, canvas.height);
      lastTick = performance.now();
      resumeFor("context");
      markDirty();
      notifyContext("restored");
    } catch (e) {
      contextLost = true;
      notifyContext("error", e);
    }
  }

  function installContextHandlers() {
    canvas.addEventListener("webglcontextlost", handleContextLost, false);
    canvas.addEventListener("webglcontextrestored", handleContextRestored, false);
  }

  function isContextLost() {
    return contextLost || !gl || (typeof gl.isContextLost === "function" && gl.isContextLost());
  }

  function observeContextLoss() {
    if (!contextLost && gl && typeof gl.isContextLost === "function" && gl.isContextLost()) {
      handleContextLost();
      return true;
    }
    return isContextLost();
  }

  function init(canvasEl, paramsGetter) {
    canvas = canvasEl;
    getParams = paramsGetter;
    contextLost = false;
    createContext();
    installContextHandlers();
    buildPipeline();
    initialized = true;
    lastTick = performance.now();
    if (document.hidden) suspendFor("visibility");
    markDirty();
  }

  function setSize(w, h) {
    if (!initialized || observeContextLoss()) return;
    w = safeRenderDimension(w);
    h = safeRenderDimension(h);
    if (!w || !h) return;
    if (canvas.width === w && canvas.height === h) return;
    if (canvas.width !== w) canvas.width = w;
    if (canvas.height !== h) canvas.height = h;
    gl.viewport(0, 0, w, h);
    markDirty();
  }

  function pushUniforms(P, phase) {
    uploadUniform2f("u_res", canvas.width, canvas.height);
    gl.uniform1f(uniforms.u_phase, phase);

    uploadUniform1f("u_seed", P.seed);
    uploadUniform1i("u_mode", P.mode);

    uploadUniform3fv("u_c1", hexToRgb01(P.c1));
    uploadUniform3fv("u_c2", hexToRgb01(P.c2));
    uploadUniform3fv("u_c3", hexToRgb01(P.c3));
    uploadUniform3fv("u_c4", hexToRgb01(P.c4));
    uploadUniform3fv("u_bg", hexToRgb01(P.bg));

    uploadUniform1f("u_hue", P.hue);
    uploadUniform1f("u_sat", P.sat);
    uploadUniform1f("u_exposure", P.exposure);
    uploadUniform1f("u_contrast", P.contrast);

    uploadUniform1f("u_scale", P.scale);
    uploadUniform1f("u_complex", P.complex);
    uploadUniform1f("u_warp", P.warp);
    uploadUniform1f("u_flow", P.flow);
    uploadUniform1f("u_stretch", P.stretch);

    uploadUniform1f("u_light", P.light);
    uploadUniform1f("u_gloss", P.gloss);
    uploadUniform1f("u_lightAngle", P.lightAngle);
    uploadUniform1f("u_irid", P.irid);
    uploadUniform1f("u_glow", P.glow);

    uploadUniform1f("u_grain", P.grain);
    uploadUniform1f("u_cell", P.cell);
    uploadUniform1f("u_lines", P.lines);
    uploadUniform1f("u_ca", P.ca);
    uploadUniform1f("u_vig", P.vig);
    uploadUniform1f("u_soft", P.soft);

    uploadUniform1f("u_travel", P.travel);

    uploadUniform1i("u_synth", P.synthOn ? 1 : 0);
    uploadUniform1i("u_modeB", P.modeB | 0);
    uploadUniform1i("u_mixOp", P.mixOp | 0);
    uploadUniform1f("u_blend", P.blend);

    var g = P.genes || [0,0,0,0, 0,0,0,0, 0,0,0,0];
    uploadUniform1i("u_genome", P.genomeOn ? 1 : 0);
    uploadUniform4f("u_g1", g[0], g[1], g[2], g[3]);
    uploadUniform4f("u_g2", g[4], g[5], g[6], g[7]);
    uploadUniform4f("u_g3", g[8], g[9], g[10], g[11]);
  }

  function renderAt(phase, P) {
    if (!initialized || observeContextLoss()) return false;
    P = P || getParams();
    pushUniforms(P, phase);
    gl.drawArrays(gl.TRIANGLES, 0, 3);
    dirty = false;
    return true;
  }

  function currentPhase() {
    var P = getParams();
    return (loopT / P.loop) % 1;
  }

  function tick(now) {
    frameHandle = 0;
    if (isSuspended()) return;
    if (!playing && !dirty) return;

    var dt = Math.min((now - lastTick) / 1000, 0.1);
    lastTick = now;
    var P = getParams();

    if (playing) {
      loopT = (loopT + dt) % P.loop;
      dirty = true;
      if (playing && targetFrameIntervalMs && lastDrawAt && now - lastDrawAt < targetFrameIntervalMs) {
        reportFrameTime(dt, false);
        scheduleFrame(targetFrameIntervalMs - (now - lastDrawAt));
        return;
      }
    }
    if (dirty) {
      try {
        if (!renderAt((loopT / P.loop) % 1, P)) return;
        lastDrawAt = now;
      } catch (e) {
        if (observeContextLoss()) {
          handleContextLost();
          return;
        }
        throw e;
      }
    }

    reportFrameTime(dt, true);
    scheduleFrame();
  }

  function reportFrameTime(dt, rendered) {
    fpsAcc += dt;
    if (rendered) fpsN++;
    if (fpsAcc >= 0.5) {
      fps = Math.round(fpsN / fpsAcc);
      fpsAcc = 0; fpsN = 0;
      if (fpsCb) fpsCb(fps);
    }
  }

  function cancelScheduledFrame() {
    if (frameHandle) {
      cancelAnimationFrame(frameHandle);
      frameHandle = 0;
    }
    if (frameTimer) {
      clearTimeout(frameTimer);
      frameTimer = 0;
    }
  }

  function scheduleFrame(delayMs) {
    if (!initialized) return;
    if (frameHandle || frameTimer || isSuspended()) return;
    if (!playing && !dirty) return;
    if (!delayMs && playing && targetFrameIntervalMs && lastDrawAt && !dirty) {
      var remaining = targetFrameIntervalMs - (performance.now() - lastDrawAt);
      if (remaining > 1) {
        scheduleFrame(remaining);
        return;
      }
    }
    if (delayMs && delayMs > 1) {
      frameTimer = setTimeout(function () {
        frameTimer = 0;
        if (isSuspended() || (!playing && !dirty)) return;
        scheduleFrame();
      }, Math.min(delayMs, targetFrameIntervalMs || delayMs));
      return;
    }
    frameHandle = requestAnimationFrame(tick);
  }

  function markDirty() {
    if (!initialized || observeContextLoss()) return;
    dirty = true;
    if (playing && targetFrameIntervalMs && frameTimer) return;
    scheduleFrame();
  }

  function isSuspended() {
    return suspensionCount > 0;
  }

  function resetFps() {
    fpsAcc = 0;
    fpsN = 0;
  }

  function resetSchedulerClock(now) {
    lastTick = now || performance.now();
    lastDrawAt = 0;
    resetFps();
  }

  function reportPausedFps() {
    if (fps === 0) return;
    fps = 0;
    resetFps();
    if (fpsCb) fpsCb(0);
  }

  function setTargetFps(fps) {
    var safeFps = Math.max(0, Number(fps) || 0);
    targetFrameIntervalMs = safeFps > 0 && safeFps < 55 ? 1000 / safeFps : 0;
    resetSchedulerClock();
    cancelScheduledFrame();
    scheduleFrame();
  }

  function suspendFor(reason) {
    reason = reason || "manual";
    if (!suspensionReasons[reason]) {
      suspensionReasons[reason] = true;
      suspensionCount++;
    }
    reportPausedFps();
    cancelScheduledFrame();
  }

  function resumeFor(reason) {
    reason = reason || "manual";
    if (suspensionReasons[reason]) {
      delete suspensionReasons[reason];
      suspensionCount = Math.max(0, suspensionCount - 1);
    }
    if (isSuspended()) return;
    resetSchedulerClock();
    markDirty();
  }

  function readPixels() {
    if (!initialized || observeContextLoss()) throw new Error("WebGL context is unavailable");
    var w = canvas.width, h = canvas.height;
    var buf = new Uint8Array(w * h * 4);
    gl.readPixels(0, 0, w, h, gl.RGBA, gl.UNSIGNED_BYTE, buf);
    /* flip vertically: GL origin is bottom-left */
    var row = w * 4;
    var scratch = new Uint8Array(row);
    for (var y = 0, half = Math.floor(h / 2); y < half; y++) {
      var top = y * row;
      var bottom = (h - 1 - y) * row;
      scratch.set(buf.subarray(top, top + row));
      buf.copyWithin(top, bottom, bottom + row);
      buf.set(scratch, bottom);
    }
    return buf;
  }

  return {
    init: init,
    setSize: setSize,
    renderAt: renderAt,
    readPixels: readPixels,
    currentPhase: currentPhase,
    resetTime: function () { loopT = 0; markDirty(); },
    setLoopTime: function (t) { loopT = t; markDirty(); },
    suspendFor: suspendFor,
    resumeFor: resumeFor,
    suspend: function () { suspendFor("manual"); },
    resume: function () { resumeFor("manual"); },
    setPlaying: function (v) {
      playing = !!v;
      cancelScheduledFrame();
      resetSchedulerClock();
      if (!playing) {
        reportPausedFps();
        return;
      }
      markDirty();
    },
    isPlaying: function () { return playing; },
    markDirty: markDirty,
    setTargetFps: setTargetFps,
    onFps: function (cb) { fpsCb = cb; },
    onContextChange: function (cb) { contextCb = cb; },
    isContextLost: isContextLost,
    canvas: function () { return canvas; },
    size: function () { return [canvas.width, canvas.height]; }
  };
})();

document.addEventListener("visibilitychange", function () {
  if (document.hidden) {
    Engine.suspendFor("visibility");
  } else {
    Engine.resumeFor("visibility");
  }
});

window.addEventListener("pagehide", function () {
  Engine.suspendFor("pagehide");
}, { passive: true });

window.addEventListener("pageshow", function () {
  Engine.resumeFor("pagehide");
}, { passive: true });
