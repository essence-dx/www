const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function cssBlock(source, selector) {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = source.match(new RegExp(`${escaped}\\s*\\{([\\s\\S]*?)\\n\\}`));
  assert.ok(match, `missing CSS selector ${selector}`);
  return match[1];
}

test("HelloGlow uses the original smooth rainbow gradient shift", () => {
  const globals = read("examples/template/styles/globals.css");

  assert.match(globals, /--dx-ref-hello-spectrum:\s*#ff5b7a,\s*#ffb84d,\s*#fff06a,\s*#59ff83,\s*#45d8ff,\s*#7867ff,\s*#ff58d0,\s*#ff5b7a;/);
  assert.match(globals, /background:\s*linear-gradient\(90deg,\s*var\(--dx-ref-hello-spectrum\)\);/);
  assert.match(globals, /animation:\s*dx-ref-hello-shift 6s linear infinite;/);
  assert.match(globals, /@keyframes dx-ref-hello-shift/);
  assert.doesNotMatch(globals, /--dx-ref-hello-triad|dx-ref-hello-hue|dx-ref-hello-column-breathe|dx-ref-hello-hue-wash/);
});

test("Friday effect keeps its border active-only and contained in the component frame", () => {
  const sourceShowcase = read("examples/template/components/dx-ui/source-showcase.tsx");
  const globals = read("examples/template/styles/globals.css");
  const fridayScreen = cssBlock(globals, ".dx-ref-friday-screen");
  const fridayGlass = cssBlock(globals, ".dx-ref-friday-screen-glass");
  const fridayEdge = cssBlock(globals, ".dx-ref-friday-edge");
  const fridayChecked = cssBlock(globals, ".dx-ref-friday-toggle:checked ~ .dx-ref-friday-screen");
  const fridayBlocks = globals.match(/\.dx-ref-friday-toggle[\s\S]*?\.dx-ref-particle/);
  assert.ok(fridayBlocks, "missing Friday CSS block");
  const fridaySurfaceCss = [fridayScreen, fridayGlass, fridayEdge, fridayChecked].join("\n");

  assert.match(sourceShowcase, /data-dx-friday-contained-border="true"/);
  assert.doesNotMatch(sourceShowcase, /dx-ref-friday-shell|dx-ref-friday-surface|dx-ref-friday-voice|dx-ref-friday-wave|dx-ref-friday-panel/);
  assert.match(fridayScreen, /position: relative;/);
  assert.match(fridayScreen, /overflow: hidden;/);
  assert.match(fridayScreen, /border-radius: 22px;/);
  assert.doesNotMatch(fridayScreen, /position: fixed;/);
  assert.doesNotMatch(globals, /\.dx-ref-friday-shell|\.dx-ref-friday-surface|\.dx-ref-friday-voice|\.dx-ref-friday-wave|\.dx-ref-friday-panel/);
  assert.doesNotMatch(sourceShowcase, /Pixel particle avatar|dx-ref-particle|DxPixelCircle/);
  assert.match(globals, /\.dx-ref-friday-toggle:checked ~ \.dx-ref-friday-screen \.dx-ref-friday-edge-bottom/);
  assert.doesNotMatch(globals, /dx-ref-friday-theme-fill|dx-ref-friday-blue-fill/);
  assert.match(globals, /\.dx-ref-friday-edge::before,[\s\S]*#ff5b7a[\s\S]*#45d8ff[\s\S]*#ff58d0/);
  assert.match(globals, /\.dx-ref-friday-edge-left::before,[\s\S]*#ff5b7a[\s\S]*#45d8ff[\s\S]*#ff58d0/);
  assert.doesNotMatch(fridaySurfaceCss, /hsl\(var\(--/);
  assert.doesNotMatch(fridayBlocks[0], /hsl\(var\(--/);
});
