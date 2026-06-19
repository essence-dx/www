import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { test } from "node:test";
import assert from "node:assert/strict";

const source = (path: string) =>
  readFileSync(new URL(`../${path}`, import.meta.url), "utf8");

const assetBytes = (path: string) =>
  statSync(new URL(`../${path}`, import.meta.url)).size;

const collectFiles = (directory: URL): URL[] =>
  readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const child = new URL(`${entry.name}${entry.isDirectory() ? "/" : ""}`, directory);
    return entry.isDirectory() ? collectFiles(child) : [child];
  });

test("brands the shipped surface as DX Shader", () => {
  const page = source("app/page.tsx");
  const layout = source("app/layout.tsx");
  const docs = source("public/dx-shader/docs.html");
  const main = source("public/dx-shader/js/main.js");
  const exporter = source("public/dx-shader/js/exporter.js");
  const modals = source("public/dx-shader/js/modals.js");
  const webmMuxer = source("public/dx-shader/js/webmmux.js");

  assert.match(page, /DX Shader/);
  assert.match(page, /data-dx-template="dx-shader"/);
  assert.match(page, /href="\/dx-shader\/docs\.html"/);
  assert.match(layout, /\/dx-shader\/assets\/favicon\.svg/);
  assert.match(docs, /DX Shader documentation/);
  assert.match(docs, /Legacy <code>LMN1\.<\/code> links still load/);
  assert.doesNotMatch(page, /LUMEN|\/lumen\//);
  assert.doesNotMatch(layout, /\/lumen\//);
  assert.doesNotMatch(docs, /LUMEN|fonts\.googleapis\.com|fonts\.gstatic\.com/);
  assert.match(main, /var STYLES_KEY = "dx-shader-styles-v1"/);
  assert.match(main, /var LEGACY_STYLES_KEY = "lumen-styles-v1"/);
  assert.match(main, /var SHARE_PREFIX = "DXS1\."/);
  assert.match(main, /var LEGACY_SHARE_PREFIX = "LMN1\."/);
  assert.match(main, /hash\.indexOf\(SHARE_PREFIX\) === 0 \|\| hash\.indexOf\(LEGACY_SHARE_PREFIX\) === 0/);
  assert.match(exporter, /return "dx-shader-" \+ mode/);
  assert.match(modals, /"dx-shader-set-" \+ String\(i \+ 1\)/);
  assert.match(modals, /Exporter\.downloadBlob\(zip, "dx-shader-set-" \+ MODES\[P\.mode\]\.key \+ "\.zip"\)/);
  assert.match(webmMuxer, /strPayload\("dx-shader"\)/);
});

test("keeps project-owned source TypeScript-first where safe", () => {
  const readme = source("README.md");

  assert.match(readme, /New project-owned tests and source\s+should use `\.ts` or `\.tsx`/);
  assert.match(readme, /browser-delivered runtime\s+artifacts/);
  assert.doesNotMatch(readme, /\.mjs|\.cjs/);
});

test("uses self-hosted JetBrains Mono and Vercel-style dark/light themes", () => {
  const layout = source("app/layout.tsx");
  const css = source("styles/dx-shader.css");
  const publicCss = source("public/dx-shader/styles.css");
  const vercel = source("vercel.json");
  const docs = source("public/dx-shader/docs.html");

  assert.equal(css, publicCss);
  assert.match(css, /@font-face\s*\{\s*font-family: "JetBrains Mono"/);
  assert.match(css, /jetbrains-mono-latin-400-normal\.woff2/);
  assert.match(css, /jetbrains-mono-latin-600-normal\.woff2/);
  assert.doesNotMatch(css, /jetbrains-mono-latin-500-normal\.woff2/);
  assert.doesNotMatch(css, /jetbrains-mono-latin-700-normal\.woff2/);
  assert.match(css, /--font-ui: "JetBrains Mono"/);
  assert.match(css, /color-scheme: dark/);
  assert.match(css, /@media \(prefers-color-scheme: light\)/);
  assert.match(css, /--bg: #000000/);
  assert.match(css, /--bg: #ffffff/);
  assert.match(css, /--primary-bg: #ffffff/);
  assert.match(css, /--primary-bg: #000000/);
  assert.match(layout, /rel="preload"[\s\S]*jetbrains-mono-latin-400-normal\.woff2/);
  assert.doesNotMatch(layout, /rel="preload"[\s\S]*jetbrains-mono-latin-600-normal\.woff2/);
  assert.match(vercel, /font-src 'self'/);
  assert.doesNotMatch(docs, /fonts\.googleapis\.com|fonts\.gstatic\.com/);

  let totalFontBytes = 0;
  for (const weight of [400, 600]) {
    const font = statSync(
      new URL(`../public/dx-shader/fonts/jetbrains-mono-latin-${weight}-normal.woff2`, import.meta.url)
    );
    totalFontBytes += font.size;
    assert.ok(font.size > 20_000, `JetBrains Mono ${weight} should be self-hosted`);
  }
  assert.ok(totalFontBytes < 45_000, "loaded JetBrains Mono weights should stay compact");
  assert.equal(
    existsSync(new URL("../public/dx-shader/fonts/jetbrains-mono-latin-500-normal.woff2", import.meta.url)),
    false
  );
  assert.equal(
    existsSync(new URL("../public/dx-shader/fonts/jetbrains-mono-latin-700-normal.woff2", import.meta.url)),
    false
  );
});

test("renders a DX loader before the shader runtime boots", () => {
  const page = source("app/page.tsx");
  const css = source("styles/dx-shader.css");

  assert.match(page, /id="shader-boot-loader"/);
  assert.match(page, /name="pack:dx"/);
  assert.match(page, /name="lucide:loader-circle"/);
  assert.match(page, /data-lucide-icon="loader-circle"/);
  assert.match(css, /\.shader-loader\b/);
  assert.match(css, /@keyframes shader-loader-shimmer/);
  assert.match(css, /\[data-loader-state="ready"\]/);
});

test("defers WebGL initialization behind a guarded boot scheduler", () => {
  const main = source("public/dx-shader/js/main.js");

  assert.match(main, /function bootWhenDomReady\(\)/);
  assert.match(main, /if \(document\.readyState === "loading"\) \{/);
  assert.match(main, /document\.addEventListener\("DOMContentLoaded", bootWhenDomReady, \{ once: true \}\)/);
  assert.match(main, /else \{\n  bootWhenDomReady\(\);\n\}/);
  assert.match(main, /function bootShaderStudio\(\)/);
  assert.match(main, /function scheduleShaderBoot\(/);
  assert.match(main, /function shouldUseManualShaderStart\(/);
  assert.match(main, /function shouldUseReducedRenderProfile\(\)/);
  assert.match(main, /function guardedRenderScale\(\)/);
  assert.match(main, /function startWhenVisible\(start, delayMs\)/);
  assert.match(main, /if \(!document\.hidden\) \{/);
  assert.match(main, /setBootLoaderState\("loading", "Waiting for visible tab"\)/);
  assert.match(main, /document\.addEventListener\("visibilitychange", function onVisible\(\)/);
  assert.match(main, /setTimeout\(function \(\) \{\n      if \(document\.hidden\) \{/);
  assert.doesNotMatch(main, /setTimeout\(start, delayMs\)/);
  assert.match(main, /requestIdleCallback/);
});

test("keeps constrained devices behind manual or visible startup", () => {
  const main = source("public/dx-shader/js/main.js");
  const manualStart = main.match(
    /function shouldUseManualShaderStart\(\) \{([\s\S]*?)\n\}/
  );
  const safetySignals = main.match(
    /function shaderSafetySignals\(\) \{([\s\S]*?)\n\}/
  );
  const reducedRender = main.match(
    /function shouldUseReducedRenderProfileFromSignals\(signals\) \{([\s\S]*?)\n\}/
  );
  const scheduler = main.match(
    /function scheduleShaderBoot\(start\) \{([\s\S]*?)\n\}/
  );

  assert.ok(manualStart, "manual-start predicate should stay explicit");
  assert.ok(safetySignals, "shader safety signals should stay explicit");
  assert.ok(reducedRender, "reduced-render predicate should stay explicit");
  assert.ok(scheduler, "shader boot scheduler should stay explicit");
  assert.match(safetySignals[1], /nav\.deviceMemory === "number" && nav\.deviceMemory <= 4/);
  assert.match(safetySignals[1], /nav\.hardwareConcurrency === "number" && nav\.hardwareConcurrency <= 4/);
  assert.match(safetySignals[1], /connection && connection\.saveData/);
  assert.match(safetySignals[1], /\^\(slow-2g\|2g\)\$/);
  assert.match(safetySignals[1], /prefers-reduced-data: reduce/);
  assert.match(safetySignals[1], /prefers-reduced-motion: reduce/);
  assert.match(safetySignals[1], /isEdgeBrowser\(\)/);
  assert.match(manualStart[1], /signals\.edgeBrowser/);
  assert.match(manualStart[1], /signals\.reducedMotion/);
  assert.match(manualStart[1], /shouldUseReducedRenderProfileFromSignals\(signals\)/);
  assert.match(reducedRender[1], /signals\.lowMemory/);
  assert.match(reducedRender[1], /signals\.lowCores/);
  assert.match(reducedRender[1], /signals\.saveData/);
  assert.match(reducedRender[1], /signals\.slowConnection/);
  assert.match(reducedRender[1], /signals\.reducedData/);
  assert.doesNotMatch(reducedRender[1], /isEdgeBrowser\(\)/);
  assert.doesNotMatch(reducedRender[1], /signals\.edgeBrowser|signals\.reducedMotion/);
  assert.match(main, /if \(cachedWeakRenderProfile === null\) cachedWeakRenderProfile = shouldUseReducedRenderProfile\(\)/);
  assert.doesNotMatch(main, /cachedWeakDeviceProfile = shouldUseManualShaderStart\(\)/);
  assert.match(scheduler[1], /data-loader-mode", "manual"/);
  assert.match(main, /if \(shouldUseManualShaderStart\(\)\) \{/);
  assert.match(scheduler[1], /setBootLoaderState\("manual", "Safe start waiting"\)/);
  assert.match(scheduler[1], /action\.addEventListener\("click", function \(\) \{/);
  assert.match(scheduler[1], /\{ once: true \}/);
  assert.match(scheduler[1], /startWhenVisible\(start, 120\)/);
  assert.doesNotMatch(scheduler[1], /Engine\.init\(/);
});

test("keeps the WebGL engine idle when animation does not need a frame", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const buildPipeline = engine.slice(engine.indexOf("function buildPipeline"), engine.indexOf("function notifyContext"));
  const setSize = engine.slice(engine.indexOf("function setSize"), engine.indexOf("function pushUniforms"));
  const scheduleFrame = engine.slice(engine.indexOf("function scheduleFrame"), engine.indexOf("function markDirty"));
  const resumeFor = engine.slice(engine.indexOf("function resumeFor"), engine.indexOf("function readPixels"));
  const setPlaying = engine.slice(engine.indexOf("setPlaying: function"), engine.indexOf("isPlaying: function"));

  assert.match(engine, /preserveDrawingBuffer:\s*false/);
  assert.match(engine, /powerPreference:\s*"low-power"/);
  assert.match(engine, /failIfMajorPerformanceCaveat:\s*true/);
  assert.match(engine, /var MAX_RENDER_DIMENSION = 8192/);
  assert.match(engine, /function safeRenderDimension\(value\)/);
  assert.match(buildPipeline, /var vertexShader = compile\(gl\.VERTEX_SHADER, VERT_SRC\)/);
  assert.match(buildPipeline, /var fragmentShader = compile\(gl\.FRAGMENT_SHADER, FRAG_SRC\)/);
  assert.match(buildPipeline, /gl\.deleteShader\(vertexShader\)/);
  assert.match(buildPipeline, /gl\.deleteShader\(fragmentShader\)/);
  assert.ok(buildPipeline.indexOf("gl.linkProgram(program)") < buildPipeline.indexOf("gl.deleteShader(vertexShader)"));
  assert.match(engine, /var frameHandle = 0/);
  assert.match(engine, /var frameTimer = 0/);
  assert.match(engine, /var targetFrameIntervalMs = 0/);
  assert.match(engine, /var lastDrawAt = 0/);
  assert.match(engine, /function cancelScheduledFrame\(\)/);
  assert.match(engine, /function scheduleFrame\(delayMs\)/);
  assert.match(engine, /function markDirty\(\)/);
  assert.match(engine, /function setTargetFps\(fps\)/);
  assert.match(engine, /function resetSchedulerClock\(now\)/);
  assert.match(engine, /lastDrawAt = 0/);
  assert.match(engine, /cancelAnimationFrame\(frameHandle\)/);
  assert.match(engine, /clearTimeout\(frameTimer\)/);
  assert.match(engine, /targetFrameIntervalMs = safeFps > 0 && safeFps < 55 \? 1000 \/ safeFps : 0/);
  assert.doesNotMatch(engine, /targetFrameIntervalMs = safeFps > 0 \? 1000 \/ safeFps : 0/);
  assert.match(engine, /if \(!playing && !dirty\) return/);
  assert.match(engine, /if \(playing && targetFrameIntervalMs && lastDrawAt && now - lastDrawAt < targetFrameIntervalMs\)/);
  assert.match(engine, /scheduleFrame\(targetFrameIntervalMs - \(now - lastDrawAt\)\)/);
  assert.match(engine, /targetFrameIntervalMs - \(performance\.now\(\) - lastDrawAt\)/);
  assert.match(engine, /frameTimer = setTimeout\(function \(\) \{/);
  assert.match(scheduleFrame, /if \(isSuspended\(\) \|\| \(!playing && !dirty\)\) return/);
  assert.match(engine, /if \(playing && targetFrameIntervalMs && frameTimer\) return/);
  assert.match(resumeFor, /resetSchedulerClock\(\)/);
  assert.match(setPlaying, /cancelScheduledFrame\(\)/);
  assert.match(setPlaying, /reportPausedFps\(\)/);
  assert.match(setPlaying, /resetSchedulerClock\(\)/);
  assert.doesNotMatch(scheduleFrame, /observeContextLoss\(\)/);
  assert.match(engine, /function observeContextLoss\(\)/);
  assert.match(engine, /if \(contextLost\) return/);
  assert.match(engine, /lastDrawAt = now/);
  assert.match(engine, /setTargetFps: setTargetFps/);
  assert.match(engine, /document\.addEventListener\("visibilitychange"/);
  assert.match(engine, /window\.addEventListener\("pagehide"/);
  assert.match(engine, /window\.addEventListener\("pageshow"/);
  assert.match(setSize, /w = safeRenderDimension\(w\)/);
  assert.match(setSize, /h = safeRenderDimension\(h\)/);
  assert.match(setSize, /if \(!w \|\| !h\) return/);
  assert.match(setSize, /if \(canvas\.width === w && canvas\.height === h\) return/);
  assert.match(setSize, /if \(canvas\.width !== w\) canvas\.width = w/);
  assert.match(setSize, /if \(canvas\.height !== h\) canvas\.height = h/);
  assert.doesNotMatch(setSize, /canvas\.width = w;\s*canvas\.height = h/);
  assert.ok(setSize.indexOf("if (canvas.width === w && canvas.height === h) return") < setSize.indexOf("if (canvas.width !== w) canvas.width = w"));
  assert.ok(setSize.indexOf("if (canvas.height !== h) canvas.height = h") < setSize.indexOf("gl.viewport(0, 0, w, h)"));
});

test("uses one live parameter snapshot per rendered engine tick", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const tick = engine.slice(engine.indexOf("function tick(now)"), engine.indexOf("function reportFrameTime"));
  const renderAt = engine.slice(engine.indexOf("function renderAt"), engine.indexOf("function currentPhase"));

  assert.equal((tick.match(/getParams\(\)/g) ?? []).length, 1);
  assert.doesNotMatch(tick, /renderAt\(currentPhase\(\)\)/);
  assert.doesNotMatch(tick, /currentPhase\(\)/);
  assert.match(tick, /renderAt\(\(loopT \/ P\.loop\) % 1, P\)/);
  assert.match(renderAt, /function renderAt\(phase, P\)/);
  assert.match(renderAt, /P = P \|\| getParams\(\)/);
});

test("keeps stable WebGL uniforms cached off the animation hot path", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const buildPipeline = engine.slice(engine.indexOf("function buildPipeline"), engine.indexOf("function notifyContext"));
  const pushUniforms = engine.slice(engine.indexOf("function pushUniforms"), engine.indexOf("function renderAt"));

  assert.match(engine, /var uniformCache = {}/);
  assert.match(engine, /function resetUniformCache\(\)/);
  assert.match(buildPipeline, /resetUniformCache\(\)/);
  assert.match(engine, /function uploadUniform1f\(name, value\)/);
  assert.match(engine, /function uploadUniform1i\(name, value\)/);
  assert.match(engine, /function uploadUniform2f\(name, x, y\)/);
  assert.match(engine, /function uploadUniform3fv\(name, value\)/);
  assert.match(engine, /function uploadUniform4f\(name, x, y, z, w\)/);
  assert.doesNotMatch(engine, /uniformCache\[name\] = \[/);
  assert.match(engine, /if \(!c\) c = uniformCache\[name\] = new Array\(2\)/);
  assert.match(engine, /if \(!c\) c = uniformCache\[name\] = new Array\(3\)/);
  assert.match(engine, /if \(!c\) c = uniformCache\[name\] = new Array\(4\)/);
  assert.match(pushUniforms, /gl\.uniform1f\(uniforms\.u_phase, phase\)/);
  assert.match(pushUniforms, /uploadUniform2f\("u_res", canvas\.width, canvas\.height\)/);
  assert.match(pushUniforms, /uploadUniform1f\("u_seed", P\.seed\)/);
  assert.match(pushUniforms, /uploadUniform1i\("u_mode", P\.mode\)/);
  assert.match(pushUniforms, /uploadUniform3fv\("u_c1", hexToRgb01\(P\.c1\)\)/);
  assert.match(pushUniforms, /uploadUniform4f\("u_g1", g\[0\], g\[1\], g\[2\], g\[3\]\)/);
  assert.doesNotMatch(pushUniforms, /gl\.uniform1f\(uniforms\.u_seed/);
  assert.doesNotMatch(pushUniforms, /gl\.uniform1i\(uniforms\.u_mode/);
  assert.doesNotMatch(pushUniforms, /gl\.uniform3fv\(uniforms\.u_c1/);
  assert.doesNotMatch(pushUniforms, /gl\.uniform4f\(uniforms\.u_g1/);
});

test("keeps color uniform conversion cached off the render loop", () => {
  const palettes = source("public/dx-shader/js/palettes.js");

  assert.match(palettes, /var rgb01Cache = {}/);
  assert.match(palettes, /if \(rgb01Cache\[hex\]\) return rgb01Cache\[hex\]/);
  assert.match(palettes, /return rgb01Cache\[hex\] = \[/);
});

test("keeps WebGL readbacks single-buffered for exports", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const readPixels = engine.slice(engine.indexOf("function readPixels"), engine.indexOf("return {"));

  assert.equal((readPixels.match(/new Uint8Array\(w \* h \* 4\)/g) ?? []).length, 1);
  assert.doesNotMatch(readPixels, /var flipped = new Uint8Array\(w \* h \* 4\)/);
  assert.match(readPixels, /gl\.readPixels\(0, 0, w, h, gl\.RGBA, gl\.UNSIGNED_BYTE, buf\)/);
  assert.match(readPixels, /var scratch = new Uint8Array\(row\)/);
  assert.match(readPixels, /Math\.floor\(h \/ 2\)/);
  assert.match(readPixels, /buf\.copyWithin\(top, bottom, bottom \+ row\)/);
  assert.match(readPixels, /return buf/);
});

test("keeps a static shader fallback available before runtime scripts execute", () => {
  const page = source("app/page.tsx");
  const css = source("styles/dx-shader.css");

  assert.match(page, /<noscript>/);
  assert.match(page, /className="shader-static-fallback"/);
  assert.match(page, /id="shader-static-fallback"/);
  assert.match(page, /JavaScript is required to start the live shader/);
  assert.match(css, /\.shader-static-fallback\b/);
  assert.match(css, /\.shader-static-fallback::before\b/);
  assert.match(css, /\[data-shader-ready="true"\] \.shader-static-fallback/);
});

test("uses one page root so DX static export keeps the full shader surface", () => {
  const page = source("app/page.tsx");

  assert.doesNotMatch(page, /return \(\s*<>/);
  assert.match(page, /return \(\s*<main className="dx-shader-root"/);
  assert.match(page, /<div className="app"/);
  assert.match(page, /<section className="stage" id="stage">/);
  assert.match(page, /<aside className="rail" id="rail" \/>/);
  assert.match(page, /<script defer src="\/dx-shader\/js\/main\.js"><\/script>/);
});

test("keeps runtime scripts ordered for dependency-safe deferred execution", () => {
  const page = source("app/page.tsx");
  const scripts = Array.from(page.matchAll(/<script defer src="([^"]+)"><\/script>/g)).map(
    ([, src]) => src
  );

  assert.deepEqual(scripts, [
    "/dx-shader/js/palettes.js",
    "/dx-shader/js/shaders.js",
    "/dx-shader/js/engine.js",
    "/dx-shader/js/ui.js",
    "/dx-shader/js/main.js",
  ]);
});

test("keeps export codecs off the initial route and lazy-loads them on demand", () => {
  const page = source("app/page.tsx");
  const main = source("public/dx-shader/js/main.js");
  const exporter = source("public/dx-shader/js/exporter.js");
  const modals = source("public/dx-shader/js/modals.js");

  assert.doesNotMatch(page, /src="\/dx-shader\/js\/exporter\.js"/);
  assert.doesNotMatch(page, /src="\/dx-shader\/js\/modals\.js"/);
  assert.doesNotMatch(page, /src="\/dx-shader\/js\/gifenc\.js"/);
  assert.doesNotMatch(page, /src="\/dx-shader\/js\/webmmux\.js"/);
  assert.doesNotMatch(page, /src="\/dx-shader\/js\/zip\.js"/);
  assert.doesNotMatch(page, /src="\/dx-shader\/js\/fx\.js"/);
  assert.match(main, /function loadDeferredRuntimeScript\(src, globalName\)/);
  assert.match(main, /script\.async = true/);
  assert.match(main, /loadDeferredRuntimeScript\("\/dx-shader\/js\/fx\.js", "FX"\)/);
  assert.match(main, /loadDeferredRuntimeScript\("\/dx-shader\/js\/exporter\.js", "Exporter"\)/);
  assert.match(main, /loadDeferredRuntimeScript\("\/dx-shader\/js\/modals\.js", "Modals"\)/);
  assert.match(main, /function ensureExportRuntime\(\)/);
  assert.match(main, /function openExportDialog\(kind\)/);
  assert.match(main, /function openSetGeneratorDialog\(\)/);
  assert.match(main, /function exportPngShortcut\(\)/);
  assert.match(exporter, /function bindCancelButton\(\)/);
  assert.match(exporter, /document\.readyState === "loading"/);
  assert.match(exporter, /function loadRuntimeScript\(/);
  assert.match(exporter, /await loadRuntimeScript\("\/dx-shader\/js\/webmmux\.js", "WebMMux"\)/);
  assert.match(exporter, /await loadRuntimeScript\("\/dx-shader\/js\/gifenc\.js", "GIFEnc"\)/);
  assert.match(modals, /await loadRuntimeScript\("\/dx-shader\/js\/zip\.js", "ZipWriter"\)/);
});

test("cleans failed lazy script nodes before retrying export tools", () => {
  const main = source("public/dx-shader/js/main.js");
  const exporter = source("public/dx-shader/js/exporter.js");

  assert.match(main, /function cleanupDeferredRuntimeScript\(script, onLoad, onError\)/);
  assert.match(main, /script\.removeEventListener\("load", onLoad\)/);
  assert.match(main, /script\.removeEventListener\("error", onError\)/);
  assert.match(main, /script\.remove\(\)/);
  assert.match(main, /function onLoad\(\) \{/);
  assert.match(main, /function onError\(\) \{/);
  assert.match(main, /cleanupDeferredRuntimeScript\(script, onLoad, onError\)/);
  assert.match(main, /script\.addEventListener\("load", onLoad, \{ once: true \}\)/);
  assert.match(main, /script\.addEventListener\("error", onError, \{ once: true \}\)/);

  assert.match(exporter, /function cleanupRuntimeScript\(script, onLoad, onError\)/);
  assert.match(exporter, /script\.removeEventListener\("load", onLoad\)/);
  assert.match(exporter, /script\.removeEventListener\("error", onError\)/);
  assert.match(exporter, /script\.remove\(\)/);
  assert.match(exporter, /function onLoad\(\) \{/);
  assert.match(exporter, /function onError\(\) \{/);
  assert.match(exporter, /cleanupRuntimeScript\(script, onLoad, onError\)/);
  assert.match(exporter, /script\.addEventListener\("load", onLoad, \{ once: true \}\)/);
  assert.match(exporter, /script\.addEventListener\("error", onError, \{ once: true \}\)/);
});

test("keeps the initial deferred runtime under the startup budget", () => {
  const page = source("app/page.tsx");
  const layout = source("app/layout.tsx");
  const globals = source("styles/globals.css");
  const scripts = Array.from(page.matchAll(/<script defer src="([^"]+)"><\/script>/g)).map(
    ([, src]) => src
  );
  const cssImports = Array.from(layout.matchAll(/import "\.\.\/styles\/([^"]+)";/g)).map(
    ([, href]) => `styles/${href}`
  );
  const fontPreloads = Array.from(layout.matchAll(/href="(\/dx-shader\/fonts\/[^"]+\.woff2)"/g)).map(
    ([, href]) => href.replace(/^\/dx-shader\//, "public/dx-shader/")
  );
  const headIcons = Array.from(layout.matchAll(/href="(\/dx-shader\/assets\/[^"]+\.(?:svg|png))"/g)).map(
    ([, href]) => href.replace(/^\/dx-shader\//, "public/dx-shader/")
  );
  const scriptBytes = scripts.reduce((total, src) =>
    total + assetBytes(src.replace(/^\/dx-shader\//, "public/dx-shader/")), 0);
  const cssBytes = cssImports.reduce(
    (total, cssPath) => total + assetBytes(cssPath),
    0
  );
  const fontBytes = fontPreloads.reduce((total, fontPath) => total + assetBytes(fontPath), 0);
  const iconBytes = headIcons.reduce((total, iconPath) => total + assetBytes(iconPath), 0);
  const initialRuntimeBytes = scriptBytes + cssBytes + fontBytes + iconBytes;

  assert.deepEqual(cssImports, [
    "styles/theme.css",
    "styles/dx-shader.css",
    "styles/globals.css",
  ]);
  assert.doesNotMatch(globals, /@import/);
  assert.deepEqual(fontPreloads, [
    "public/dx-shader/fonts/jetbrains-mono-latin-400-normal.woff2",
  ]);
  assert.ok(fontBytes <= 22_000, `preloaded font assets are ${fontBytes} bytes`);
  assert.deepEqual(headIcons, ["public/dx-shader/assets/favicon.svg"]);
  assert.ok(iconBytes <= 1_500, `head icon assets are ${iconBytes} bytes`);
  assert.doesNotMatch(layout, /rel="apple-touch-icon"/);
  assert.ok(scriptBytes < 94_000, `initial script assets are ${scriptBytes} bytes`);
  assert.ok(initialRuntimeBytes < 150_000, `initial critical assets are ${initialRuntimeBytes} bytes`);
});

test("coalesces resize work and applies shared hash only once", () => {
  const main = source("public/dx-shader/js/main.js");
  const sharedToastCount = main.match(/Shared design loaded/g)?.length ?? 0;

  assert.equal(sharedToastCount, 1);
  assert.match(main, /function scheduleFitCanvas\(\)/);
  assert.match(main, /function installCanvasResizeHandlers\(\)/);
  assert.match(main, /if \(pendingFitCanvas\) return/);
  assert.match(main, /if \(canvasResizeHandlersInstalled\) return/);
  assert.match(main, /canvasResizeHandlersInstalled = true/);
  assert.match(main, /canvasResizeObserver = new ResizeObserver\(scheduleFitCanvas\)/);
  assert.match(main, /window\.addEventListener\("resize", scheduleFitCanvas, \{ passive: true \}\)/);
  assert.match(main, /window\.visualViewport\.addEventListener\("resize", scheduleFitCanvas/);
  assert.match(main, /if \(!frame\) return/);
  assert.match(main, /window\.getComputedStyle \? window\.getComputedStyle\(frame\) : null/);
  assert.match(main, /var paddingX = style \? cssPixel\(style\.paddingLeft\) \+ cssPixel\(style\.paddingRight\) : 0/);
  assert.doesNotMatch(main, /clientWidth - 80/);
  assert.doesNotMatch(main, /clientHeight - 52/);
  assert.doesNotMatch(main, /new ResizeObserver\(fitCanvas\)/);
});

test("keeps hostile share and saved-style payloads from blocking startup", () => {
  const main = source("public/dx-shader/js/main.js");
  const decodeStart = main.indexOf("function decodeDesign(code)");
  const atobStart = main.indexOf("atob(b64)", decodeStart);
  const parserLengthGuard = main.indexOf(
    'if (typeof code !== "string" || code.length > MAX_SHARE_CODE_LENGTH) return false',
    decodeStart
  );

  assert.match(main, /var MAX_SHARE_CODE_LENGTH = 8192/);
  assert.match(main, /var MAX_SAVED_STYLES_BYTES = 128 \* 1024/);
  assert.match(main, /var MAX_SAVED_STYLES = 24/);
  assert.match(main, /var savedStylesHydrated = false/);
  assert.match(main, /var savedStylesCache = \[\]/);
  assert.match(main, /var SHARE_CODE_PATTERN = \/\^\[A-Za-z0-9_-\]\+\$\/;/);
  assert.match(main, /var SHARE_NUM_LIMITS = \{/);
  assert.match(main, /var GENE_LIMITS = \[/);
  assert.match(main, /function normalizeSavedStyles\(styles\)/);
  assert.match(main, /function trimSavedStylesForStorage\(list\)/);
  assert.match(main, /while \(normalized\.length\) \{/);
  assert.match(main, /serialized\.length <= MAX_SAVED_STYLES_BYTES/);
  assert.match(main, /raw\.length > MAX_SAVED_STYLES_BYTES/);
  assert.match(main, /function readSavedStylesRecord\(key\)/);
  assert.match(main, /if \(raw === null\) return \{ found: false, list: \[\] \}/);
  assert.match(main, /return \{ found: true, list: normalizeSavedStyles\(parsed\) \}/);
  assert.match(main, /function hydrateSavedStyles\(\)/);
  assert.match(main, /if \(savedStylesHydrated\) return/);
  assert.match(main, /current\.found \? current\.list : readSavedStylesRecord\(LEGACY_STYLES_KEY\)\.list/);
  assert.match(main, /return savedStylesCache\.slice\(\)/);
  assert.match(main, /savedWrap\.replaceChildren\(fragment\)/);
  assert.doesNotMatch(main, /if \(styles\.length\) return styles/);
  assert.match(main, /function safeDecodeHash\(hash\)/);
  assert.match(main, /var rawHash = location\.hash \|\| ""/);
  assert.match(main, /if \(rawHash\.length <= 1 \|\| rawHash\.length > MAX_SHARE_CODE_LENGTH \+ 1\) return/);
  assert.match(main, /function clampShareNumber\(key, value\)/);
  assert.match(main, /function clampGene\(index, value\)/);
  assert.match(main, /P\[k\] = clampShareNumber\(k, arr\[9 \+ i\]\)/);
  assert.match(main, /P\.genes\.push\(clampGene\(gi, gv\)\)/);
  assert.doesNotMatch(main, /if \(isFinite\(v\)\) P\[k\] = v/);
  assert.doesNotMatch(main, /P\.genes\.push\(isFinite\(gv\) \? gv : 0\)/);
  assert.doesNotMatch(main, /history\.|pushState|replaceState|hashchange|popstate/);
  assert.match(main, /if \(!hash \|\| hash\.length > MAX_SHARE_CODE_LENGTH\) return ""/);
  assert.match(main, /try \{\n    return decodeURIComponent\(hash\);/);
  assert.match(main, /catch \(e\) \{\n    return "";\n  \}/);
  assert.match(main, /var hash = safeDecodeHash\(rawHash\.slice\(1\)\)/);
  assert.doesNotMatch(main, /var hash = decodeURIComponent\(location\.hash\.slice\(1\) \|\| ""\)/);
  assert.doesNotMatch(main, /safeDecodeHash\(location\.hash\.slice\(1\)/);
  assert.ok(decodeStart > -1, "decodeDesign should stay explicit");
  assert.ok(parserLengthGuard > decodeStart, "decodeDesign should reject oversized codes before parsing");
  assert.ok(parserLengthGuard < atobStart, "decodeDesign should guard size before atob");
  assert.match(main, /if \(!code \|\| code\.length > MAX_SHARE_CODE_LENGTH\) return false/);
  assert.match(main, /if \(!SHARE_CODE_PATTERN\.test\(b64\)\) return false/);
  assert.match(main, /pasteInput\.maxLength = MAX_SHARE_CODE_LENGTH/);
  assert.match(main, /if \(typeof scheduleFitCanvas === "function"\) scheduleFitCanvas\(\)/);
});

test("avoids redundant metadata DOM writes during frequent refreshes", () => {
  const main = source("public/dx-shader/js/main.js");
  const setTextIfChanged = main.slice(
    main.indexOf("function setTextIfChanged"),
    main.indexOf("function updateMeta")
  );
  const updateMeta = main.slice(
    main.indexOf("function updateMeta"),
    main.indexOf("function cssPixel")
  );

  assert.match(main, /var lastMetaText = {}/);
  assert.match(main, /function setTextIfChanged\(id, text\)/);
  assert.match(setTextIfChanged, /if \(lastMetaText\[id\] === text\) return/);
  assert.match(setTextIfChanged, /lastMetaText\[id\] = text/);
  assert.match(setTextIfChanged, /var element = document\.getElementById\(id\)/);
  assert.match(setTextIfChanged, /if \(element\) element\.textContent = text/);
  assert.match(updateMeta, /setTextIfChanged\("meta-mode", styleName\(\)\)/);
  assert.match(updateMeta, /setTextIfChanged\("meta-seed", "seed " \+/);
  assert.match(updateMeta, /setTextIfChanged\("meta-loop", P\.loop\.toFixed\(1\) \+ "s loop"\)/);
  assert.match(updateMeta, /setTextIfChanged\("meta-res", s\[0\] \+ "\\u00d7" \+ s\[1\]\)/);
  assert.doesNotMatch(updateMeta, /document\.getElementById\("meta-mode"\)\.textContent/);
  assert.doesNotMatch(updateMeta, /document\.getElementById\("meta-seed"\)\.textContent/);
  assert.doesNotMatch(updateMeta, /document\.getElementById\("meta-loop"\)\.textContent/);
  assert.doesNotMatch(updateMeta, /document\.getElementById\("meta-res"\)\.textContent/);
});

test("keeps high-frequency control refresh idempotent", () => {
  const ui = source("public/dx-shader/js/ui.js");
  const slider = ui.slice(ui.indexOf("function slider"), ui.indexOf("function selectRow"));
  const swatches = ui.slice(ui.indexOf("function colorSwatches"), ui.indexOf("function presetChips"));

  assert.match(slider, /var lastFill = ""/);
  assert.match(slider, /var lastValueText = ""/);
  assert.match(slider, /if \(lastFill !== fill\) \{[\s\S]*?input\.style\.setProperty\("--fill", fill\)/);
  assert.match(slider, /if \(lastValueText !== valueText\) \{[\s\S]*?val\.textContent = valueText/);
  assert.match(slider, /if \(input\.value !== nextValue\) input\.value = nextValue/);
  assert.match(swatches, /var currentColor = ""/);
  assert.match(swatches, /if \(currentColor !== nextColor\) \{[\s\S]*?sw\.style\.background = nextColor/);
  assert.match(swatches, /if \(get\(d\.key\) !== nextColor\) set\(d\.key, nextColor\)/);
  assert.match(swatches, /if \(input\.value !== v\) input\.value = v/);
  assert.match(swatches, /if \(currentColor !== v\) \{[\s\S]*?sw\.style\.background = v/);
  assert.match(ui, /function segmented\(container, options, get, set\) \{\n    if \(!container\) return function \(\) \{\}/);
  assert.match(ui, /function toast\(msg\) \{\n    var t = document\.getElementById\("toast"\);\n    if \(!t\) return/);
});

test("keeps parameter and shortcut churn out of unchanged frames", () => {
  const main = source("public/dx-shader/js/main.js");
  const setter = main.slice(main.indexOf("function set(k)"), main.indexOf("function fmt2"));
  const fpsHandler = main.slice(
    main.indexOf("Engine.onFps"),
    main.indexOf('document.getElementById("btn-random")')
  );
  const keydown = main.slice(
    main.indexOf('document.addEventListener("keydown"'),
    main.indexOf("updateMeta();", main.indexOf('document.addEventListener("keydown"'))
  );

  assert.match(main, /var META_KEYS = \{ mode: true, seed: true, loop: true \}/);
  assert.match(setter, /if \(P\[k\] === v\) return/);
  assert.match(setter, /if \(META_KEYS\[k\]\) updateMeta\(\)/);
  assert.match(setter, /requestShaderRender\(\)/);
  assert.doesNotMatch(setter, /P\[k\] = v; updateMeta\(\); requestShaderRender\(\)/);
  assert.match(main, /var FPS_META_INTERVAL_MS = 250/);
  assert.match(main, /var lastFpsMetaAt = 0/);
  assert.match(fpsHandler, /observeLiveFps\(fps\)/);
  assert.match(fpsHandler, /var now = Date\.now\(\)/);
  assert.match(fpsHandler, /if \(fps === 0 \|\| now - lastFpsMetaAt >= FPS_META_INTERVAL_MS\) \{/);
  assert.match(fpsHandler, /lastFpsMetaAt = now;\n      setTextIfChanged\("meta-fps", fps \+ " fps"\)/);
  assert.doesNotMatch(fpsHandler, /document\.getElementById\("meta-fps"\)\.textContent/);
  assert.match(keydown, /if \(e\.repeat\) return/);
});

test("keeps hidden-tab visibility listeners passive and lifecycle bounded", () => {
  const main = source("public/dx-shader/js/main.js");
  const exporter = source("public/dx-shader/js/exporter.js");
  const modals = source("public/dx-shader/js/modals.js");
  const startWhenVisible = main.slice(
    main.indexOf("function startWhenVisible"),
    main.indexOf("function scheduleShaderBoot")
  );

  assert.match(startWhenVisible, /document\.addEventListener\("visibilitychange", function onVisible\(\) \{[\s\S]*?\}, \{ once: true, passive: true \}\)/);
  assert.match(exporter, /document\.addEventListener\("visibilitychange", onVisible, \{ passive: true \}\)/);
  assert.match(exporter, /document\.removeEventListener\("visibilitychange", onVisible\)/);
  assert.match(modals, /document\.addEventListener\("visibilitychange", activePreviewVisibilityHandler, \{ passive: true \}\)/);
  assert.match(modals, /document\.removeEventListener\("visibilitychange", activePreviewVisibilityHandler\)/);
  assert.doesNotMatch(startWhenVisible, /document\.addEventListener\("visibilitychange", function onVisible\(\) \{[\s\S]*?\}, \{ once: true \}\);/);
  assert.doesNotMatch(exporter, /document\.addEventListener\("visibilitychange", onVisible\);/);
  assert.doesNotMatch(modals, /document\.addEventListener\("visibilitychange", activePreviewVisibilityHandler\);/);
});

test("keeps repeated rail styles single-owned and cheap to invalidate", () => {
  const css = source("styles/dx-shader.css");
  const publicCss = source("public/dx-shader/styles.css");

  assert.equal(css, publicCss);
  assert.equal((css.match(/\/\* ---------- share section ---------- \*\//g) ?? []).length, 1);
  assert.equal((css.match(/\.share-row \{ display: flex; gap: 8px; margin-bottom: 10px; \}/g) ?? []).length, 1);
  assert.equal((css.match(/\.share-row \.mini-btn \{ flex: 1; justify-content: center; \}/g) ?? []).length, 1);
  assert.equal((css.match(/\.share-input\s*\{/g) ?? []).length, 1);
  assert.match(css, /\.saved-styles\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.export-grid\s*\{[\s\S]*?contain:\s*layout paint/);
});

test("keeps the shader shell cheap on constrained renderers", () => {
  const page = source("app/page.tsx");
  const css = source("styles/dx-shader.css");
  const publicCss = source("public/dx-shader/styles.css");
  const main = source("public/dx-shader/js/main.js");
  const fx = source("public/dx-shader/js/fx.js");

  assert.equal(css, publicCss);
  assert.match(page, /className="canvas-shell"/);
  assert.match(css, /\.canvas-shell\b/);
  assert.match(css, /\.shader-loader\[hidden\]/);
  assert.match(css, /animation:\s*none !important/);
  assert.match(css, /transition:\s*none !important/);
  assert.doesNotMatch(css, /0\.01ms/);
  assert.doesNotMatch(css, /backdrop-filter:\s*blur/);
  assert.doesNotMatch(css, /blur\((?:[1-9]|1[0-9]|2[0-9])px\)/);
  assert.doesNotMatch(css, /\.rail\s*\{[^}]*backdrop-filter:\s*blur/);
  assert.doesNotMatch(css, /\.modal-card\s*\{[^}]*backdrop-filter:\s*blur/);
  assert.match(css, /\.modal-preview-meta\s*\{[\s\S]*?backdrop-filter:\s*none/);
  assert.match(css, /\.stage\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.stage\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.shader-loader\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.shader-loader\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.shader-loader-panel\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.shader-loader-shimmer\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /\.canvas-shell\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /#view\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /\.rail\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.rail-section\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /content-visibility:\s*auto/);
  assert.match(css, /\.modal-backdrop\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.modal-backdrop\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.modal-card\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.modal-card\s*\{[\s\S]*?overscroll-behavior:\s*contain/);
  assert.match(css, /\.modal-preview\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.set-grid\s*\{[\s\S]*?contain:\s*layout/);
  assert.match(css, /\.set-grid\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.set-tile\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.set-tile canvas\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /\.overlay\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.overlay\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.overlay-card\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.progress\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /@media \(update: slow\), \(hover: none\), \(pointer: coarse\)/);
  assert.match(css, /@media \(update: slow\), \(hover: none\), \(pointer: coarse\), \(prefers-reduced-data: reduce\)/);
  assert.match(css, /--loader-shimmer:\s*transparent/);
  assert.match(css, /--loader-bg:\s*var\(--bg-stage\)/);
  assert.doesNotMatch(css, /transition:[^;]*box-shadow/);
  assert.doesNotMatch(css, /animation:[^;]*infinite/);
  assert.match(css, /\.shader-loader\[data-loader-state="loading"\] \.shader-loader-spinner\s*\{\n  animation: shader-loader-spin 1100ms linear 6;/);
  assert.match(css, /\.shader-loader\[data-loader-state="loading"\] \.shader-loader-shimmer\s*\{\n  animation: shader-loader-shimmer 1550ms ease-in-out 4;/);
  assert.match(css, /\.shader-loader-spinner,\n  \.shader-loader-shimmer,[\s\S]*?\.fx-layer \{\n    animation: none;/);
  assert.match(css, /\.shader-loader\[data-loader-state="loading"\] \.shader-loader-spinner,\n  \.shader-loader\[data-loader-state="loading"\] \.shader-loader-shimmer,[\s\S]*?\.toast\.success svg path \{\n    animation: none;/);
  assert.match(css, /\.modal-backdrop,\n  \.modal-card,\n  \.toast,/);
  assert.match(css, /\.mode-card:hover,\n  \.mode-card:hover svg,[\s\S]*?\.set-tile:hover,[\s\S]*?input\[type="range"\]:active::-webkit-slider-thumb \{\n    transform: none;/);
  assert.match(css, /\.btn:hover,\n  \.btn:hover svg,[\s\S]*?input\[type="range"\]:active::-webkit-slider-thumb \{\n    transform: none;/);
  assert.match(css, /\.shader-loader-shimmer,\n  \.fx-ring,\n  \.fx-layer \{\n    display: none;/);
  assert.match(css, /bottom: max\(22px, calc\(env\(safe-area-inset-bottom\) \+ 14px\)\)/);
  assert.match(css, /max-height: min\(88dvh, calc\(100dvh - 48px - env\(safe-area-inset-bottom\)\)\)/);
  assert.match(css, /\.canvas-shell \{\n    border-radius: var\(--radius-m\);\n    box-shadow: 0 0 0 1px var\(--line\);/);
  assert.match(css, /\.progress-fill\s*\{[\s\S]*?transform: scaleX\(0\)/);
  assert.match(css, /transition:\s*transform 160ms ease/);
  assert.doesNotMatch(css, /transition:\s*width/);
  assert.match(css, /height: calc\(100dvh - var\(--topbar-h\)\)/);
  assert.match(css, /\.stage \{ height: 58vh; height: 58dvh; \}/);
  assert.match(css, /\.shader-static-title\s*\{[\s\S]*?font-weight:\s*600/);
  assert.doesNotMatch(css, /will-change:/);
  assert.match(main, /function shouldSkipIntroAnimation\(\)/);
  assert.match(main, /return shouldUseManualShaderStart\(\)/);
  assert.match(main, /\(function intro\(\) \{\n    if \(shouldSkipIntroAnimation\(\)\) return;\n    holdAdaptiveBudget\(3200\);\n    var items = \[\];/);
  assert.match(fx, /function shouldSkipCelebrationBurst\(\)/);
  assert.match(fx, /prefers-reduced-motion: reduce/);
  assert.doesNotMatch(fx, /Edg\\\/|isEdgeBrowser/);
  assert.match(fx, /prefers-reduced-data: reduce/);
  assert.match(fx, /update: slow/);
  assert.match(fx, /pointer: coarse/);
  assert.match(fx, /var MAX_BURST_PIXELS = 1400000/);
  assert.match(fx, /var activeBurstCancel = null/);
  assert.match(fx, /typeof nav\.deviceMemory === "number" && nav\.deviceMemory <= 4/);
  assert.match(fx, /typeof nav\.hardwareConcurrency === "number" && nav\.hardwareConcurrency <= 4/);
  assert.match(fx, /\/\^\(slow-2g\|2g\)\$\/\.test\(effectiveType\)/);
  assert.match(fx, /if \(!shouldSkipCelebrationBurst\(\)\) burst\(\)/);
  assert.match(fx, /if \(activeBurstCancel\) activeBurstCancel\(\)/);
  assert.match(fx, /Math\.sqrt\(MAX_BURST_PIXELS \/ Math\.max\(1, innerWidth \* innerHeight\)\)/);
  assert.match(fx, /decay: 0\.012 \+ Math\.random\(\) \* 0\.006/);
  assert.match(fx, /p\.life -= p\.decay/);
  assert.match(fx, /function createSuccessIcon\(\)/);
  assert.match(fx, /label\.textContent = message/);
  assert.doesNotMatch(fx, /innerHTML\s*=/);
  assert.match(css, /\.fx-layer\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /\.fx-ring\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.toast\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.canvas-frame\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.canvas-frame\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.shader-static-fallback\s*\{[\s\S]*?contain:\s*layout paint/);
  assert.match(css, /\.shader-static-fallback\s*\{[\s\S]*?isolation:\s*isolate/);
  assert.match(css, /\.modal-preview canvas\s*\{[\s\S]*?contain:\s*paint/);
  assert.match(css, /\.modal-preview-meta\s*\{[\s\S]*?contain:\s*paint/);
});

test("keeps startup assets self-hosted and parser-friendly", () => {
  const layout = source("app/layout.tsx");
  const page = source("app/page.tsx");
  const globals = source("styles/globals.css");
  const docs = source("public/dx-shader/docs.html");
  const docsCss = source("public/dx-shader/docs.css");
  const vercel = source("vercel.json");
  const scriptTags = Array.from(page.matchAll(/<script[^>]+src="[^"]+"[^>]*>/g));

  assert.doesNotMatch(layout, /fonts\.googleapis\.com|fonts\.gstatic\.com/);
  assert.doesNotMatch(globals, /generated\.css/);
  assert.doesNotMatch(globals, /@import/);
  assert.match(layout, /import "\.\.\/styles\/theme\.css"/);
  assert.match(layout, /import "\.\.\/styles\/dx-shader\.css"/);
  assert.match(layout, /import "\.\.\/styles\/globals\.css"/);
  assert.match(docs, /href="docs\.css"/);
  assert.doesNotMatch(docs, /href="styles\.css"/);
  assert.doesNotMatch(docs, /<style[\s>]/);
  assert.doesNotMatch(docs, /\sstyle="/);
  assert.match(docsCss, /\.docs-link\s*\{/);
  assert.match(vercel, /script-src 'self'/);
  assert.doesNotMatch(vercel, /script-src 'none'/);
  assert.ok(scriptTags.length > 0, "shader route should still load ordered runtime scripts");
  for (const [tag] of scriptTags) {
    assert.match(tag, /\sdefer(?:[=>\s]|$)/);
  }
});

test("keeps unused public preview payloads out of the deployment", () => {
  const readme = source("README.md");
  const removedPayloads = [
    "public/dx-shader/assets/icon-512.png",
    "public/dx-shader/assets/og.png",
    "public/dx-shader/docs/preview-chrome.png",
    "public/dx-shader/docs/preview-glyphs.png",
    "public/dx-shader/docs/preview-silk.png",
  ];
  const publicFiles = collectFiles(new URL("../public/dx-shader/", import.meta.url));
  const totalPublicBytes = publicFiles.reduce((total, file) => total + statSync(file).size, 0);
  const oversizedFiles = publicFiles.filter((file) => statSync(file).size > 500_000);

  for (const payload of removedPayloads) {
    assert.equal(existsSync(new URL(`../${payload}`, import.meta.url)), false, `${payload} should not ship`);
  }
  assert.deepEqual(oversizedFiles, []);
  assert.ok(totalPublicBytes < 450_000, `dx-shader public payload is ${totalPublicBytes} bytes`);
  assert.doesNotMatch(readme, /Open Graph|preview assets/);
});

test("uses reason-based render suspension and a live pixel budget", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const main = source("public/dx-shader/js/main.js");
  const exporter = source("public/dx-shader/js/exporter.js");
  const modals = source("public/dx-shader/js/modals.js");

  assert.match(engine, /var suspensionReasons = {}/);
  assert.match(engine, /function suspendFor\(reason\)/);
  assert.match(engine, /function resumeFor\(reason\)/);
  assert.match(engine, /Engine\.suspendFor\("visibility"\)/);
  assert.match(engine, /Engine\.resumeFor\("visibility"\)/);
  assert.match(exporter, /Engine\.suspendFor\("export"\)/);
  assert.match(exporter, /Engine\.resumeFor\("export"\)/);
  assert.match(modals, /Engine\.suspendFor\("modal"\)/);
  assert.match(modals, /Engine\.resumeFor\("modal"\)/);
  assert.match(modals, /var activePreviewSuspended = false/);
  assert.match(modals, /Engine\.suspendFor\("modal-preview"\)/);
  assert.match(modals, /Engine\.resumeFor\("modal-preview"\)/);
  assert.match(main, /var MAX_LIVE_PIXELS = 1400000/);
  assert.match(main, /var WEAK_LIVE_PIXELS = 850000/);
  assert.match(main, /var MIN_LIVE_PIXELS = 420000/);
  assert.match(main, /var STRONG_LIVE_FPS = 60/);
  assert.match(main, /var WEAK_LIVE_FPS = 30/);
  assert.match(main, /function livePixelBudget\(\)/);
  assert.match(main, /function liveTargetFps\(\)/);
  assert.match(main, /return weakDeviceProfile\(\) \? WEAK_LIVE_FPS : STRONG_LIVE_FPS/);
  assert.match(main, /Math\.sqrt\(livePixelBudget\(\) \/ \(cssW \* cssH\)\)/);
  assert.match(main, /Engine\.setTargetFps\(liveTargetFps\(\)\)/);
  assert.doesNotMatch(main, /Engine\.setTargetFps\(weakDeviceProfile\(\) \? 30 : 0\)/);
});

test("guards expensive exports and modal generation with budgets and cooperative yields", () => {
  const exporter = source("public/dx-shader/js/exporter.js");
  const modals = source("public/dx-shader/js/modals.js");
  const gifenc = source("public/dx-shader/js/gifenc.js");
  const zip = source("public/dx-shader/js/zip.js");
  const muxer = source("public/dx-shader/js/webmmux.js");
  const pickEncoderIndex = exporter.indexOf("var picked = await pickEncoderConfig(w, h, fps)");
  const webmMuxLoadIndex = exporter.indexOf('await loadRuntimeScript("/dx-shader/js/webmmux.js", "WebMMux")');

  assert.match(exporter, /var MAX_EXPORT_MEMORY_BYTES =/);
  assert.match(exporter, /var MAX_EXPORT_MEMORY_BYTES = 384 \* 1024 \* 1024/);
  assert.match(exporter, /function deviceMemoryBudgetBytes\(\)/);
  assert.match(exporter, /function canvasToBlob\(canvas, type\)/);
  assert.match(exporter, /if \(!blob\) \{[\s\S]*?reject\(new Error\("Canvas export produced no data"\)\)/);
  assert.match(exporter, /function estimateImageExportCost\(w, h\)/);
  assert.match(exporter, /function estimateVideoExportCost\(w, h, nFrames\)/);
  assert.match(exporter, /function encodedVideoBudgetBytes\(w, h, nFrames\)/);
  assert.match(exporter, /function estimateGifExportCost\(w, h, nFrames\)/);
  assert.match(exporter, /function canRunExport\(cost\)/);
  assert.match(exporter, /function waitForVisibleOrCancel\(\)/);
  assert.match(exporter, /async function waitForEncoderQueue\(encoder, hasEncoderError\)/);
  assert.match(exporter, /while \(encoder\.encodeQueueSize > 2 && !cancelled && !hasEncoderError\(\)\)/);
  assert.match(exporter, /if \(!await waitForVisibleOrCancel\(\)\) return false/);
  assert.match(exporter, /function yieldToBrowser\(\)/);
  assert.match(exporter, /function downloadBlob\(blob, name\)/);
  assert.match(exporter, /var url = URL\.createObjectURL\(blob\)/);
  assert.match(exporter, /finally \{[\s\S]*?setTimeout\(function \(\) \{ URL\.revokeObjectURL\(url\); \}, 4000\)/);
  assert.match(exporter, /downloadBlob: downloadBlob/);
  assert.doesNotMatch(exporter, /URL\.revokeObjectURL\(a\.href\)/);
  assert.match(exporter, /\$\("overlay-bar"\)\.style\.transform = "scaleX\(0\)"/);
  assert.match(exporter, /\$\("overlay-bar"\)\.style\.transform = "scaleX\("/);
  assert.doesNotMatch(exporter, /overlay-bar"\)\.style\.width/);
  assert.match(exporter, /var pngCost = estimateImageExportCost\(w, h\)/);
  assert.match(exporter, /if \(!canRunExport\(pngCost\)\)/);
  assert.match(exporter, /var blob = await canvasToBlob\(Engine\.canvas\(\), "image\/png"\)/);
  assert.match(exporter, /downloadBlob\(blob, stamp\(P, "png"\)\)/);
  assert.match(exporter, /catch \(e\) \{\n      UI\.toast\("PNG export failed"/);
  assert.match(exporter, /finally \{\n      Engine\.setSize\(prev\[0\], prev\[1\]\);/);
  assert.match(exporter, /try \{\n          encoder\.encode\(vf,/);
  assert.match(exporter, /finally \{\n          vf\.close\(\);\n        \}/);
  assert.match(exporter, /var videoCost = estimateVideoExportCost\(w, h, nFrames\)/);
  assert.match(exporter, /if \(!canRunExport\(videoCost\)\)/);
  assert.ok(pickEncoderIndex > -1, "video export should choose codec support explicitly");
  assert.ok(webmMuxLoadIndex > pickEncoderIndex, "WebM muxer should lazy-load only after codec support is confirmed");
  assert.match(exporter, /var gifCost = estimateGifExportCost\(w, h, nFrames\)/);
  assert.match(exporter, /if \(!canRunExport\(gifCost\)\)/);
  assert.match(exporter, /await yieldToBrowser\(\)/);
  assert.match(exporter, /if \(!await waitForVisibleOrCancel\(\)\) break/);
  assert.match(exporter, /var encodedBytes = 0/);
  assert.match(exporter, /var maxEncodedBytes = encodedVideoBudgetBytes\(w, h, nFrames\)/);
  assert.match(exporter, /var activeCancel = null/);
  assert.match(exporter, /var activeVisibilityCancel = null/);
  assert.match(exporter, /function requestCancel\(\) \{\n    cancelled = true;\n    if \(activeVisibilityCancel\) activeVisibilityCancel\(\);\n    if \(activeCancel\) activeCancel\(\);\n  \}/);
  assert.match(exporter, /activeVisibilityCancel = function \(\) \{[\s\S]*?document\.removeEventListener\("visibilitychange", onVisible\);[\s\S]*?activeVisibilityCancel = null;[\s\S]*?resolve\(false\);[\s\S]*?\};/);
  assert.match(exporter, /output: function \(chunk\) \{\n          if \(cancelled \|\| encError\) return;/);
  assert.match(exporter, /if \(encodedBytes \+ chunk\.byteLength > maxEncodedBytes\) \{/);
  assert.match(exporter, /encodedBytes \+= data\.byteLength/);
  assert.doesNotMatch(exporter, /var data = new Uint8Array\(chunk\.byteLength\);\n          chunk\.copyTo\(data\);\n          encodedBytes \+= data\.byteLength;\n          if \(encodedBytes > maxEncodedBytes\) \{/);
  assert.match(exporter, /new Error\("Encoded video exceeded the safe memory budget"\)/);
  assert.match(exporter, /try \{\n      encoder = new VideoEncoder/);
  assert.match(exporter, /encoder\.configure\(picked\.config\);/);
  assert.match(exporter, /activeCancel = function \(\) \{\n        try \{ encoder\.reset\(\); \} catch \(ignoreReset\) \{\}\n      \};/);
  assert.match(exporter, /catch \(e\) \{\n      encFrames\.length = 0;\n      restore\(\);/);
  assert.match(exporter, /UI\.toast\("Video encoder unavailable"/);
  assert.match(exporter, /if \(!await waitForEncoderQueue\(encoder, function \(\) \{ return encError; \}\)\) break/);
  assert.doesNotMatch(exporter, /await wait\(2\)/);
  assert.match(exporter, /if \(cancelled \|\| encError\) \{\n      try \{ encoder\.reset\(\); \} catch \(ignoreReset\) \{\}\n    \}\n    try \{ encoder\.close\(\); \} catch \(ignore\) \{\}/);
  assert.match(exporter, /try \{ encoder\.close\(\); \} catch \(ignore\) \{\}\n    activeCancel = null;/);
  assert.match(exporter, /button\.addEventListener\("click", requestCancel\)/);
  assert.doesNotMatch(exporter, /button\.addEventListener\("click", function \(\) \{ cancelled = true; \}\)/);
  assert.match(exporter, /var muxCost = exportCost\("Video mux", encodedBytes \* 5 \+ w \* h \* BYTES_PER_PIXEL/);
  assert.match(exporter, /if \(!canRunExport\(muxCost\)\)/);
  assert.match(exporter, /WebMMux\.muxAsync/);
  assert.match(exporter, /if \(!WebMMux\.muxAsync\) \{/);
  assert.doesNotMatch(exporter, /WebMMux\.mux\(muxOptions\)/);
  assert.match(exporter, /catch \(e\) \{\n      encFrames\.length = 0;\n      hideOverlay\(\);\n      busy = false;\n      UI\.toast\("Video mux failed"/);
  assert.match(exporter, /encFrames\.length = 0/);
  assert.match(exporter, /frames\.length = 0/);
  assert.match(exporter, /finally \{\n      frames\.length = 0;\n      Engine\.setSize\(prev\[0\], prev\[1\]\);/);
  assert.match(exporter, /yieldToBrowser: yieldToBrowser/);
  assert.match(exporter, /canvasToBlob: canvasToBlob/);

  assert.match(modals, /var activePreviewFrame = 0/);
  assert.match(modals, /var activePreviewVisibilityHandler = null/);
  assert.match(modals, /var setBuildToken = 0/);
  assert.match(modals, /function clearPreviewTimer\(\)/);
  assert.match(modals, /function clearSetEntries\(entries\)/);
  assert.match(modals, /function cancelSetExport\(entries\)/);
  assert.match(modals, /function snapshotInto\(canvas2d, aspect\)/);
  assert.match(modals, /if \(!src \|\| !ctx \|\| !canvas2d\.width \|\| !canvas2d\.height\) return false/);
  assert.match(modals, /ctx\.drawImage\(src, 0, 0, canvas2d\.width, canvas2d\.height\)/);
  assert.match(modals, /function startPreviewLoop\(canvas2d, aspect\)/);
  assert.doesNotMatch(modals, /setTimeout\(schedule, 250\)/);
  assert.match(modals, /document\.addEventListener\("visibilitychange", activePreviewVisibilityHandler, \{ passive: true \}\)/);
  assert.doesNotMatch(modals, /setInterval\(function \(\) \{/);
  assert.match(modals, /var previewBuildQueue = Promise\.resolve\(\)/);
  assert.match(modals, /var queuedBuildToken = 0/);
  assert.match(modals, /function queueBuild\(\)/);
  assert.match(modals, /queuedBuildToken = \+\+setBuildToken/);
  assert.match(modals, /await previewBuildQueue\.catch\(function \(\) \{}\)/);
  assert.match(modals, /setBuildToken\+\+/);
  assert.match(modals, /async function build\(token\)/);
  assert.doesNotMatch(modals, /grid\.innerHTML/);
  assert.match(modals, /grid\.replaceChildren\(\)/);
  assert.match(modals, /var fragment = document\.createDocumentFragment\(\)/);
  assert.match(modals, /var tile = el\("button", "set-tile", fragment\)/);
  assert.match(modals, /grid\.replaceChildren\(fragment\)/);
  assert.match(modals, /function estimateSetExportCost\(w, h, count\)/);
  assert.match(modals, /var setCost = estimateSetExportCost\(w, h, seeds\.length\)/);
  assert.match(modals, /if \(!Exporter\.canRunExport\(setCost\)\)/);
  assert.match(modals, /await Exporter\.yieldToBrowser\(\)/);
  assert.match(modals, /var captureError = null/);
  assert.match(modals, /var blob = await Exporter\.canvasToBlob\(Engine\.canvas\(\), "image\/png"\);\n          if \(token !== setBuildToken \|\| !document\.body\.contains\(dl\)\) \{ cancelSetExport\(entries\); return; \}/);
  assert.match(modals, /var data = await blob\.arrayBuffer\(\);[\s\S]*?if \(token !== setBuildToken \|\| !document\.body\.contains\(dl\)\) \{ cancelSetExport\(entries\); return; \}/);
  assert.match(modals, /catch \(e\) \{\n        captureError = e;\n      \} finally/);
  assert.match(modals, /if \(captureError\) \{\n        clearSetEntries\(entries\);\n        resetDownloadButton\(\);\n        UI\.toast\("Set export failed"/);
  assert.match(modals, /var packedCount = entries\.length/);
  assert.match(modals, /ZipWriter\.buildAsync\(entries, function \(frac\)/);
  assert.match(modals, /clearSetEntries\(entries\);\n        resetDownloadButton\(\);\n        UI\.toast\("ZIP export failed"/);
  assert.match(exporter, /downloadBlob\(webm, stamp\(P, "webm"\)\)/);
  assert.match(exporter, /downloadBlob\(new Blob\(\[data\], \{ type: "image\/gif" \}\), stamp\(P, "gif"\)\)/);
  assert.match(modals, /clearSetEntries\(entries\);\n      Exporter\.downloadBlob\(zip, "dx-shader-set-" \+ MODES\[P\.mode\]\.key \+ "\.zip"\)/);
  assert.doesNotMatch(modals, /URL\.createObjectURL\(zip\)/);
  assert.doesNotMatch(modals, /URL\.revokeObjectURL\(a\.href\)/);
  assert.match(modals, /function \(\) \{ return token !== setBuildToken \|\| !document\.body\.contains\(dl\); \}/);

  assert.match(zip, /function buildAsync\(entries, onProgress, isCancelled\)/);
  assert.match(zip, /var TEXT_ENCODER = new TextEncoder\(\)/);
  assert.match(zip, /async function crc32Async\(buf, isCancelled\)/);
  assert.match(zip, /function clearZipState\(state\)/);
  assert.match(zip, /await waitForBrowser\(\)/);
  assert.match(zip, /var crc = await crc32Async\(entries\[i\]\.data, isCancelled\)/);
  assert.match(zip, /appendEntry\(state, entries\[i\], crc\);\n      entries\[i\]\.data = null;/);
  assert.match(zip, /var nameBytes = TEXT_ENCODER\.encode\(e\.name\)/);
  assert.doesNotMatch(zip, /new TextEncoder\(\)\.encode/);
  assert.match(zip, /clearZipState\(state\);\n        return null/);
  assert.match(zip, /await waitForBrowser\(\)/);
  assert.match(zip, /try \{\n      return finalizeZip\(state, entries\.length\);\n    \} finally \{\n      clearZipState\(state\);\n    \}/);
  assert.match(zip, /return \{ buildAsync: buildAsync \}/);
  assert.doesNotMatch(zip, /build: build/);
  assert.doesNotMatch(modals, /ZipWriter\.build\(/);

  assert.match(muxer, /function clearMuxState\(state\)/);
  assert.match(muxer, /function partsLength\(parts\)/);
  assert.match(muxer, /function elementHeader\(id, payloadLength\)/);
  assert.match(muxer, /function elementFromParts\(id, parts\)/);
  assert.match(muxer, /function simpleBlockElement\(f, rel\)/);
  assert.match(muxer, /function finalizeMuxBlob\(opts, state\)/);
  assert.match(muxer, /state\.blocks\.push\(simpleBlockElement\(f, rel\)\)/);
  assert.doesNotMatch(muxer, /state\.blocks\.push\(el\(0xA3, concat\(\[head, f\.data\]\)\)/);
  assert.doesNotMatch(muxer, /var webm = finalizeMux\(opts, state\)/);
  assert.doesNotMatch(muxer, /var segment = elementFromParts\(0x18538067/);
  assert.match(muxer, /var segmentParts = \[\n      infoElement\(opts\.durationMs\),\n      tracksElement\(opts\.codecId, opts\.width, opts\.height\)\n    \]\.concat\(state\.clusters\)/);
  assert.match(muxer, /var segmentHeader = elementHeader\(0x18538067, partsLength\(segmentParts\)\)/);
  assert.match(muxer, /return new Blob\(\[headerElement\(\), segmentHeader\]\.concat\(segmentParts\), \{ type: "video\/webm" \}\)/);
  assert.match(muxer, /frames\[i\]\.data = null/);
  assert.match(muxer, /frames\[i\] = null/);
  assert.match(muxer, /clearMuxState\(state\);\n        return null/);
  assert.match(muxer, /try \{\n      return finalizeMuxBlob\(opts, state\);\n    \} finally \{\n      clearMuxState\(state\);\n    \}/);
  assert.doesNotMatch(muxer, /var webm = finalizeMuxBlob\(opts, state\);\n    clearMuxState\(state\);\n    return webm/);

  assert.match(gifenc, /var sampleCount = 0/);
  assert.match(gifenc, /function sampleChannel\(sample, channel\)/);
  assert.match(gifenc, /async function buildPalette\(samples, sampleCount, maxColors, isCancelled\)/);
  assert.match(gifenc, /var boxes = \[samples\.subarray\(0, sampleCount\)\]/);
  assert.match(gifenc, /var samples = new Uint32Array\(Math\.min\(targetSamples, Math\.max\(1, totalPx\)\)\)/);
  assert.doesNotMatch(gifenc, /samples\.push\(\[/);
  assert.doesNotMatch(gifenc, /box\[j\]\[c\]/);
  assert.match(gifenc, /samples\[sampleCount\+\+\] = \(d\[p \* 4\] << 16\) \| \(d\[p \* 4 \+ 1\] << 8\) \| d\[p \* 4 \+ 2\]/);
  assert.match(gifenc, /var palette = await buildPalette\(samples, sampleCount, 256, isCancelled\)/);
  assert.match(gifenc, /var palette = await buildPalette\(samples, sampleCount, 256, isCancelled\);\n\s+samples = null;\n\s+if \(!palette\) return null/);
  assert.match(gifenc, /if \(!palette\) return null/);
  assert.match(gifenc, /finally \{\n\s+frames\[fi\] = null;\n\s+data = null;\n\s+\}/);
  assert.match(gifenc, /function clearFrameBuffers\(frames, start\)/);
  assert.match(gifenc, /for \(var i = start \|\| 0; i < frames\.length; i\+\+\) frames\[i\] = null/);
  assert.match(gifenc, /var completed = false/);
  assert.match(gifenc, /finally \{\n      if \(!completed\) clearFrameBuffers\(frames, 0\);\n    \}/);
  assert.match(gifenc, /async function lzwEncode\(minCodeSize, indices, out, isCancelled\)/);
  assert.match(gifenc, /if \(\(i & 8191\) === 0\) \{/);
  assert.match(gifenc, /if \(!await lzwEncode\(8, indices, out, isCancelled\)\) return null/);
  assert.match(gifenc, /if \(\(sampleCount & 4095\) === 0\) \{/);
  assert.match(gifenc, /var rowsPerYield = Math\.max\(1, Math\.min\(32, Math\.floor\(16384 \/ w\)\)\)/);
  assert.match(gifenc, /if \(\(\(y \+ 1\) % rowsPerYield\) === 0\) \{/);
  assert.doesNotMatch(gifenc, /if \(\(y & 31\) === 31\) \{/);
  assert.match(gifenc, /if \(isCancelled\(\)\) return null/);
});

test("keeps WebM muxing cancellable without a synchronous UI-freeze fallback", () => {
  const muxer = source("public/dx-shader/js/webmmux.js");

  assert.match(muxer, /async function muxAsync\(opts, isCancelled, yieldToBrowser\)/);
  assert.match(muxer, /function partsLength\(parts\)/);
  assert.match(muxer, /function elementHeader\(id, payloadLength\)/);
  assert.match(muxer, /function elementFromParts\(id, parts\)/);
  assert.match(muxer, /function simpleBlockElement\(f, rel\)/);
  assert.match(muxer, /function finalizeMuxBlob\(opts, state\)/);
  assert.match(muxer, /var shouldCancel = typeof isCancelled === "function"/);
  assert.match(muxer, /var yieldNow = typeof yieldToBrowser === "function"/);
  assert.match(muxer, /if \(shouldCancel\(\)\) \{\n        clearMuxState\(state\);\n        return null;\n      \}/);
  assert.match(muxer, /await yieldNow\(\)/);
  assert.match(muxer, /else if \(\(i & 15\) === 15\) \{/);
  assert.doesNotMatch(muxer, /state\.blocks\.push\(el\(0xA3, concat\(\[head, f\.data\]\)\)/);
  assert.doesNotMatch(muxer, /var webm = finalizeMux\(opts, state\)/);
  assert.doesNotMatch(muxer, /var segment = elementFromParts\(0x18538067/);
  assert.match(muxer, /return new Blob\(\[headerElement\(\), segmentHeader\]\.concat\(segmentParts\), \{ type: "video\/webm" \}\)/);
  assert.match(muxer, /try \{\n      return finalizeMuxBlob\(opts, state\);\n    \} finally \{\n      clearMuxState\(state\);\n    \}/);
  assert.doesNotMatch(muxer, /function mux\(opts\)/);
  assert.match(muxer, /return \{ muxAsync: muxAsync \}/);
});

test("handles WebGL context loss and weak-device live quality before heavy rendering", () => {
  const engine = source("public/dx-shader/js/engine.js");
  const main = source("public/dx-shader/js/main.js");

  assert.match(engine, /var contextLost = false/);
  assert.match(engine, /function isContextLost\(\)/);
  assert.match(engine, /function observeContextLoss\(\)/);
  assert.match(engine, /function handleContextLost\(event\)/);
  assert.match(engine, /event\.preventDefault\(\)/);
  assert.match(engine, /suspendFor\("context"\)/);
  assert.match(engine, /function handleContextRestored\(\)/);
  assert.match(engine, /gl\.viewport\(0, 0, canvas\.width, canvas\.height\)/);
  assert.match(engine, /function reportPausedFps\(\)/);
  assert.match(engine, /if \(fpsCb\) fpsCb\(0\)/);
  assert.match(engine, /canvas\.addEventListener\("webglcontextlost", handleContextLost, false\)/);
  assert.match(engine, /canvas\.addEventListener\("webglcontextrestored", handleContextRestored, false\)/);
  assert.match(engine, /if \(!initialized \|\| observeContextLoss\(\)\) return/);
  assert.match(engine, /throw new Error\("WebGL context is unavailable"\)/);
  assert.match(engine, /onContextChange: function \(cb\)/);

  assert.match(main, /function weakDeviceProfile\(\)/);
  assert.match(main, /var cachedWeakRenderProfile = null/);
  assert.match(main, /var cachedWeakRenderParams = null/);
  assert.match(main, /function requestShaderRender\(\) \{\n  cachedWeakRenderParams = null;/);
  assert.match(main, /if \(cachedWeakRenderProfile === null\) cachedWeakRenderProfile = shouldUseReducedRenderProfile\(\)/);
  assert.match(main, /function applyWeakRenderProfile\(params\)/);
  assert.match(main, /function copyRenderParams\(params\)/);
  assert.match(main, /if \(cachedWeakRenderParams\) return cachedWeakRenderParams/);
  assert.match(main, /cachedWeakRenderParams = next;\n  return next/);
  assert.match(main, /next\.complex = Math\.min\(next\.complex, 4\.5\)/);
  assert.match(main, /next\.ca = 0/);
  assert.match(main, /next\.glow = Math\.min\(next\.glow, 0\.35\)/);
  assert.match(main, /Engine\.init\(canvas, function \(\) \{ return liveRenderParams\(\); \}\)/);
  assert.match(main, /var errorFrame = document\.querySelector\("\.canvas-frame"\);\n    if \(errorFrame\) errorFrame\.innerHTML =/);
  assert.match(main, /Engine\.onContextChange\(function \(state, error\)/);
  assert.match(main, /state === "restored"[\s\S]*?lastCanvasLayout = null;[\s\S]*?scheduleFitCanvas\(\)/);
  assert.match(main, /installCanvasResizeHandlers\(\)/);
  assert.match(main, /Math\.max\(2, 2 \* Math\.round\(w \* dpr \/ 2\)\)/);
  assert.match(main, /function fitCanvas\(\) \{\n  var frame = document\.getElementById\("canvas-frame"\);\n  if \(!frame\) return/);
  assert.match(main, /var canvasResizeHandlersInstalled = false/);
  assert.match(main, /var canvasResizeObserver = null/);
  assert.match(main, /if \(canvasResizeHandlersInstalled\) return/);
  assert.match(main, /canvasResizeObserver = new ResizeObserver\(scheduleFitCanvas\)/);
});

test("adapts live pixel budget after sustained low FPS", () => {
  const main = source("public/dx-shader/js/main.js");

  assert.match(main, /var MIN_LIVE_PIXELS =/);
  assert.match(main, /var ADAPTIVE_BUDGET_COOLDOWN_MS = 2500/);
  assert.match(main, /var ADAPTIVE_RESUME_HOLD_MS = 900/);
  assert.match(main, /var adaptiveLivePixels = null/);
  assert.match(main, /var lastAdaptiveBudgetChangeAt = 0/);
  assert.match(main, /var lowFpsSamples = 0/);
  assert.match(main, /var stableFpsSamples = 0/);
  assert.match(main, /var adaptiveBudgetHoldUntil = 0/);
  assert.match(main, /function holdAdaptiveBudget\(ms\)/);
  assert.match(main, /adaptiveBudgetHoldUntil = Math\.max\(adaptiveBudgetHoldUntil, Date\.now\(\) \+ ms\)/);
  assert.match(main, /function resetAdaptiveFpsSamples\(\)/);
  assert.match(main, /function baseLivePixelBudget\(\)/);
  assert.match(main, /function livePixelBudget\(\)/);
  assert.match(main, /return adaptiveLivePixels \|\| baseLivePixelBudget\(\)/);
  assert.match(main, /function resetAdaptiveLiveBudget\(\)/);
  assert.match(main, /function canChangeAdaptiveBudget\(now\)/);
  assert.match(main, /return lastAdaptiveBudgetChangeAt === 0 \|\| now - lastAdaptiveBudgetChangeAt >= ADAPTIVE_BUDGET_COOLDOWN_MS/);
  assert.match(main, /function stableFpsThreshold\(\)/);
  assert.match(main, /return Math\.round\(liveTargetFps\(\) \* 0\.9\)/);
  assert.match(main, /function lowFpsThreshold\(\)/);
  assert.match(main, /return Math\.round\(liveTargetFps\(\) \* 0\.8\)/);
  assert.match(main, /function observeLiveFps\(fps\)/);
  assert.match(main, /if \(fps <= 0\) \{\n    resetAdaptiveFpsSamples\(\);\n    holdAdaptiveBudget\(ADAPTIVE_RESUME_HOLD_MS\);\n    return;\n  \}/);
  assert.match(main, /var now = Date\.now\(\)/);
  assert.match(main, /if \(now < adaptiveBudgetHoldUntil\) \{[\s\S]*?lowFpsSamples = 0;[\s\S]*?stableFpsSamples = 0;[\s\S]*?return;[\s\S]*?\}/);
  assert.match(main, /if \(fps > 0 && fps < lowFpsThreshold\(\)\)/);
  assert.match(main, /else if \(fps >= stableFpsThreshold\(\)\)/);
  assert.match(main, /if \(lowFpsSamples >= 3 && livePixelBudget\(\) > MIN_LIVE_PIXELS\)/);
  assert.match(main, /if \(!canChangeAdaptiveBudget\(now\)\) return/);
  assert.match(main, /adaptiveLivePixels = Math\.max\(MIN_LIVE_PIXELS, Math\.round\(livePixelBudget\(\) \* 0\.75\)\)/);
  assert.match(main, /lastAdaptiveBudgetChangeAt = now/);
  assert.match(main, /lastCanvasLayout = null/);
  assert.match(main, /scheduleFitCanvas\(\)/);
  assert.match(main, /if \(stableFpsSamples >= 8 && adaptiveLivePixels && adaptiveLivePixels < baseLivePixelBudget\(\)\)/);
  assert.match(main, /observeLiveFps\(fps\)/);
  assert.match(main, /holdAdaptiveBudget\(3200\)/);
  assert.match(main, /resetAdaptiveLiveBudget\(\)/);
});

test("keeps production cache policy explicit for static shader assets", () => {
  const vercel = JSON.parse(source("vercel.json")) as {
    headers: Array<{ source: string; headers: Array<{ key: string; value: string }> }>;
  };
  const layout = source("app/layout.tsx");
  const headerValue = (source: string, key: string) =>
    vercel.headers
      .find((entry) => entry.source === source)
      ?.headers.find((header) => header.key === key)
      ?.value;
  const headerSources = vercel.headers.map((entry) => entry.source);
  const staticCacheSources = [
    "/dx-shader/fonts/(.*)",
    "/dx-shader/js/(.*)",
    "/dx-shader/styles.css",
    "/dx-shader/docs.css",
    "/dx-shader/docs.html",
    "/dx-shader/assets/(.*)",
  ];
  const csp = headerValue("/(.*)", "Content-Security-Policy") || "";

  assert.doesNotMatch(layout, /<meta\s+name="description"/);
  assert.equal(new Set(headerSources).size, headerSources.length);
  for (const source of staticCacheSources) {
    const entry = vercel.headers.find((item) => item.source === source);
    assert.ok(entry, `${source} should have explicit headers`);
    assert.equal(entry.headers.filter((header) => header.key === "Cache-Control").length, 1);
  }
  assert.equal(headerValue("/dx-shader/fonts/(.*)", "Cache-Control"), "public, max-age=31536000, immutable");
  assert.equal(headerValue("/dx-shader/js/(.*)", "Cache-Control"), "public, max-age=300, stale-while-revalidate=86400");
  assert.equal(headerValue("/dx-shader/styles.css", "Cache-Control"), "public, max-age=300, stale-while-revalidate=86400");
  assert.equal(headerValue("/dx-shader/docs.css", "Cache-Control"), "public, max-age=300, stale-while-revalidate=86400");
  assert.equal(headerValue("/dx-shader/docs.html", "Cache-Control"), "public, max-age=300, stale-while-revalidate=86400");
  assert.equal(headerValue("/dx-shader/assets/(.*)", "Cache-Control"), "public, max-age=86400, stale-while-revalidate=604800");
  assert.match(csp, /script-src 'self'/);
  assert.match(csp, /connect-src 'self'/);
  assert.match(csp, /font-src 'self'/);
  assert.match(csp, /style-src 'self'/);
  assert.match(csp, /worker-src 'self' blob:/);
  assert.doesNotMatch(csp, /unsafe-inline|unsafe-eval|https?:|fonts\.googleapis|fonts\.gstatic/);
  assert.equal(headerValue("/(.*)", "X-Content-Type-Options"), "nosniff");
  assert.equal(headerValue("/(.*)", "Referrer-Policy"), "no-referrer");
  assert.equal(
    headerValue("/(.*)", "Permissions-Policy"),
    "camera=(), microphone=(), geolocation=(), payment=(), usb=()"
  );
});
