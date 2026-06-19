const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json";
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

test("Internationalization receipt visibility is consumed by the shared package-status read model", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const provider = readMirror(
    "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
  );
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/next-intl.md");
  const packageLock = read("examples/template/forge-package-lock.ts");
  const dashboardWorkflow = read(
    "examples/template/next-intl-dashboard-locale.tsx",
  );

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "i18n/next-intl",
  );

  assert.ok(visibility, "Internationalization visibility row is missing");
  assert.equal(visibility.official_package_name, "Internationalization");
  assert.equal(visibility.upstream_package, "next-intl");
  assert.equal(visibility.upstream_version, "4.12.0");
  assert.equal(visibility.source_mirror, "G:/WWW/inspirations/next-intl");
  assert.equal(visibility.status, "present");
  assert.equal(visibility.receipt_status, "present");
  assert.equal(visibility.package_receipt_path, receiptPath);
  assert.deepEqual(visibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );

  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "next-intl-dashboard-locale-workflow" &&
        surface.receipt_path === receiptPath &&
        surface.files.includes("components/template-app/next-intl-dashboard-locale.tsx") &&
        surface.files.includes("tools/launch/runtime-template/pages/index.html") &&
        surface.source_markers.includes('data-dx-package="i18n/next-intl"') &&
        surface.source_markers.includes(
          'data-dx-component="next-intl-dashboard-locale-workflow"',
        ) &&
        surface.source_markers.includes(
          'data-dx-intl-action="switch-dashboard-locale"',
        ) &&
        surface.source_markers.includes(
          'data-dx-style-surface="internationalization"',
        ) &&
        surface.source_markers.includes("data-dx-intl-format-source-api"),
    ),
    "Internationalization dashboard locale workflow surface is missing",
  );
  assert.match(
    dashboardWorkflow,
    /data-dx-style-surface="internationalization"/,
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "next-intl-dashboard-message-contract" &&
        surface.receipt_path === receiptPath &&
        surface.files.includes(
          "components/template-app/next-intl-dashboard-locale-contract.ts",
        ) &&
        surface.source_markers.includes("data-dx-intl-message-namespace"),
    ),
    "Internationalization message contract surface is missing",
  );
  assert.deepEqual(visibility.blocked_surfaces, []);
  assert.ok(
    visibility.unsupported_surfaces.includes("production-locale-routing-runtime"),
  );

  assert.equal(
    receipt.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(receipt.dx_style_compatibility.status, "present");
  assert.equal(receipt.dx_style_compatibility.token_source, "styles/globals.css");
  assert.equal(
    receipt.dx_style_compatibility.generated_css,
    "styles/globals.css",
  );
  assert.equal(receipt.dx_style_compatibility.runtime_proof, false);
  assert.ok(
    receipt.dx_style_compatibility.visible_surfaces.includes(
      "next-intl-dashboard-locale-workflow",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.source_files.includes(
      "examples/template/next-intl-dashboard-locale.tsx",
    ),
  );
  assert.ok(
    receipt.dx_style_compatibility.data_dx_markers.includes(
      'data-dx-style-surface="internationalization"',
    ),
  );
  assert.equal(
    visibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.equal(visibility.dx_style_compatibility.status, "present");
  assert.deepEqual(visibility.dx_style_compatibility.visible_surfaces, [
    "next-intl-dashboard-locale-workflow",
  ]);
  assert.ok(
    visibility.dx_style_compatibility.source_files.includes(
      "examples/template/next-intl-dashboard-locale.tsx",
    ),
  );

  for (const metric of [
    "internationalization_receipt_present",
    "internationalization_receipt_stale",
    "internationalization_missing_receipt",
    "internationalization_blocked_surface",
    "internationalization_unsupported_surface",
    "internationalization_hash_manifest_present",
    "internationalization_hash_mismatch",
    "internationalization_dx_style_compatibility_present",
    "internationalization_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Internationalization visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.ok(
    status.zed_receipt_surfaces.includes(
      "internationalization:next-intl-dashboard-locale-workflow",
    ),
    "Internationalization workflow surface is missing from Zed receipt surfaces",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "internationalization:next-intl-dashboard-message-contract",
    ),
    "Internationalization message contract surface is missing from Zed receipt surfaces",
  );
  assert.ok(status.receipts.examples.includes(receiptPath));

  assert.match(readModel, /export const internationalizationPackageVisibility/);
  assert.match(statusSource, /internationalizationPackageVisibility/);
  assert.match(
    statusSource,
    /internationalizationVisibility: internationalizationPackageVisibility/,
  );
  assert.match(packageLock, /checkInternationalizationPackageVisibility/);
  assert.match(packageLock, /internationalization_receipt_present/);
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(
    readModel,
    /visibleSurfaces: \[\s*"next-intl-dashboard-locale-workflow"/,
  );
  assert.match(packageDoc, /## DX-Style Compatibility/);
  assert.match(packageDoc, /internationalization_dx_style_compatibility_present/);
  assert.match(
    packageDoc,
    /internationalization-missing-dx-style-compatibility/,
  );
  assert.match(packageDoc, /shared dx-check\/Zed package-status read model/i);
});
