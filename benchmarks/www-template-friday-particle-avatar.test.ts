const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("WWW template wires the Friday particle avatar without client package dependencies", () => {
  const avatar = read("examples/template/components/template-app/friday-particle-avatar.tsx");
  const landingScene = read("examples/template/components/template-app/landing-scene.tsx");
  const landingPage = read("examples/template/components/template-app/landing-page.tsx");
  const sourceShowcase = read("examples/template/components/dx-ui/source-showcase.tsx");
  const globals = read("examples/template/styles/globals.css");

  assert.match(avatar, /data-dx-component="friday-particle-scene"/);
  assert.match(avatar, /data-dx-component="friday-particle-avatar"/);
  assert.match(avatar, /data-dx-component="friday-hello-glow"/);
  assert.match(avatar, /data-dx-component="friday-rainbow-voice-frame"/);
  assert.match(avatar, /data-dx-source="archive-friday-particle-study"/);
  assert.match(avatar, /data-dx-package="dx\/style dx\/icon dx\/effects\/friday-particle-avatar"/);
  assert.match(avatar, /data-dx-node-modules="forbidden"/);
  assert.match(avatar, /data-dx-runtime="tsx-css-only"/);

  assert.doesNotMatch(avatar, /"use client"|framer-motion|next\/image|data:image\/gif|setInterval|useEffect|useState/);
  assert.match(landingScene, /FridayParticleScene/);
  assert.match(landingScene, /data-dx-scene-mode="pure-visual"/);
  assert.match(landingScene, /data-dx-visual-mode="friday-particle-avatar"/);
  assert.match(landingPage, /LandingSceneSurface/);
  assert.match(landingPage, /data-dx-visual-mode="friday-particle-avatar"/);
  assert.match(sourceShowcase, /data-dx-component="friday-particle-scene"/);
  assert.match(sourceShowcase, /data-dx-component="friday-particle-avatar"/);
  assert.match(sourceShowcase, /data-dx-visual-mode="friday-particle-avatar"/);

  assert.match(globals, /\.dx-friday-landing-scene/);
  assert.match(globals, /\.dx-friday-hero-system/);
  assert.match(globals, /\.dx-particle-avatar/);
  assert.match(globals, /\.dx-hello-glow/);
  assert.match(globals, /\.dx-friday-effect/);
});
