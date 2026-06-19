const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-validation-zod-dashboard-settings.json";
const templateFormsSourcePath =
  "examples/template/lib/validation/zod/template-forms.ts";
const templateFormsComponentPath =
  "examples/template/components/template-app/forms.tsx";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("default template forms stay wired to the Validation & Schemas receipt without runtime imports", () => {
  const schemas = read(templateFormsSourcePath);
  const forms = read(templateFormsComponentPath);
  const receipt = readJson(receiptPath);
  const packageStatus = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageDoc = read("docs/packages/validation-zod.md");

  const validationRow = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "validation/zod",
  );
  assert.ok(validationRow, "validation/zod package-status row is missing");
  const templateSurface = validationRow.selected_surfaces.find(
    (surface) => surface.surface_id === "template-forms-validation",
  );
  assert.ok(templateSurface, "template-forms-validation selected surface is missing");

  assert.match(schemas, /surfaceId: "template-forms-validation"/);
  assert.match(
    schemas,
    new RegExp(`receiptPath:\\s*(?:\\r?\\n\\s*)?"${receiptPath}"`),
  );
  assert.match(schemas, /sourceOwnedApi: "lib\/validation\/zod\/template-forms\.ts"/);
  assert.match(schemas, /formsComponent: "components\/template-app\/forms\.tsx"/);
  assert.match(schemas, /hashAlgorithm: "sha256"/);

  assert.match(forms, /templateFormValidationMetadata/);
  assert.match(
    forms,
    /data-dx-validation-surface=\{templateFormValidationMetadata\.surfaceId\}/,
  );
  assert.match(
    forms,
    /data-dx-validation-receipt-path=\{templateFormValidationMetadata\.receiptPath\}/,
  );
  assert.match(
    forms,
    /data-dx-source-owned-api=\{templateFormValidationMetadata\.sourceOwnedApi\}/,
  );
  assert.match(
    forms,
    /data-dx-validation-schema=\{templateFormValidationMetadata\.schema\}/,
  );

  assert.ok(receipt.files.includes(templateFormsSourcePath));
  assert.ok(receipt.files.includes(templateFormsComponentPath));
  assert.match(receipt.file_hashes[templateFormsSourcePath], /^[a-f0-9]{64}$/);
  assert.match(receipt.file_hashes[templateFormsComponentPath], /^[a-f0-9]{64}$/);

  assert.deepEqual(templateSurface.files, [
    "lib/validation/zod/template-forms.ts",
    "components/template-app/forms.tsx",
  ]);
  assert.equal(
    templateSurface.file_hashes[templateFormsSourcePath],
    receipt.file_hashes[templateFormsSourcePath],
  );
  assert.equal(
    templateSurface.file_hashes[templateFormsComponentPath],
    receipt.file_hashes[templateFormsComponentPath],
  );
  assert.equal(
    validationRow.source_hashes.files[templateFormsComponentPath],
    receipt.file_hashes[templateFormsComponentPath],
  );
  assert.equal(validationRow.receipt_hash_refresh.tracked_file_count, 14);
  assert.ok(
    validationRow.receipt_hash_refresh.tracked_files.includes(
      templateFormsComponentPath,
    ),
  );

  assert.match(
    readModel,
    /"examples\/template\/components\/template-app\/forms\.tsx":\s*\r?\n\s*"[a-f0-9]{64}"/,
  );
  assert.match(readModel, /trackedFileCount: 14/);
  assert.match(packageDoc, /tracks 14 selected files/);
});
