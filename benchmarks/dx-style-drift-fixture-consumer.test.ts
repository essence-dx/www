const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const { pathToFileURL } = require("node:url");

const root = path.resolve(__dirname, "..");

async function importReadModel() {
  return import(
    pathToFileURL(
      path.join(
        root,
        "examples",
        "www-template",
        "preview-style-package-panel-read-model.ts",
      ),
    ).href
  );
}

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("dx-style drift fixture metadata is consumed without scraping receipts", async () => {
  const readModel = await importReadModel();

  const previewManifest = {
    routes: [
      {
        route: "/",
        styleEvidenceDriftFixtures: [
          {
            rowId: "dx-style-browser-compat",
            status: "source-guarded",
            loaderFile: "components/template-app/template-shell-evidence-loader.ts",
            markerHelperFile:
              "components/template-app/template-shell-style-evidence-drift.ts",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            states: ["unknown", "false", "true"],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
      },
    ],
  };
  const readinessReceipt = {
    style_evidence_drift_fixture: {
      row_id: "dx-style-browser-compat",
      route: "/",
      status: "source-guarded",
      loader_file: "components/template-app/template-shell-evidence-loader.ts",
      marker_helper_file:
        "components/template-app/template-shell-style-evidence-drift.ts",
      fixture_path:
        "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
      states: ["unknown", "false", "true"],
      full_autoprefixer_parity: false,
      full_tailwind_postcss_output_parity: false,
    },
  };

  const drift =
    readModel.dxStyleDriftFixtureFromPreviewAndReadiness(
      previewManifest,
      readinessReceipt,
    );

  assert.equal(
    drift.schema,
    "dx.www.template.style_evidence_drift_fixture_read_model",
  );
  assert.equal(drift.exerciseState, "source-guarded");
  assert.equal(drift.exercised, true);
  assert.equal(drift.readsHtml, false);
  assert.equal(drift.readsRawStyleReceipt, false);
  assert.equal(drift.readsReadinessReceipt, true);
  assert.deepEqual(drift.states, ["unknown", "false", "true"]);
  assert.equal(drift.fullAutoprefixerParity, false);
  assert.equal(drift.fullTailwindPostcssOutputParity, false);
  assert.deepEqual(drift.mismatchFields, []);

  const missing =
    readModel.dxStyleDriftFixtureFromPreviewAndReadiness({}, {});
  assert.equal(missing.exerciseState, "missing");
  assert.equal(missing.exercised, false);
  assert.equal(missing.fullAutoprefixerParity, false);
  assert.equal(missing.fullTailwindPostcssOutputParity, false);
});

test("dx-style package panel consumes drift fixture state without raw receipt reads", async () => {
  const readModel = await importReadModel();
  const source = readRequiredFile(
    "examples/template/preview-style-package-panel-read-model.ts",
  );

  const previewManifest = {
    routes: [
      {
        route: "/",
        styleEvidenceRows: [
          {
            rowId: "dx-style-browser-compat",
            title: "dx-style browser compatibility",
            status: "present",
            receiptPath: ".dx/receipts/style/check.json",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            zedVisibility: "dx-style:browser-compat",
            canaryClassCount: 3,
            tailwindParityStateAliasSupportedClassCount: 6,
            tailwindParitySupportedStateAliasExamples: ["target:p-4"],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
        styleEvidenceDriftFixtures: [
          {
            rowId: "dx-style-browser-compat",
            status: "source-guarded",
            loaderFile: "components/template-app/template-shell-evidence-loader.ts",
            markerHelperFile:
              "components/template-app/template-shell-style-evidence-drift.ts",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            states: ["unknown", "false", "true"],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
      },
    ],
  };
  const readinessReceipt = {
    style_evidence_drift_fixture: {
      row_id: "dx-style-browser-compat",
      route: "/",
      status: "source-guarded",
      loader_file: "components/template-app/template-shell-evidence-loader.ts",
      marker_helper_file:
        "components/template-app/template-shell-style-evidence-drift.ts",
      fixture_path:
        "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
      states: ["unknown", "false", "true"],
      full_autoprefixer_parity: false,
      full_tailwind_postcss_output_parity: false,
    },
  };

  const panel = readModel.dxStylePackagePanelFromPreviewAndReadiness(
    previewManifest,
    readinessReceipt,
  );

  assert.equal(
    panel.schema,
    "dx.www.template.preview_style_package_panel_with_drift_read_model",
  );
  assert.equal(panel.panelId, "dx-style-browser-compat-package-panel");
  assert.equal(panel.status, "present");
  assert.equal(panel.driftExerciseState, "source-guarded");
  assert.equal(panel.driftExercised, true);
  assert.equal(panel.driftStatus, "source-guarded");
  assert.equal(
    panel.driftLoaderFile,
    "components/template-app/template-shell-evidence-loader.ts",
  );
  assert.equal(
    panel.driftMarkerHelperFile,
    "components/template-app/template-shell-style-evidence-drift.ts",
  );
  assert.deepEqual(panel.driftStates, ["unknown", "false", "true"]);
  assert.deepEqual(panel.driftMismatchFields, []);
  assert.equal(panel.readsHtml, false);
  assert.equal(panel.readsRawStyleReceipt, false);
  assert.equal(panel.readsCheckReceipt, false);
  assert.equal(panel.readsReadinessReceipt, true);

  const mismatch = readModel.dxStylePackagePanelFromPreviewAndReadiness(
    previewManifest,
    {
      style_evidence_drift_fixture: {
        ...readinessReceipt.style_evidence_drift_fixture,
        marker_helper_file: "components/template-app/stale-drift-helper.ts",
      },
    },
  );
  assert.equal(mismatch.driftExerciseState, "mismatch");
  assert.deepEqual(mismatch.driftMismatchFields, ["markerHelperFile"]);

  assert.match(source, /dxStylePackagePanelFromPreviewAndReadiness/);
  assert.match(source, /dxStyleDriftFixtureFromPreviewAndReadiness/);
  assert.doesNotMatch(
    source,
    /readFileSync|fs\.|querySelector|document\.|browser_compat_receipt_contract/,
  );
});

test("dx-style drift fixture markers are discoverable by Studio contracts", () => {
  const editContract = readRequiredFile(
    "examples/template/dx-studio-edit-contract.ts",
  );
  const studioManifest = readRequiredFile("dx-www/src/cli/studio_manifest.rs");

  for (const marker of [
    "data-dx-check-style-evidence-drift",
    "data-dx-check-style-evidence-drift-state",
    "data-dx-check-style-evidence-drift-loader",
    "data-dx-check-style-evidence-drift-helper",
    "data-dx-check-style-evidence-drift-states",
  ]) {
    assert.match(
      editContract,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
    assert.match(
      studioManifest,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(editContract, /stateMarkers: dxCheckPanelStateMarkers/);
  assert.match(studioManifest, /studio_dx_check_panel_contract/);
});

test("dx-style drift fixture markers are emitted by launch check surfaces", () => {
  const launchShell = readRequiredFile("examples/template/template-shell.tsx");
  const staticLaunchPage = readRequiredFile(
    "tools/launch/runtime-template/pages/index.html",
  );
  const materializer = readRequiredFile(
    "tools/launch/materialize-www-template.ts",
  );

  for (const marker of [
    "data-dx-check-style-evidence-drift",
    "data-dx-check-style-evidence-drift-state",
    "data-dx-check-style-evidence-drift-loader",
    "data-dx-check-style-evidence-drift-helper",
    "data-dx-check-style-evidence-drift-states",
  ]) {
    const markerPattern = new RegExp(
      marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"),
    );

    assert.match(launchShell, markerPattern);
    assert.match(staticLaunchPage, markerPattern);
    assert.match(materializer, markerPattern);
  }

  assert.match(launchShell, /components\/template-app\/template-shell-evidence-loader\.ts/);
  assert.match(
    staticLaunchPage,
    /components\/template-app\/template-shell-style-evidence-drift\.ts/,
  );
  assert.match(materializer, /styleEvidenceDriftFixtures/);
});

test("dx-style package panel drift markers are emitted by check panel surfaces", () => {
  const launchShell = readRequiredFile("examples/template/template-shell.tsx");
  const staticLaunchPage = readRequiredFile(
    "tools/launch/runtime-template/pages/index.html",
  );
  const editContract = readRequiredFile(
    "examples/template/dx-studio-edit-contract.ts",
  );
  const studioManifest = readRequiredFile("dx-www/src/cli/studio_manifest.rs");
  const materializer = readRequiredFile(
    "tools/launch/materialize-www-template.ts",
  );

  for (const marker of [
    "data-dx-style-package-panel",
    "data-dx-style-package-panel-read-model",
    "data-dx-style-package-panel-drift-state",
    "data-dx-style-package-panel-drift-status",
    "data-dx-style-package-panel-drift-mismatch-fields",
    "data-dx-style-package-panel-readiness-receipt",
  ]) {
    const markerPattern = new RegExp(
      marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"),
    );

    assert.match(launchShell, markerPattern);
    assert.match(staticLaunchPage, markerPattern);
    assert.match(editContract, markerPattern);
    assert.match(studioManifest, markerPattern);
    assert.match(materializer, markerPattern);
  }

  assert.match(launchShell, /dxStylePackagePanelMarkersFromPreviewAndReadiness/);
  assert.match(staticLaunchPage, /dx-style-browser-compat-package-panel/);
  assert.match(
    staticLaunchPage,
    /dx\.www\.template\.preview_style_package_panel_with_drift_read_model/,
  );
  assert.match(materializer, /preview_style_package_panel_with_drift_read_model/);
});

test("dx-style package panel marker state hydrates from the combined read model", async () => {
  const readModel = await importReadModel();
  const launchShell = readRequiredFile("examples/template/template-shell.tsx");

  const previewManifest = {
    routes: [
      {
        route: "/",
        styleEvidenceRows: [
          {
            rowId: "dx-style-browser-compat",
            title: "dx-style browser compatibility",
            status: "present",
            receiptPath: ".dx/receipts/style/check.json",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            zedVisibility: "dx-style:browser-compat",
            canaryClassCount: 3,
            tailwindParityStateAliasSupportedClassCount: 6,
            tailwindParitySupportedStateAliasExamples: ["target:p-4"],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
        styleEvidenceDriftFixtures: [
          {
            rowId: "dx-style-browser-compat",
            status: "source-guarded",
            loaderFile: "components/template-app/template-shell-evidence-loader.ts",
            markerHelperFile:
              "components/template-app/template-shell-style-evidence-drift.ts",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            states: ["unknown", "false", "true"],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
      },
    ],
  };
  const readinessReceipt = {
    style_evidence_drift_fixture: {
      row_id: "dx-style-browser-compat",
      route: "/",
      status: "source-guarded",
      loader_file: "components/template-app/template-shell-evidence-loader.ts",
      marker_helper_file:
        "components/template-app/template-shell-style-evidence-drift.ts",
      fixture_path:
        "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
      states: ["unknown", "false", "true"],
      full_autoprefixer_parity: false,
      full_tailwind_postcss_output_parity: false,
    },
  };

  const markers = readModel.dxStylePackagePanelMarkersFromPreviewAndReadiness(
    previewManifest,
    readinessReceipt,
  );

  assert.equal(markers.panelId, "dx-style-browser-compat-package-panel");
  assert.equal(
    markers.readModel,
    "dx.www.template.preview_style_package_panel_with_drift_read_model",
  );
  assert.equal(markers.driftState, "source-guarded");
  assert.equal(markers.driftStatus, "source-guarded");
  assert.equal(markers.readinessReceipt, "read");
  assert.deepEqual(markers.driftMismatchFields, []);

  const mismatchMarkers =
    readModel.dxStylePackagePanelMarkersFromPreviewAndReadiness(
      previewManifest,
      {
        style_evidence_drift_fixture: {
          ...readinessReceipt.style_evidence_drift_fixture,
          marker_helper_file: "components/template-app/stale-drift-helper.ts",
        },
      },
    );
  assert.equal(mismatchMarkers.driftState, "mismatch");
  assert.deepEqual(mismatchMarkers.driftMismatchFields, ["markerHelperFile"]);

  assert.match(launchShell, /dxStylePackagePanelFromPreviewAndReadiness/);
  assert.match(launchShell, /dxStylePackagePanelMarkersFromPreviewAndReadiness/);
  assert.match(launchShell, /dxStyleBrowserCompatFallbackPackagePanel/);
  assert.match(launchShell, /hydrateDxStylePackagePanelMarkers/);
  assert.match(
    launchShell,
    /styleEvidenceRows = hydrateDxStylePackagePanelMarkers/,
  );
});
