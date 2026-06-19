import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const nextSceneFiles = [
  "examples/template/components/scene/launch-scene.tsx",
  "examples/template/lib/scene/index.ts",
  "examples/template/lib/scene/types.ts",
  "examples/template/lib/scene/preset.ts",
  "examples/template/lib/scene/interaction.ts",
  "examples/template/lib/scene/dashboard-workflow.ts",
  "examples/template/lib/scene/dashboard-controls.ts",
  "examples/template/lib/scene/frame-sample.ts",
  "examples/template/lib/scene/capability-report.ts",
  "examples/template/lib/scene/viewport-report.ts",
  "examples/template/lib/scene/bounds-report.ts",
  "examples/template/lib/scene/raycast-report.ts",
  "examples/template/lib/scene/preview-readiness.ts",
  "examples/template/lib/scene/performance-monitor.ts",
  "examples/template/lib/scene/renderer-handoff.ts",
  "examples/template/lib/scene/r3f-renderer-adapter.ts",
  "examples/template/lib/scene/webgl-runtime.ts",
  "examples/template/lib/scene/metadata.ts",
  "examples/template/lib/scene/README.md",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("3D Scene System materializes Next-shaped component and lib sources", () => {
  for (const relativePath of nextSceneFiles) {
    assert.ok(
      fs.existsSync(path.join(root, relativePath)),
      `${relativePath} should be materialized front-facing source`,
    );
  }

  const launchShell = read("examples/template/template-shell.tsx");
  assert.match(
    launchShell,
    /import \{ LaunchScene \} from "@\/components\/scene\/launch-scene";/,
  );

  const launchScene = read(
    "examples/template/components/scene/launch-scene.tsx",
  );
  assert.match(launchScene, /from "@\/lib\/scene\/dashboard-controls"/);
  assert.match(launchScene, /from "@\/lib\/scene\/dashboard-workflow"/);
  assert.match(launchScene, /from "@\/lib\/scene\/renderer-handoff"/);
  assert.match(launchScene, /data-dx-package="3d\/launch-scene"/);
  assert.match(launchScene, /data-dx-component="launch-scene-webgl-proof"/);
  assert.doesNotMatch(launchScene, /from "@\/scene\//);
  assert.doesNotMatch(launchScene, /from ["']lucide-react["']/);

  const libIndex = read("examples/template/lib/scene/index.ts");
  assert.match(libIndex, /createDxLaunchScenePreset/);
  assert.match(libIndex, /mountDxSceneWithRenderer/);
  assert.match(libIndex, /createDxSceneR3FDreiRendererAdapter/);
  assert.doesNotMatch(
    libIndex,
    /from ["'](?:three|@react-three\/fiber|@react-three\/drei)["']/,
  );

  const receipt = readJson(
    "examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
  );
  for (const relativePath of nextSceneFiles) {
    assert.ok(
      receipt.files.includes(relativePath),
      `${relativePath} should be tracked by the 3D receipt`,
    );
    assert.match(receipt.file_hashes[relativePath], /^[a-f0-9]{64}$/);
  }

  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  const visibility = packageStatus.package_lane_visibility.find(
    (entry: { package_id?: string }) => entry.package_id === "3d/launch-scene",
  );
  assert.ok(visibility, "3D Scene System package visibility should exist");
  const nextSurface = visibility.selected_surfaces.find(
    (surface: { surface_id?: string }) =>
      surface.surface_id === "three-scene-system-next-template-source",
  );
  assert.ok(nextSurface, "Next-shaped 3D source surface should be visible");
  assert.deepEqual(nextSurface.files, [...nextSceneFiles]);
  for (const relativePath of nextSceneFiles) {
    assert.match(nextSurface.file_hashes[relativePath], /^[a-f0-9]{64}$/);
    assert.ok(visibility.receipt_hash_refresh.tracked_files.includes(relativePath));
  }

  const readModel = read("examples/template/forge-package-status-read-model.ts");
  assert.match(readModel, /three-scene-system-next-template-source/);
  for (const relativePath of nextSceneFiles) {
    assert.match(readModel, new RegExp(relativePath.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
