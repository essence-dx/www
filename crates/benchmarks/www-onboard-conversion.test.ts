import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const example = path.join(root, "examples", "onboard");

function read(relativePath) {
  return fs.readFileSync(path.join(example, relativePath), "utf8");
}

function exists(relativePath) {
  return fs.existsSync(path.join(example, relativePath));
}

test("onboard keeps the original Zen source and exposes a WWW render route", () => {
  const preservedSource = read("components/onboard-next-source.tsx");
  const page = read("app/page.tsx");

  assert.match(
    preservedSource,
    /<DndContext[\s\S]*<SidebarExpanded[\s\S]*<BrowserContent[\s\S]*<DragOverlay>/,
  );
  assert.match(preservedSource, /@\/lib\/zen-dnd/);
  assert.match(preservedSource, /@\/lib\/zen-motion/);
  assert.match(preservedSource, /@\/components\/icons\/lucide/);

  for (const marker of [
    'data-dx-component="zen-browser-onboard"',
    "Search or enter address...",
    "Personal",
    "Work",
    "Development",
    "zen basics",
    "Welcome!",
    "Beautiful themes for shadcn/ui - tweakcn | Theme Editor",
    "GitHub - zen-browser/desktop",
    "Every Level Of Intelligence Explained in 5 Minutes",
    "Start building with Gemini",
    "Premium Animations",
    "Activate Friday",
  ]) {
    assert.ok(page.includes(marker), `active route is missing ${marker}`);
  }

  for (const slopMarker of [
    "onboard-shell",
    'data-dx-component="onboard-zen-shell"',
    "Friday active",
    "source owned browser UI",
    "Ship preview",
    "Visual route ownership",
    "The editor preview belongs to DX WWW",
  ]) {
    assert.ok(!page.includes(slopMarker), `active route still contains ${slopMarker}`);
  }
});

test("onboard project boundary is Forge-first and source-owned", () => {
  assert.ok(exists("dx"), "extensionless dx config must exist");
  assert.match(read("dx"), /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(read("dx"), /app_dir=app/);
  assert.match(read("package.json"), /"nodeModulesRequired": false/);
  assert.match(read("package.json"), /"sourceProject": "onboard"/);

  for (const baggage of [
    "node_modules",
    ".next",
    ".env",
    ".env.local",
    ".kiro",
    ".vscode",
    "cursed",
    "hexed",
  ]) {
    assert.ok(!exists(baggage), `copied archive baggage remains: ${baggage}`);
  }
});

test("active WWW files do not import external Node package UI/runtime APIs", () => {
  const banned =
    /from\s+["'](?:@dnd-kit\/[^"']+|motion\/react|framer-motion|@radix-ui\/[^"']+|lucide-react|cmdk|next\/[^"']+|next-themes|react-markdown|re-resizable|tailwind-merge|clsx|@ai-sdk\/[^"']+|ai)["']/;

  for (const file of [
    "app/layout.tsx",
    "app/page.tsx",
    "components/onboard-next-source.tsx",
    "components/hello-glow.tsx",
    "components/friday.tsx",
    "components/browser/sidebar-expanded.tsx",
    "components/screens/welcome-screen.tsx",
    "components/theme-picker.tsx",
    "components/theme-switcher.tsx",
    "lib/zen-dnd.tsx",
    "lib/zen-motion.tsx",
    "lib/zen-theme.tsx",
  ]) {
    assert.doesNotMatch(read(file), banned, `${file} still imports an external package`);
  }
});

test("active route has the style hooks required by the preserved visual identity", () => {
  const css = read("styles/globals.css");

  for (const marker of [
    ".zen-preview-dock",
    ".zen-friday-preview-trigger",
    ".dx-eye-dot",
    ".dx-pixel-canvas",
    ".dx-pixel-shell",
    ".hello-glow-container",
    "@keyframes hello-gradient-shift",
  ]) {
    assert.ok(css.includes(marker), `missing CSS hook ${marker}`);
  }

  assert.match(read("app/layout.tsx"), /data-dx-scroll-system="native"/);
});

test("active route restores Friday and PixelCircle behavior through DX-owned runtime hooks", () => {
  const page = read("app/page.tsx");
  const runtime = read("public/onboard-runtime.js");
  const css = read("styles/globals.css");

  for (const marker of [
    'data-dx-onboard-runtime="source-owned"',
    'data-dx-friday-toggle',
    'data-dx-friday-border="top"',
    'data-dx-friday-border="bottom"',
    'data-dx-friday-border="left"',
    'data-dx-friday-border="right"',
    'data-dx-pixel-circle',
    'data-dx-pixel-toggle',
    'data-dx-pixel-control="speed"',
    'data-dx-pixel-control="resolution"',
    'data-dx-pixel-control="size"',
    'data-dx-pixel-control="overlap"',
    'data-dx-pixel-control="noise"',
  ]) {
    assert.ok(page.includes(marker), `active route is missing runtime hook ${marker}`);
  }

  assert.match(page, /<script[^>]+src="\/public\/onboard-runtime\.js"[^>]+defer[^>]+data-dx-onboard-runtime="source-owned"/);
  assert.deepEqual(
    [...page.matchAll(/<canvas[^>]+data-dx-pixel-circle="([^"]+)"/g)].map((match) => match[1]),
    ["green", "red", "purple", "pink"],
  );
  assert.doesNotMatch(page, /<div className="dx-pixel-circle dx-pixel-/);

  for (const marker of [
    "initDxFriday",
    "initDxPixelCircles",
    "requestAnimationFrame",
    "drawPixelCircle",
    "springScrollTo",
    "smoothScrollTo",
    "devicePixelRatio",
  ]) {
    assert.ok(runtime.includes(marker), `runtime is missing ${marker}`);
  }

  for (const marker of [
    ".friday-border",
    ".friday-border.visible",
    ".friday-glow",
    "@keyframes friday-gradient-shift",
    ".dx-pixel-canvas",
    ".dx-pixel-shell",
  ]) {
    assert.ok(css.includes(marker), `CSS is missing ${marker}`);
  }

  assert.doesNotMatch(runtime, /from\s+["']react["']|ReactDOM|node_modules/);
});
