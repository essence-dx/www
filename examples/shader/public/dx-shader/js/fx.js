/* Celebration effect for finished exports: a palette-colored particle
   burst with an expanding ring and a success toast with a drawn check.
   Pure transform/opacity + one lightweight 2d canvas, auto-cleans. */

var FX = (function () {
  var MAX_BURST_PIXELS = 1400000;
  var activeBurstCancel = null;

  function mediaMatches(query) {
    try {
      return Boolean(window.matchMedia && window.matchMedia(query).matches);
    } catch (e) {
      return false;
    }
  }

  function shouldSkipCelebrationBurst() {
    var nav = window.navigator || {};
    var connection = nav.connection || nav.mozConnection || nav.webkitConnection;
    var effectiveType = (connection && connection.effectiveType) || "";
    return Boolean(connection && connection.saveData) ||
      (typeof nav.deviceMemory === "number" && nav.deviceMemory <= 4) ||
      (typeof nav.hardwareConcurrency === "number" && nav.hardwareConcurrency <= 4) ||
      /^(slow-2g|2g)$/.test(effectiveType) ||
      mediaMatches("(prefers-reduced-motion: reduce)") ||
      mediaMatches("(prefers-reduced-data: reduce)") ||
      mediaMatches("(update: slow)") ||
      mediaMatches("(hover: none)") ||
      mediaMatches("(pointer: coarse)");
  }

  function celebrate(message) {
    if (!shouldSkipCelebrationBurst()) burst();
    successToast(message);
  }

  function burst() {
    if (activeBurstCancel) activeBurstCancel();

    var c = document.createElement("canvas");
    c.className = "fx-layer";
    var dpr = Math.min(window.devicePixelRatio || 1, 2);
    var maxDpr = Math.sqrt(MAX_BURST_PIXELS / Math.max(1, innerWidth * innerHeight));
    dpr = Math.max(0.5, Math.min(dpr, maxDpr));
    c.width = Math.max(1, Math.round(innerWidth * dpr));
    c.height = Math.max(1, Math.round(innerHeight * dpr));
    document.body.appendChild(c);
    var ctx = c.getContext("2d");
    ctx.scale(dpr, dpr);

    var cx = innerWidth / 2;
    var cy = innerHeight * 0.42;

    var ring = document.createElement("div");
    ring.className = "fx-ring";
    ring.style.left = cx + "px";
    ring.style.top = cy + "px";
    document.body.appendChild(ring);
    var ringTimer = setTimeout(function () { ring.remove(); }, 750);
    var frameId = 0;

    function cleanup() {
      clearTimeout(ringTimer);
      if (frameId) cancelAnimationFrame(frameId);
      c.remove();
      ring.remove();
      if (activeBurstCancel === cleanup) activeBurstCancel = null;
    }
    activeBurstCancel = cleanup;

    var colors = [P.c1, P.c2, P.c3, P.c4, "#ffffff"];
    var parts = [];
    var N = 110;
    for (var i = 0; i < N; i++) {
      var a = Math.random() * Math.PI * 2;
      var sp = 4 + Math.random() * 13;
      parts.push({
        x: cx, y: cy,
        vx: Math.cos(a) * sp,
        vy: Math.sin(a) * sp - 3,
        r: 1.5 + Math.random() * 3.2,
        rot: Math.random() * Math.PI,
        vr: (Math.random() - 0.5) * 0.3,
        col: colors[(Math.random() * colors.length) | 0],
        shape: Math.random() < 0.4 ? 1 : 0,
        decay: 0.012 + Math.random() * 0.006,
        life: 1
      });
    }

    var t0 = performance.now();
    function frame(now) {
      frameId = 0;
      var dt = Math.min((now - t0) / 1000, 2);
      ctx.clearRect(0, 0, innerWidth, innerHeight);
      var alive = false;
      for (var i = 0; i < parts.length; i++) {
        var p = parts[i];
        p.x += p.vx; p.y += p.vy;
        p.vx *= 0.965; p.vy = p.vy * 0.965 + 0.32;
        p.rot += p.vr;
        p.life -= p.decay;
        if (p.life <= 0) continue;
        alive = true;
        ctx.globalAlpha = Math.max(p.life, 0);
        ctx.fillStyle = p.col;
        if (p.shape === 1) {
          ctx.save();
          ctx.translate(p.x, p.y);
          ctx.rotate(p.rot);
          ctx.fillRect(-p.r, -p.r * 0.55, p.r * 2, p.r * 1.1);
          ctx.restore();
        } else {
          ctx.beginPath();
          ctx.arc(p.x, p.y, p.r * p.life, 0, Math.PI * 2);
          ctx.fill();
        }
      }
      if (alive && dt < 2) frameId = requestAnimationFrame(frame);
      else cleanup();
    }
    frameId = requestAnimationFrame(frame);
  }

  function createSuccessIcon() {
    var svgNS = "http://www.w3.org/2000/svg";
    var svg = document.createElementNS(svgNS, "svg");
    var circle = document.createElementNS(svgNS, "circle");
    var path = document.createElementNS(svgNS, "path");

    svg.setAttribute("viewBox", "0 0 16 16");
    svg.setAttribute("aria-hidden", "true");
    circle.setAttribute("cx", "8");
    circle.setAttribute("cy", "8");
    circle.setAttribute("r", "6.6");
    circle.setAttribute("fill", "none");
    circle.setAttribute("stroke", "#7de8a4");
    circle.setAttribute("stroke-width", "1.4");
    circle.setAttribute("opacity", "0.6");
    path.setAttribute("d", "M5 8.2 L7.2 10.4 L11.2 5.8");
    path.setAttribute("fill", "none");
    path.setAttribute("stroke", "#7de8a4");
    path.setAttribute("stroke-width", "1.8");
    path.setAttribute("stroke-linecap", "round");
    path.setAttribute("stroke-linejoin", "round");
    svg.appendChild(circle);
    svg.appendChild(path);
    return svg;
  }

  var toastTimer = null;
  function successToast(message) {
    var t = document.getElementById("toast");
    var label = t.querySelector("span");
    if (!label) {
      t.textContent = "";
      t.appendChild(createSuccessIcon());
      label = document.createElement("span");
      t.appendChild(label);
    }
    label.textContent = message;
    t.classList.add("success");
    t.hidden = false;
    clearTimeout(toastTimer);
    toastTimer = setTimeout(function () {
      t.hidden = true;
      t.classList.remove("success");
      label.textContent = "";
    }, 3400);
  }

  return { celebrate: celebrate };
})();
