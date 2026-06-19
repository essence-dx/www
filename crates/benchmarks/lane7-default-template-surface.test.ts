import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(
  root,
  "tools",
  "launch",
  "materialize-www-template.ts",
);

const lane7PackageIds = [
  "shadcn/ui/button",
  "animation/motion",
  "3d/launch-scene",
  "wasm/bindgen",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("Lane 7 default template uses DX icons and honest UI/Motion/3D/WASM surfaces", () => {
  const iconPath = path.join(
    root,
    "examples",
    "www-template",
    "components",
    "icons",
    "icon.tsx",
  );
  assert.ok(fs.existsSync(iconPath), "DX Icon component should be source-owned");

  const iconSource = fs.readFileSync(iconPath, "utf8");
  const dxUiSource = read("examples/template/components/template-app/dx-ui.tsx");
  const dashboardSource = read(
    "examples/template/components/template-app/dashboard-page.tsx",
  );
  const realitySource = read(
    "examples/template/components/template-app/package-reality.ts",
  );
  const materializerSource = read("tools/launch/materialize-www-template.ts");
  const globals = read("examples/template/styles/globals.css");
  const runtimeSource = read("tools/launch/runtime-template/assets/launch-runtime.ts");

  assert.match(iconSource, /export function Icon/);
  assert.match(iconSource, /data-icon-source="dx-icons"/);
  assert.match(iconSource, /data-dx-icon=\{canonicalName\}/);
  assert.doesNotMatch(iconSource, /from ["']lucide-react["']/);
  assert.match(dxUiSource, /export \{ Icon \} from "\.\.\/icons\/icon";/);
  assert.doesNotMatch(dxUiSource, /const iconPaths/);

  assert.match(dashboardSource, /data-dx-component="lane7-forge-systems"/);
  assert.match(dashboardSource, /data-dx-lane7-boundary="lock-backed-source-owned-3d"/);
  assert.match(dashboardSource, /data-dx-lane7-boundary="source-only-wasm"/);
  assert.match(dashboardSource, /data-dx-component="lane7-motion-control"/);
  assert.match(dashboardSource, /data-dx-style-surface="lane7-forge-systems"/);
  assert.match(dashboardSource, /data-dx-score-scope="lane7-package-summary"/);
  assert.match(dashboardSource, /data-dx-score-scope="lane7-package-readiness-row"/);
  assert.match(dashboardSource, /Icon name="pack:motion"/);
  assert.match(dashboardSource, /Icon name="pack:three-scene"/);
  assert.match(dashboardSource, /Icon name="pack:wasm-bindgen"/);

  assert.match(realitySource, /"shadcn\/ui\/button": 84/);
  assert.match(realitySource, /"animation\/motion": 72/);
  assert.match(realitySource, /"3d\/launch-scene": 84/);
  assert.match(realitySource, /"wasm\/bindgen": 70/);

  assert.match(materializerSource, /function renderLane7ForgeSystems/);
  assert.doesNotMatch(
    materializerSource,
    /renderDxIcon\(["']lucide:/,
    "materialized template chrome should call the DX Icon system with canonical DX names",
  );
  assert.match(globals, /\.lane7-forge-systems/);
  assert.match(globals, /\.lane7-boundary-grid/);
  assert.match(runtimeSource, /function sceneTokenColor/);
  assert.match(runtimeSource, /gradientTokens/);
  assert.doesNotMatch(
    runtimeSource,
    /gradient:\s*\[\s*["']#[0-9a-f]{3,8}["']/i,
  );
  assert.doesNotMatch(runtimeSource, /overlay:\s*["']rgba\(/i);
  assert.doesNotMatch(runtimeSource, /text:\s*["']#[0-9a-f]{3,8}["']/i);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-lane7-template-"));
  try {
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      stdio: "pipe",
    });
    const dashboard = fs.readFileSync(
      path.join(dir, "pages", "dashboard.html"),
      "utf8",
    );
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
    );
    const dashboardRoute = manifest.routes.find(
      (route: { route: string }) => route.route === "/dashboard",
    );

    assert.match(dashboard, /data-dx-component="lane7-forge-systems"/);
    assert.match(dashboard, /data-dx-style-surface="lane7-forge-systems"/);
    assert.match(dashboard, /data-dx-component="lane7-motion-control"/);
    assert.match(dashboard, /data-dx-lane7-boundary="lock-backed-source-owned-3d"/);
    assert.match(dashboard, /data-dx-lane7-boundary="source-only-wasm"/);
    assert.match(
      dashboard,
      /class="lane7-score" data-dx-score-scope="lane7-package-summary" data-dx-lane7-score="78">78\/100/,
    );
    assert.match(
      dashboard,
      /data-dx-package="3d\/launch-scene"[^>]*data-dx-lane7-package-score="84"[\s\S]{0,1200}<strong data-dx-score-scope="lane7-package-readiness-row" data-dx-package-score="84">84\/100<\/strong>/,
    );
    assert.doesNotMatch(dashboard, /<strong>84\/100<\/strong>/);
    assert.match(dashboard, /data-dx-icon="pack:motion"/);
    assert.match(dashboard, /data-dx-icon="pack:three-scene"/);
    assert.match(dashboard, /data-dx-icon="pack:wasm-bindgen"/);
    assert.doesNotMatch(dashboard, /data-dx-icon="lucide:/);
    assert.doesNotMatch(dashboard, /lucide-react/);
    const runtime = fs.readFileSync(
      path.join(dir, "public", "launch-runtime.js"),
      "utf8",
    );
    assert.match(runtime, /function sceneTokenColor/);
    assert.doesNotMatch(
      runtime,
      /gradient:\s*\[\s*["']#[0-9a-f]{3,8}["']/i,
    );
    assert.doesNotMatch(runtime, /overlay:\s*["']rgba\(/i);
    assert.doesNotMatch(runtime, /text:\s*["']#[0-9a-f]{3,8}["']/i);

    for (const packageId of lane7PackageIds) {
      assert.match(dashboard, new RegExp(escapeRegExp(packageId)));
      assert.ok(
        dashboardRoute.forgePackages.includes(packageId),
        `${packageId} should be in the generated dashboard route packages`,
      );
    }
    assert.equal(dashboardRoute.forgePackages.includes("lucide-react"), false);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
