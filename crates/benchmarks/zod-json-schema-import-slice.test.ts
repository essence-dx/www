const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real JSON Schema import helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '"js/validation/zod/json-schema-import.ts"',
    "ZOD_JSON_SCHEMA_IMPORT_TS",
    "const ZOD_JSON_SCHEMA_IMPORT_TS",
    "z.fromJSONSchema",
    "defaultTarget: \"draft-2020-12\"",
    "dxLaunchExternalPackageJsonSchema",
    "dxLaunchExternalPackageSchema",
    "parseDxLaunchExternalPackage",
    "safeParseDxLaunchExternalPackage",
    "importDxLaunchJsonSchema",
    '"z.fromJSONSchema"',
    '"lib/validation/zod/json-schema-import.ts"',
    'jsonSchemaImportHelper: "safeParseDxLaunchExternalPackage(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template, CLI, and registry consume Zod JSON Schema import", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");

  for (const marker of [
    "@/lib/validation/zod/json-schema-import",
    "safeParseDxLaunchExternalPackage",
    "data-dx-zod-json-schema-loader-status",
    "externalPackageValidation",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "NEXT_FAMILIAR_ZOD_STATUS_TSX",
    'include_str!("../../../examples/template/zod-validation-status.tsx")',
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }

  for (const marker of [
    "4.4.3-dx.12",
    "JSON Schema import",
    "lib/validation/zod/json-schema-import.ts",
    "z.fromJSONSchema",
    "experimental",
  ]) {
    assert.match(
      registry,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing registry marker ${marker}`,
    );
  }
});
