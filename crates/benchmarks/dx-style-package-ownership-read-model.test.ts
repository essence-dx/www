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
        "preview-style-package-ownership-read-model.ts",
      ),
    ).href,
  );
}

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("dx-style package ownership read model keeps generated classes and unsupported warnings package-scoped", async () => {
  const readModel = await importReadModel();

  const model = readModel.dxStylePackageOwnershipFromPreviewAndReadiness({
    routes: [
      {
        route: "/",
        stylePackageOwnershipRows: [
          {
            packageId: "shadcn/ui/button",
            packageName: "UI Components",
            styleScope: "ui-components",
            sourceFiles: ["examples/template/components/ui/button.tsx"],
            requiredTokens: ["primary", "primary-foreground", "ring"],
            generatedClasses: [
              "inline-flex",
              "items-center",
              "rounded-md",
              "bg-token(surface)",
            ],
            unsupportedClasses: [],
            tokenSource: "styles/globals.css",
            generatedCss: "styles/generated.css",
            receiptPath:
              ".dx/forge/receipts/packages/shadcn-ui-button.json",
            zedVisibility: "shadcn-ui-button:style-ownership",
            runtimeProof: false,
          },
          {
            packageId: "animation/motion",
            packageName: "Motion Animation",
            styleScope: "motion-animation",
            sourceFiles: [
              "examples/template/components/template-app/motion.tsx",
            ],
            requiredTokens: ["accent", "accent-foreground"],
            generatedClasses: ["motion-safe:animate-pulse"],
            unsupportedClasses: [
              {
                className: "animate-[var(--package-animation)]",
                reason:
                  "arbitrary animation value ownership is recorded, but generated CSS parity is not proved for this package yet",
              },
            ],
            tokenSource: "styles/globals.css",
            generatedCss: "styles/generated.css",
            receiptPath:
              ".dx/forge/receipts/packages/animation-motion.json",
            zedVisibility: "animation-motion:style-ownership",
            runtimeProof: false,
          },
        ],
      },
    ],
  });

  assert.equal(
    model.schema,
    "dx.www.template.style_package_ownership_read_model",
  );
  assert.equal(model.source, "preview_manifest.routes[/].stylePackageOwnershipRows");
  assert.equal(model.route, "/");
  assert.equal(model.packageCount, 2);
  assert.equal(model.classOwnershipCount, 5);
  assert.equal(model.unsupportedClassCount, 1);
  assert.deepEqual(model.packageIds, ["animation/motion", "shadcn/ui/button"]);
  assert.deepEqual(model.tokenUsage, [
    "accent",
    "accent-foreground",
    "primary",
    "primary-foreground",
    "ring",
  ]);
  assert.equal(model.readsHtml, false);
  assert.equal(model.readsRawStyleReceipt, false);
  assert.equal(model.readsCheckReceipt, false);
  assert.equal(model.readsReadinessReceipt, false);

  const motion = model.packages.find(
    (row) => row.packageId === "animation/motion",
  );
  assert.ok(motion, "expected animation/motion ownership row");
  assert.equal(motion.unsupportedClasses[0].packageId, "animation/motion");
  assert.equal(
    motion.unsupportedClasses[0].reason,
    "arbitrary animation value ownership is recorded, but generated CSS parity is not proved for this package yet",
  );

  const missing =
    readModel.dxStylePackageOwnershipFromPreviewAndReadiness({}, {});
  assert.equal(missing.packageCount, 0);
  assert.equal(missing.classOwnershipCount, 0);
  assert.equal(missing.unsupportedClassCount, 0);
  assert.equal(missing.source, "missing");
});

test("dx-style package ownership metadata is published through preview-manifest source, not raw receipts", () => {
  const materializer = readRequiredFile(
    "tools/launch/materialize-www-template.ts",
  );
  const readModel = readRequiredFile(
    "examples/template/preview-style-package-ownership-read-model.ts",
  );
  const launchShell = readRequiredFile("examples/template/template-shell.tsx");
  const editContract = readRequiredFile(
    "examples/template/dx-studio-edit-contract.ts",
  );
  const studioManifest = readRequiredFile("dx-www/src/cli/studio_manifest.rs");

  for (const marker of [
    "DX_STYLE_PACKAGE_OWNERSHIP_ROWS",
    "stylePackageOwnershipRows",
    "shadcn/ui/button",
    "animation/motion",
    "requiredTokens",
    "generatedClasses",
    "unsupportedClasses",
    "styles/generated.css",
  ]) {
    assert.match(
      materializer,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  for (const marker of [
    "data-dx-style-package-ownership-read-model",
    "data-dx-style-package-ownership-packages",
    "data-dx-style-package-ownership-generated-classes",
    "data-dx-style-package-ownership-unsupported-classes",
  ]) {
    const pattern = new RegExp(
      marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"),
    );
    assert.match(launchShell, pattern);
    assert.match(editContract, pattern);
    assert.match(studioManifest, pattern);
    assert.match(materializer, pattern);
  }

  assert.match(launchShell, /dxStylePackageOwnershipFromPreviewAndReadiness/);

  assert.doesNotMatch(
    readModel,
    /readFileSync|fs\.|querySelector|document\.|browser_compat_receipt_contract/,
  );
});

test("dx-style package ownership is promoted into lower dx-check and panel evidence", () => {
  const styleReceiptReader = readRequiredFile(
    "core/src/ecosystem/dx_style_receipts.rs",
  );
  const projectCheck = readRequiredFile(
    "core/src/ecosystem/project_check.rs",
  );
  const checkPanel = readRequiredFile(
    "core/src/ecosystem/dx_check_receipt.rs",
  );
  const publicFrameworkTools = readRequiredFile(
    "dx-www/src/cli/public_framework_tools.rs",
  );

  for (const marker of [
    "style_package_ownership_rows",
    "collect_style_package_ownership_rows",
    "stylePackageOwnershipRows",
    "dx.forge.package.dx_style_compatibility",
  ]) {
    assert.match(publicFrameworkTools, new RegExp(marker));
  }

  for (const marker of [
    "DxStylePackageOwnershipSummary",
    "DxStylePackageOwnershipRow",
    "DxStylePackageUnsupportedClassFinding",
    "dx_style_package_ownership_summary",
    "package_id",
    "unsupported_classes",
  ]) {
    assert.match(styleReceiptReader, new RegExp(marker));
  }

  for (const marker of [
    "dx_style_package_ownership_summary",
    "dx_style_package_ownership_receipt_present",
    "dx_style_package_ownership_package_count",
    "dx_style_package_ownership_generated_class_count",
    "dx_style_package_ownership_unsupported_class_count",
    "dx-style-package-owned-unsupported-class",
  ]) {
    assert.match(projectCheck, new RegExp(marker));
  }

  for (const marker of [
    "DX_STYLE_PACKAGE_OWNERSHIP_ROW_ID",
    "dx_style_package_ownership_panel_row",
    "package_ownership_package_ids",
    "package_ownership_unsupported_class_examples",
    "dx-style:package-ownership",
  ]) {
    assert.match(checkPanel, new RegExp(marker));
  }
});
