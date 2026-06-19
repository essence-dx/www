const assert = require("node:assert");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(file) {
  return fs.readFileSync(file, "utf8");
}

test("materialized launch runtime keeps the visible Motion proof", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-motion-launch-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

  const launch = read(path.join(dir, "pages", "index.html"));
  assert.match(launch, /data-dx-route="\/"/);
  assert.match(launch, /data-dx-package="animation\/motion"/);
  assert.match(launch, /data-dx-component="launch-motion-dashboard-summary"/);
  assert.match(launch, /data-dx-dashboard-card="animation"/);
  assert.match(launch, /id="mission-motion-status"/);
  assert.match(launch, /id="mission-motion-detail"/);
  assert.match(launch, /id="mission-motion-policy"/);
  assert.match(launch, /data-dx-component="motion-animation-card"/);
  assert.match(launch, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(launch, /data-dx-product-surface="launch-dashboard"/);
  assert.match(launch, /data-dx-motion-policy="app-owned-reduced-motion-preview"/);
  assert.match(launch, /data-dx-motion-reduced="system"/);
  assert.match(launch, /data-dx-motion-state="source-owned"/);
  assert.match(launch, /data-dx-motion-progress="34"/);
  assert.match(launch, /data-dx-motion-order="source-owned,interactive,preview-ready"/);
  assert.match(launch, /data-dx-motion-preference-storage="local-storage"/);
  assert.match(launch, /data-dx-motion-storage-key="dx\.launch\.motion\.dashboard"/);
  assert.match(launch, /data-dx-motion-keyboard-reorder="arrow-home-end"/);
  assert.match(launch, /data-dx-motion-keyboard-state="source-owned"/);
  assert.match(launch, /aria-activedescendant="motion-stage-source-owned"/);
  assert.match(launch, /tabindex="0"/);
  assert.match(launch, /data-dx-motion-interaction="advance-stage"/);
  assert.match(launch, /data-dx-motion-interaction="reverse-order"/);
  assert.match(launch, /data-dx-motion-interaction="move-stage-previous"/);
  assert.match(launch, /data-dx-motion-interaction="move-stage-next"/);
  assert.match(launch, /data-dx-motion-order-available="false"/);
  assert.match(launch, /data-dx-motion-order-available="true"/);
  assert.match(launch, /data-dx-motion-interaction="reset-proof"/);
  assert.match(launch, /data-dx-motion-interaction="toggle-reduced-motion"/);
  assert.match(launch, /data-dx-motion-progress-bar/);

  const runtime = read(path.join(dir, "public", "launch-runtime.js"));
  assert.match(runtime, /function renderMotionProof\(\)/);
  assert.match(runtime, /function bindMotion\(\)/);
  assert.match(runtime, /dashboard\.dataset\.dxDashboardMotion/);
  assert.match(runtime, /setText\(\s*"#mission-motion-status"/);
  assert.match(runtime, /setText\(\s*"#mission-motion-detail"/);
  assert.match(runtime, /dataset\.dxMotionState/);
  assert.match(runtime, /dataset\.dxMotionProgress/);
  assert.match(runtime, /dataset\.dxMotionOrder/);
  assert.match(runtime, /function moveMotionStage\(/);
  assert.match(runtime, /data-dx-motion-order-available/);
  assert.match(runtime, /function setMotionMoveAvailability\(/);
  assert.match(runtime, /function setMotionKeyboardReorderState\(/);
  assert.match(runtime, /function placeMotionStage\(/);
  assert.match(runtime, /const motionPreferenceStorageKey = "dx\.launch\.motion\.dashboard"/);
  assert.match(runtime, /function readMotionPreference\(/);
  assert.match(runtime, /function writeMotionPreference\(/);
  assert.match(runtime, /data-dx-motion-keyboard-state/);
  assert.match(runtime, /addEventListener\("keydown"/);
  assert.match(runtime, /dataset\.dxMotionReduced/);
  assert.match(runtime, /"#mission-motion-policy"/);

  const styles = read(path.join(dir, "styles", "globals.css"));
  const motionStyles = styles.match(/\.motion-proof[\s\S]*?\.markdown/)?.[0] ?? "";
  assert.match(motionStyles, /var\(--dx-motion-active-border\)/);
  assert.match(motionStyles, /var\(--dx-motion-active-bg\)/);
  assert.match(motionStyles, /var\(--dx-motion-progress-track\)/);
  assert.match(motionStyles, /\.motion-proof\[data-dx-motion-reduced="preview"\]/);
  assert.doesNotMatch(motionStyles, /#[0-9a-f]{3,8}|rgba?\(/i);

  const manifest = JSON.parse(read(path.join(dir, "public", "preview-manifest.json")));
  assert.ok(
    manifest.routes.some(
      (route) => route.route === "/" && route.forgePackages.includes("animation/motion"),
    ),
  );
});
