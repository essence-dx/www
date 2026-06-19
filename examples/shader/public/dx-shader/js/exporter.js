/* Export pipeline: PNG stills, WebM/MP4 video, looping GIF. */

var Exporter = (function () {
  var cancelled = false;
  var busy = false;
  var activeCancel = null;
  var activeVisibilityCancel = null;
  var BYTES_PER_PIXEL = 4;
  var MIN_EXPORT_MEMORY_BYTES = 64 * 1024 * 1024;
  var MAX_EXPORT_MEMORY_BYTES = 384 * 1024 * 1024;

  function $(id) { return document.getElementById(id); }

  function showOverlay(title) {
    cancelled = false;
    activeCancel = null;
    activeVisibilityCancel = null;
    $("overlay-title").textContent = title;
    $("overlay-detail").textContent = "preparing";
    $("overlay-bar").style.transform = "scaleX(0)";
    $("overlay").hidden = false;
    $("overlay").classList.remove("is-hidden");
  }
  function setProgress(frac, detail) {
    $("overlay-bar").style.transform = "scaleX(" + Math.max(0, Math.min(1, frac)).toFixed(4) + ")";
    if (detail) $("overlay-detail").textContent = detail;
  }
  function hideOverlay() {
    $("overlay").hidden = true;
    $("overlay").classList.add("is-hidden");
  }

  function downloadBlob(blob, name) {
    var a = document.createElement("a");
    var url = URL.createObjectURL(blob);
    try {
      a.href = url;
      a.download = name;
      a.click();
    } finally {
      setTimeout(function () { URL.revokeObjectURL(url); }, 4000);
    }
  }

  function canvasToBlob(canvas, type) {
    return new Promise(function (resolve, reject) {
      try {
        canvas.toBlob(function (blob) {
          if (!blob) {
            reject(new Error("Canvas export produced no data"));
            return;
          }
          resolve(blob);
        }, type);
      } catch (e) {
        reject(e);
      }
    });
  }

  function stamp(P, ext) {
    var mode = MODES[P.mode].key;
    return "dx-shader-" + mode + "-" + String(Math.round(P.seed)).padStart(4, "0") + "." + ext;
  }

  function formatBytes(bytes) {
    var mb = bytes / (1024 * 1024);
    return mb >= 100 ? Math.round(mb) + " MB" : mb.toFixed(1) + " MB";
  }

  function deviceMemoryBudgetBytes() {
    var nav = window.navigator || {};
    var memoryGb = typeof nav.deviceMemory === "number" ? nav.deviceMemory : 4;
    var budget = memoryGb * 48 * 1024 * 1024;
    if (typeof nav.hardwareConcurrency === "number" && nav.hardwareConcurrency <= 4) {
      budget *= 0.75;
    }
    return Math.max(MIN_EXPORT_MEMORY_BYTES, Math.min(MAX_EXPORT_MEMORY_BYTES, Math.round(budget)));
  }

  function exportCost(label, bytes, detail) {
    return { label: label, bytes: Math.ceil(bytes), detail: detail || "" };
  }

  function estimateImageExportCost(w, h) {
    var frameBytes = w * h * BYTES_PER_PIXEL;
    return exportCost("PNG export", frameBytes * 3, w + "x" + h);
  }

  function estimateVideoExportCost(w, h, nFrames) {
    var frameBytes = w * h * BYTES_PER_PIXEL;
    var encodedBudget = Math.max(nFrames * 128 * 1024, frameBytes * 0.35);
    return exportCost("Video export", frameBytes * 4 + encodedBudget * 2.4, w + "x" + h + " / " + nFrames + " frames");
  }

  function encodedVideoBudgetBytes(w, h, nFrames) {
    var frameBytes = w * h * BYTES_PER_PIXEL;
    var expectedEncodedBytes = Math.max(nFrames * 128 * 1024, frameBytes * 0.35);
    var safeMuxBytes = Math.max(0, (deviceMemoryBudgetBytes() - frameBytes) / 5);
    return Math.max(1024 * 1024, Math.floor(Math.min(expectedEncodedBytes, safeMuxBytes)));
  }

  function estimateGifExportCost(w, h, nFrames) {
    var frameBytes = w * h * BYTES_PER_PIXEL;
    var encoderOverhead = Math.max(12 * 1024 * 1024, frameBytes * 4 + nFrames * 96 * 1024);
    return exportCost("GIF export", frameBytes * (nFrames + 6) + encoderOverhead, w + "x" + h + " / " + nFrames + " frames");
  }

  function canRunExport(cost) {
    var budget = deviceMemoryBudgetBytes();
    if (cost.bytes <= budget) return true;
    UI.toast(cost.label + " is too large for this device (" + formatBytes(cost.bytes) + " needs, " + formatBytes(budget) + " safe budget)");
    return false;
  }

  var runtimeScripts = {};
  function cleanupRuntimeScript(script, onLoad, onError) {
    script.removeEventListener("load", onLoad);
    script.removeEventListener("error", onError);
    script.remove();
  }

  function loadRuntimeScript(src, globalName) {
    if (window[globalName]) return Promise.resolve(window[globalName]);
    if (runtimeScripts[src]) return runtimeScripts[src];

    runtimeScripts[src] = new Promise(function (resolve, reject) {
      var script = document.createElement("script");
      script.src = src;
      script.async = true;
      script.dataset.dxShaderRuntime = globalName;
      function onLoad() {
        cleanupRuntimeScript(script, onLoad, onError);
        if (window[globalName]) {
          resolve(window[globalName]);
        } else {
          reject(new Error(globalName + " did not register"));
        }
      }
      function onError() {
        cleanupRuntimeScript(script, onLoad, onError);
        reject(new Error("Could not load " + src));
      }
      script.addEventListener("load", onLoad, { once: true });
      script.addEventListener("error", onError, { once: true });
      document.head.appendChild(script);
    }).catch(function (error) {
      delete runtimeScripts[src];
      throw error;
    });

    return runtimeScripts[src];
  }

  function evenRound(v) { return 2 * Math.round(v / 2); }

  function waitForVisibleOrCancel() {
    if (!document.hidden || cancelled) return Promise.resolve(!cancelled);
    return new Promise(function (resolve) {
      function onVisible() {
        if (document.hidden && !cancelled) return;
        document.removeEventListener("visibilitychange", onVisible);
        activeVisibilityCancel = null;
        resolve(!cancelled);
      }
      activeVisibilityCancel = function () {
        document.removeEventListener("visibilitychange", onVisible);
        activeVisibilityCancel = null;
        resolve(false);
      };
      document.addEventListener("visibilitychange", onVisible, { passive: true });
      if (cancelled && activeVisibilityCancel) activeVisibilityCancel();
    });
  }

  async function waitForEncoderQueue(encoder, hasEncoderError) {
    while (encoder.encodeQueueSize > 2 && !cancelled && !hasEncoderError()) {
      if (!await waitForVisibleOrCancel()) return false;
      await yieldToBrowser();
    }
    return !cancelled && !hasEncoderError();
  }

  function requestCancel() {
    cancelled = true;
    if (activeVisibilityCancel) activeVisibilityCancel();
    if (activeCancel) activeCancel();
  }

  /* ---------- PNG ---------- */
  async function exportPNG(P, aspect) {
    if (busy) return;
    var h = parseInt(P.imgRes, 10);
    var w = evenRound(h * aspect);
    var pngCost = estimateImageExportCost(w, h);
    if (!canRunExport(pngCost)) return;

    busy = true;
    var prev = Engine.size();
    var wasPlaying = Engine.isPlaying();
    Engine.suspendFor("export");
    try {
      Engine.setPlaying(false);
      Engine.setSize(w, h);
      Engine.renderAt(Engine.currentPhase());
      var blob = await canvasToBlob(Engine.canvas(), "image/png");
      downloadBlob(blob, stamp(P, "png"));
      FX.celebrate("Saved " + w + "\u00d7" + h + " PNG");
    } catch (e) {
      UI.toast("PNG export failed" + (e && e.message ? ": " + e.message : ""));
    } finally {
      Engine.setSize(prev[0], prev[1]);
      Engine.setPlaying(wasPlaying);
      Engine.renderAt(Engine.currentPhase());
      Engine.resumeFor("export");
      busy = false;
    }
  }

  /* ---------- Video (WebCodecs: deterministic offline encode, no MediaRecorder) ---------- */

  function videoDurationSec(P) {
    var v = String(P.vidLen || "l2");
    if (v.charAt(0) === "s") return Math.max(1, parseInt(v.slice(1), 10) || 5);
    return P.loop * Math.max(1, parseInt(v.slice(1), 10) || 1);
  }

  function videoBitrate(w, h, fps) {
    var px = w * h;
    var base = px >= 2560 * 1440 ? 14000000 : px >= 1920 * 1080 ? 9000000 : px >= 1280 * 720 ? 6000000 : 3500000;
    return fps >= 60 ? Math.round(base * 1.4) : base;
  }

  async function pickEncoderConfig(w, h, fps) {
    var candidates = [
      { codec: "vp09.00.10.08", codecId: "V_VP9" },
      { codec: "vp8", codecId: "V_VP8" }
    ];
    for (var i = 0; i < candidates.length; i++) {
      var cfg = {
        codec: candidates[i].codec,
        width: w, height: h,
        bitrate: videoBitrate(w, h, fps),
        framerate: fps
      };
      try {
        var sup = await VideoEncoder.isConfigSupported(cfg);
        if (sup && sup.supported) return { config: cfg, codecId: candidates[i].codecId };
      } catch (e) { /* try next codec */ }
    }
    return null;
  }

  async function exportVideo(P, aspect) {
    if (busy) return;
    if (typeof VideoEncoder === "undefined" || typeof VideoFrame === "undefined") {
      UI.toast("This browser has no WebCodecs support, use a current Chrome, Edge or Firefox");
      return;
    }
    var h = parseInt(P.vidRes, 10);
    var w = evenRound(h * aspect);
    var fps = parseInt(P.vidFps, 10) || 30;
    var totalSec = videoDurationSec(P);
    var nFrames = Math.max(2, Math.round(totalSec * fps));
    var videoCost = estimateVideoExportCost(w, h, nFrames);
    if (!canRunExport(videoCost)) return;

    busy = true;

    var picked = await pickEncoderConfig(w, h, fps);
    if (!picked) {
      busy = false;
      UI.toast("No supported video codec (VP9/VP8) found");
      return;
    }

    try {
      await loadRuntimeScript("/dx-shader/js/webmmux.js", "WebMMux");
    } catch (e) {
      busy = false;
      UI.toast("Video exporter unavailable");
      return;
    }
    if (!WebMMux.muxAsync) {
      busy = false;
      UI.toast("Video exporter unavailable");
      return;
    }

    var prev = Engine.size();
    var wasPlaying = Engine.isPlaying();

    showOverlay("Rendering video");
    Engine.suspendFor("export");
    Engine.setPlaying(false);
    Engine.setSize(w, h);

    var encFrames = [];
    var encodedBytes = 0;
    var maxEncodedBytes = encodedVideoBudgetBytes(w, h, nFrames);
    var encError = null;
    var encoder = null;
    try {
      encoder = new VideoEncoder({
        output: function (chunk) {
          if (cancelled || encError) return;
          if (encodedBytes + chunk.byteLength > maxEncodedBytes) {
            encError = new Error("Encoded video exceeded the safe memory budget");
            return;
          }
          var data = new Uint8Array(chunk.byteLength);
          chunk.copyTo(data);
          encodedBytes += data.byteLength;
          encFrames.push({
            data: data,
            timestampMs: chunk.timestamp / 1000,
            key: chunk.type === "key"
          });
        },
        error: function (e) { encError = e; }
      });
      encoder.configure(picked.config);
      activeCancel = function () {
        try { encoder.reset(); } catch (ignoreReset) {}
      };
    } catch (e) {
      encFrames.length = 0;
      restore();
      UI.toast("Video encoder unavailable" + (e && e.message ? ": " + e.message : ""));
      return;
    }

    var canvas = Engine.canvas();
    var usPerFrame = 1e6 / fps;

    try {
      for (var f = 0; f < nFrames; f++) {
        if (cancelled || encError) break;
        if (!await waitForVisibleOrCancel()) break;

        var t = f / fps;
        Engine.setLoopTime(t % P.loop);
        Engine.renderAt((t % P.loop) / P.loop);

        var vf = new VideoFrame(canvas, {
          timestamp: Math.round(f * usPerFrame),
          duration: Math.round(usPerFrame)
        });
        /* keyframe every 2 seconds keeps files small and seekable */
        try {
          encoder.encode(vf, { keyFrame: f % (fps * 2) === 0 });
        } finally {
          vf.close();
        }

        setProgress(0.9 * (f + 1) / nFrames,
          "frame " + (f + 1) + "/" + nFrames + " \u00b7 " + w + "\u00d7" + h + " @ " + fps + "fps");

        /* backpressure: never let the encoder queue grow unbounded */
        if (!await waitForEncoderQueue(encoder, function () { return encError; })) break;
        if (f % 8 === 7) await yieldToBrowser();
      }

      if (!cancelled && !encError) {
        setProgress(0.93, "finalizing encode");
        await encoder.flush();
      }
    } catch (e) {
      encError = e;
    }

    if (cancelled || encError) {
      try { encoder.reset(); } catch (ignoreReset) {}
    }
    try { encoder.close(); } catch (ignore) {}
    activeCancel = null;

    /* restore live view before the (fast) muxing step */
    Engine.setSize(prev[0], prev[1]);
    Engine.setPlaying(wasPlaying);
    Engine.resumeFor("export");

    if (cancelled) { encFrames.length = 0; hideOverlay(); busy = false; return; }
    if (encError || !encFrames.length) {
      encFrames.length = 0;
      hideOverlay(); busy = false;
      UI.toast("Video encode failed" + (encError && encError.message ? ": " + encError.message : ""));
      return;
    }

    setProgress(0.97, "writing webm container");
    await yieldToBrowser();
    if (cancelled) { encFrames.length = 0; hideOverlay(); busy = false; return; }
    var muxCost = exportCost("Video mux", encodedBytes * 5 + w * h * BYTES_PER_PIXEL, w + "x" + h + " / " + formatBytes(encodedBytes));
    if (!canRunExport(muxCost)) { encFrames.length = 0; hideOverlay(); busy = false; return; }
    var muxOptions = {
      codecId: picked.codecId,
      width: w, height: h,
      durationMs: totalSec * 1000,
      frames: encFrames
    };
    var webm = null;
    try {
      webm = await WebMMux.muxAsync(muxOptions, function () { return cancelled; }, yieldToBrowser);
    } catch (e) {
      encFrames.length = 0;
      hideOverlay();
      busy = false;
      UI.toast("Video mux failed" + (e && e.message ? ": " + e.message : ""));
      return;
    }
    encFrames.length = 0;
    if (cancelled || !webm) { hideOverlay(); busy = false; return; }

    hideOverlay();
    busy = false;
    downloadBlob(webm, stamp(P, "webm"));
    FX.celebrate("Saved " + totalSec.toFixed(1) + "s video \u00b7 " + w + "\u00d7" + h + " @ " + fps + "fps");

    function restore() {
      Engine.setSize(prev[0], prev[1]);
      Engine.setPlaying(wasPlaying);
      Engine.resumeFor("export");
      hideOverlay();
      busy = false;
    }
  }

  /* ---------- GIF (offline, deterministic, perfect loop) ---------- */
  async function exportGIF(P, aspect) {
    if (busy) return;
    var w = parseInt(P.gifW, 10);
    var h = evenRound(w / aspect);
    var fps = parseInt(P.gifFps, 10);
    var nFrames = Math.max(2, Math.round(P.loop * fps));
    var gifCost = estimateGifExportCost(w, h, nFrames);
    if (!canRunExport(gifCost)) return;

    busy = true;

    try {
      await loadRuntimeScript("/dx-shader/js/gifenc.js", "GIFEnc");
    } catch (e) {
      busy = false;
      UI.toast("GIF encoder unavailable");
      return;
    }

    var prev = Engine.size();
    var wasPlaying = Engine.isPlaying();
    Engine.suspendFor("export");
    Engine.setPlaying(false);

    showOverlay("Rendering GIF");
    Engine.setSize(w, h);

    var frames = [];
    var data = null;
    var gifError = null;

    try {
      for (var f = 0; f < nFrames; f++) {
        if (cancelled) break;
        if (!await waitForVisibleOrCancel()) break;
        Engine.renderAt(f / nFrames);
        frames.push(Engine.readPixels());
        setProgress(0.4 * (f + 1) / nFrames, "capturing " + (f + 1) + "/" + nFrames);
        if (f % 4 === 3) await yieldToBrowser();
      }

      if (!cancelled) {
        data = await GIFEnc.encode({
          frames: frames, width: w, height: h, fps: fps,
          dither: P.gifDither, loop: P.gifLoop,
          onProgress: function (frac, detail) { setProgress(0.4 + 0.6 * frac, "encoding \u00b7 " + detail); },
          isCancelled: function () { return cancelled; }
        });
      }
    } catch (e) {
      gifError = e;
    } finally {
      frames.length = 0;
      Engine.setSize(prev[0], prev[1]);
      Engine.setPlaying(wasPlaying);
      Engine.resumeFor("export");
      hideOverlay();
      busy = false;
    }

    if (gifError) {
      UI.toast("GIF export failed" + (gifError && gifError.message ? ": " + gifError.message : ""));
      return;
    }
    if (data && !cancelled) {
      downloadBlob(new Blob([data], { type: "image/gif" }), stamp(P, "gif"));
      FX.celebrate("Saved " + nFrames + "-frame looping GIF (" + w + "\u00d7" + h + ")");
    }
  }

  function wait(ms) { return new Promise(function (r) { setTimeout(r, ms); }); }
  function yieldToBrowser() {
    if (window.scheduler && typeof window.scheduler.yield === "function") {
      return window.scheduler.yield();
    }
    if (typeof window.requestIdleCallback === "function") {
      return new Promise(function (resolve) {
        window.requestIdleCallback(function () { resolve(); }, { timeout: 80 });
      });
    }
    return wait(0);
  }

  function bindCancelButton() {
    var button = $("overlay-cancel");
    if (!button || button.dataset.dxShaderCancelBound === "true") return;
    button.dataset.dxShaderCancelBound = "true";
    button.addEventListener("click", requestCancel);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", bindCancelButton, { once: true });
  } else {
    bindCancelButton();
  }

  return {
    exportPNG: exportPNG,
    exportVideo: exportVideo,
    exportGIF: exportGIF,
    loadRuntimeScript: loadRuntimeScript,
    yieldToBrowser: yieldToBrowser,
    canvasToBlob: canvasToBlob,
    downloadBlob: downloadBlob,
    canRunExport: canRunExport,
    estimateImageExportCost: estimateImageExportCost,
    isBusy: function () { return busy; }
  };
})();
