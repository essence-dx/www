const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `${relativePath} should exist`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Reactive Store react-context visibility is wired into the launch package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/packages/reactive-store.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const catalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/reactive-store.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "reactive/store",
  );

  assert.ok(visibility, "Reactive Store package-status visibility row is missing");
  assert.equal(visibility.official_package_name, "Reactive Store");
  assert.equal(visibility.upstream_package, "@tanstack/store");
  assert.equal(visibility.upstream_version, "0.11.0");
  assert.equal(
    visibility.source_mirror,
    "G:/WWW/inspirations/tanstack-store",
  );
  assert.equal(visibility.status, "present");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(
    visibility.package_receipt_path,
    ".dx/forge/receipts/packages/reactive-store.json",
  );
  assert.deepEqual(visibility.status_vocabulary, statusVocabulary);

  const contextSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "react-context",
  );

  assert.ok(contextSurface, "react-context surface is missing");
  assert.equal(contextSurface.status, "present");
  assert.equal(
    contextSurface.receipt_path,
    ".dx/forge/receipts/packages/reactive-store.json",
  );
  assert.deepEqual(contextSurface.files, [
    "lib/forge/state/reactive-store/context.tsx",
    "lib/forge/state/reactive-store/metadata.ts",
    "lib/forge/state/reactive-store/README.md",
  ]);
  assert.equal(contextSurface.hash_algorithm, "sha256");
  assert.deepEqual(
    contextSurface.file_hashes,
    Object.fromEntries(
      contextSurface.files.map((filePath) => [
        filePath,
        receipt.file_hashes[filePath],
      ]),
    ),
  );

  const runbookSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "source-guard-runbook-fixture",
  );

  assert.ok(runbookSurface, "source-guard runbook fixture surface is missing");
  assert.equal(runbookSurface.status, "present");
  assert.equal(runbookSurface.hash_algorithm, "sha256");
  assert.deepEqual(runbookSurface.files, [
    "docs/packages/reactive-store.source-guard-runbook.json",
  ]);
  assert.equal(
    runbookSurface.file_hashes["docs/packages/reactive-store.source-guard-runbook.json"],
    receipt.file_hashes["docs/packages/reactive-store.source-guard-runbook.json"],
  );

  assert.equal(receipt.schema, "forge.package_add_receipt");
  assert.equal(receipt.official_package_name, "Reactive Store");
  assert.equal(receipt.package_id, "reactive/store");
  assert.equal(receipt.upstream_package, "@tanstack/store");
  assert.equal(receipt.based_on, "@tanstack/react-store");
  assert.equal(receipt.source_mirror, "G:/WWW/inspirations/tanstack-store");
  assert.deepEqual(receipt.selected_surfaces, [
    "react-context",
    "source-guard-runbook-fixture",
    "preview-manifest-materializer",
    "template-reactive-dashboard-store",
  ]);
  for (const filePath of contextSurface.files) {
    assert.ok(receipt.files.includes(filePath), `${filePath} missing from receipt files`);
  }
  assert.ok(
    receipt.files.includes(
      "examples/template/components/template-app/dashboard-reactive-store.ts",
    ),
    "Reactive Store dashboard source surface is missing from receipt files",
  );
  assert.equal(receipt.status, "present");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.deepEqual(receipt.dx_check_visibility.status_vocabulary, statusVocabulary);
  assert.ok(receipt.dx_check_visibility.monitored_surfaces.includes("react-context"));
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.includes(
      "source-guard-runbook-fixture",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.includes(
      "preview-manifest-materializer",
    ),
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) => surface.id === "template-reactive-dashboard-store",
    ),
    "Reactive Store dashboard source should be monitored by dx-check visibility",
  );

  for (const filePath of receipt.files) {
    const sourcePath = filePath.startsWith("examples/template/")
      ? filePath
      : `examples/template/${filePath}`;
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(sourcePath),
      `${filePath} hash is stale in Reactive Store receipt`,
    );
  }
  assert.equal(
    receipt.file_hashes["docs/packages/reactive-store.source-guard-runbook.json"],
    sha256("docs/packages/reactive-store.source-guard-runbook.json"),
    "Reactive Store runbook fixture hash is stale in the package receipt",
  );
  assert.equal(
    receipt.file_hashes["tools/launch/materialize-www-template.ts"],
    sha256("tools/launch/materialize-www-template.ts"),
    "Reactive Store preview-manifest materializer hash is stale in the package receipt",
  );

  const materializerSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "reactive-store-preview-manifest-materializer",
  );

  assert.ok(
    materializerSurface,
    "Reactive Store preview-manifest materializer surface is missing",
  );
  assert.equal(materializerSurface.status, "present");
  assert.equal(materializerSurface.hash_algorithm, "sha256");
  assert.deepEqual(materializerSurface.files, [
    "tools/launch/materialize-www-template.ts",
  ]);
  assert.equal(
    materializerSurface.file_hashes["tools/launch/materialize-www-template.ts"],
    receipt.file_hashes["tools/launch/materialize-www-template.ts"],
  );

  const dashboardStoreSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === "template-reactive-dashboard-store",
  );
  assert.ok(
    dashboardStoreSurface,
    "Reactive Store dashboard source surface is missing",
  );
  assert.equal(dashboardStoreSurface.status, "present");
  assert.deepEqual(dashboardStoreSurface.files, [
    "examples/template/components/template-app/dashboard-reactive-store.ts",
  ]);
  assert.ok(
    dashboardStoreSurface.source_markers.includes("createDashboardReactiveStore") &&
      dashboardStoreSurface.source_markers.includes(
        'data-dx-reactive-store-runtime="source-owned-template-store"',
      ),
    "Reactive Store dashboard source markers are missing",
  );
  assert.equal(
    dashboardStoreSurface.file_hashes[
      "examples/template/components/template-app/dashboard-reactive-store.ts"
    ],
    receipt.file_hashes[
      "examples/template/components/template-app/dashboard-reactive-store.ts"
    ],
  );

  const contextSource = read(
    "examples/template/lib/forge/state/reactive-store/context.tsx",
  );
  assert.match(contextSource, /export function createStoreContext<TValue extends object>\(\)/);
  assert.match(contextSource, /createContext<TValue \| null>\(null\)/);
  assert.match(contextSource, /Context\.displayName = "StoreContext"/);
  assert.match(contextSource, /throw new Error\("Missing StoreProvider for StoreContext"\)/);

  for (const marker of [
    "lib/forge/state/reactive-store/context.tsx#createStoreContext",
    "createStoreContext",
    "StoreProvider",
    "useStoreContext",
  ]) {
    assert.ok(
      contextSurface.source_markers.includes(marker),
      `${marker} missing from Reactive Store react-context markers`,
    );
  }

  for (const metric of [
    "reactive_store_receipt_present",
    "reactive_store_receipt_stale",
    "reactive_store_missing_receipt",
    "reactive_store_blocked_surface",
    "reactive_store_unsupported_surface",
    "reactive_store_hash_manifest_present",
    "reactive_store_hash_mismatch",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Reactive Store visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.ok(
    status.zed_receipt_surfaces.includes("reactive-store:react-context"),
    "Reactive Store Zed receipt surface is missing",
  );
  assert.match(readModel, /export const reactiveStorePackageVisibility/);
  assert.match(readModel, /reactiveStorePackageVisibility,/);
  assert.match(statusSource, /reactiveStorePackageVisibility/);
  assert.match(statusSource, /reactiveStoreVisibility: reactiveStorePackageVisibility/);
  assert.match(catalog, /packageId: "reactive\/store"[\s\S]*?currentStatus: "present"/);
  assert.match(packageDoc, /## Package Status Read Model/);
  assert.match(packageDoc, /package-status row is `present`/);
});
