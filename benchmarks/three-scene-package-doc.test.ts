const assert = require("node:assert");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function sha256(relativePath) {
  return crypto
    .createHash("sha256")
    .update(read(relativePath))
    .digest("hex");
}

test("3D Scene System package docs and metadata use official lane naming", () => {
  const docs = read("docs/packages/3d-scene-system.md");
  const catalog = read("examples/template/package-catalog.ts");
  const metadata = read("examples/template/scene/metadata.ts");
  const launchScene = read("examples/template/launch-scene.tsx");
  const packageStatus = JSON.parse(
    read("examples/template/.dx/forge/package-status.json"),
  );
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const forge = read("core/src/ecosystem/forge_three_scene.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const receipt = JSON.parse(
    read("examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"),
  );

  assert.match(docs, /^# 3D Scene System/m);
  assert.match(docs, /official_package_name: 3D Scene System/);
  assert.match(docs, /package_id: 3d\/launch-scene/);
  assert.match(docs, /upstream_package: three \+ @react-three\/fiber \+ @react-three\/drei/);
  assert.match(
    docs,
    /source_mirror: G:\/WWW\/inspirations\/three\.js; G:\/WWW\/inspirations\/react-three-fiber; G:\/WWW\/inspirations\/drei/,
  );
  assert.match(docs, /upstream_version: three 0\.184\.0; @react-three\/fiber 9\.6\.1; @react-three\/drei local mirror/);
  assert.match(docs, /honesty_label: LOCK-BACKED SOURCE-OWNED/);
  assert.match(docs, /src\/renderers\/WebGLRenderer\.js/);
  assert.match(docs, /packages\/fiber\/src\/core\/renderer\.tsx/);
  assert.match(docs, /src\/core\/Bounds\.tsx/);
  assert.match(docs, /src\/web\/KeyboardControls\.tsx/);
  assert.match(docs, /THREE\.WebGLRenderer/);
  assert.match(docs, /@react-three\/fiber createRoot\(canvas\)/);
  assert.match(docs, /@react-three\/drei Bounds\.fit/);
  assert.match(docs, /createDxSceneDashboardWorkflow/);
  assert.match(docs, /mountDxSceneWithRenderer/);
  assert.match(docs, /createDxSceneR3FDreiRendererAdapter/);
  assert.match(docs, /data-dx-package="3d\/launch-scene"/);
  assert.match(docs, /data-dx-component="launch-scene-dashboard-workflow"/);
  assert.match(docs, /data-dx-style-surface="launch-scene"/);
  assert.match(docs, /data-dx-token-scope="3d\/launch-scene"/);
  assert.match(docs, /data-dx-edit-id="launch\.scene"/);
  assert.match(docs, /present/);
  assert.match(docs, /stale/);
  assert.match(docs, /missing receipt/);
  assert.match(docs, /hash_algorithm: sha256/);
  assert.match(docs, /file_hashes/);
  assert.match(docs, /blocked/);
  assert.match(docs, /unsupported surface/);
  assert.match(docs, /dx\.forge\.package\.dx_style_compatibility/);
  assert.match(docs, /three_scene_system_dx_style_compatibility_present/);
  assert.match(docs, /three-scene-system-missing-dx-style-compatibility/);
  assert.doesNotMatch(docs, /^# Three/m);

  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?officialPackageName: "3D Scene System"/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?upstreamPackage: "three \+ @react-three\/fiber \+ @react-three\/drei"/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?command: "dx add 3d-scene-system --write"/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?"docs\/packages\/3d-scene-system\.md"/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?honestyLabel: "LOCK-BACKED SOURCE-OWNED"/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?dxStyleCompatibility: \{/,
  );
  assert.match(
    catalog,
    /packageId: "3d\/launch-scene",[\s\S]*?visibleSurfaces: \[[\s\S]*?"launch-scene-dashboard-workflow"/,
  );
  assert.match(catalog, /"3d\/launch-scene": \{\s*name: "3D Scene System"/);

  assert.match(metadata, /officialPackageName: "3D Scene System"/);
  assert.match(metadata, /honestyLabel: "LOCK-BACKED SOURCE-OWNED"/);
  assert.match(metadata, /docsPath: "docs\/packages\/3d-scene-system\.md"/);
  assert.match(metadata, /dxStyleCompatibility: \{/);
  assert.match(metadata, /visibleSurfaces: \[/);
  assert.match(metadata, /"launch-scene-dashboard-workflow"/);
  assert.match(metadata, /inspectedSourceFiles: \[/);
  assert.match(metadata, /"three\.js\/src\/renderers\/WebGLRenderer\.js"/);
  assert.match(metadata, /"react-three-fiber\/packages\/fiber\/src\/core\/renderer\.tsx"/);
  assert.match(metadata, /"drei\/src\/core\/Bounds\.tsx"/);
  assert.match(metadata, /selectedSurfaces: \[/);
  assert.match(metadata, /"launch-scene-dashboard-workflow"/);
  assert.match(metadata, /dxCheckVisibility: \{/);
  assert.match(metadata, /"missing receipt"/);

  assert.match(launchScene, /data-dx-edit-id="launch\.scene"/);
  assert.match(launchScene, /data-dx-edit-kind="visual-scene"/);
  assert.match(launchScene, /data-dx-edit-ops="design-token insert_icon_media scene-preview"/);
  assert.match(launchScene, /data-dx-style-surface="launch-scene"/);
  assert.match(launchScene, /data-dx-token-scope="3d\/launch-scene"/);

  assert.match(forge, /THREE_SCENE_OFFICIAL_PACKAGE_NAME: &str = "3D Scene System"/);
  assert.match(registry, /"3d-scene-system"/);
  assert.match(cli, /"official_name": "3D Scene System"/);
  assert.match(cli, /"command": "dx add 3d-scene-system --write"/);
  assert.match(cli, /"docs\/packages\/3d-scene-system\.md"/);

  assert.equal(receipt.package_name, "3D Scene System");
  assert.equal(receipt.official_package_name, "3D Scene System");
  assert.equal(receipt.package_id, "3d/launch-scene");
  assert.equal(receipt.upstream_package, "three + @react-three/fiber + @react-three/drei");
  assert.equal(
    receipt.source_mirror,
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
  );
  assert.equal(receipt.honesty_label, "LOCK-BACKED SOURCE-OWNED");
  assert.ok(receipt.selected_surfaces.includes("launch-scene-dashboard-workflow"));
  assert.ok(receipt.source_files.includes("docs/packages/3d-scene-system.md"));
  assert.ok(receipt.files.includes("docs/packages/3d-scene-system.md"));
  assert.equal(receipt.surface, "launch-scene-dashboard-workflow");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.equal(
    receipt.file_hashes["docs/packages/3d-scene-system.md"],
    sha256("docs/packages/3d-scene-system.md"),
  );
  assert.equal(
    receipt.file_hashes["examples/template/launch-scene.tsx"],
    sha256("examples/template/launch-scene.tsx"),
  );
  assert.equal(
    receipt.file_hashes["examples/template/scene/dashboard-workflow.ts"],
    sha256("examples/template/scene/dashboard-workflow.ts"),
  );
  assert.equal(
    receipt.file_hashes["examples/template/scene/dashboard-controls.ts"],
    sha256("examples/template/scene/dashboard-controls.ts"),
  );
  assert.equal(
    receipt.file_hashes["examples/template/scene/metadata.ts"],
    sha256("examples/template/scene/metadata.ts"),
  );
  assert.ok(receipt.provenance.inspected_source_files.includes("three.js/src/renderers/WebGLRenderer.js"));
  assert.deepStrictEqual(receipt.dx_check_visibility.statuses, [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ]);
  assert.equal(
    receipt.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dx_style_compatibility.status, "present");
  assert.equal(
    receipt.dx_style_compatibility.token_source,
    "examples/template/launch-scene.tsx",
  );
  assert.equal(
    receipt.dx_style_compatibility.generated_css,
    "examples/template/styles/globals.css",
  );
  assert.equal(receipt.dx_style_compatibility.runtime_proof, false);
  assert.ok(
    receipt.dx_style_compatibility.visible_surfaces.includes(
      "launch-scene-dashboard-workflow",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="launch-scene"',
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-token-scope="3d/launch-scene"',
    ),
  );

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "3d/launch-scene",
  );
  assert.equal(
    visibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(visibility.dx_style_compatibility.status, "present");
  assert.ok(
    visibility.dx_check_metrics.includes(
      "three_scene_system_dx_style_compatibility_present",
    ),
  );
  assert.ok(
    visibility.dx_check_metrics.includes(
      "three_scene_system_dx_style_compatibility_missing",
    ),
  );
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(readModel, /three_scene_system_dx_style_compatibility_present/);
  assert.strictEqual(
    receipt.dx_check_visibility.receipt_path,
    ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
  );
  assert.equal(receipt.runtime_execution, false);
});
