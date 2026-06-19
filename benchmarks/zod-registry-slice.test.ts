const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real Zod metadata registry helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/registry.ts", ZOD_REGISTRY_TS)',
    "const ZOD_REGISTRY_TS",
    "z.registry",
    "z.globalRegistry",
    ".register(",
    ".meta(",
    ".describe(",
    "dxLaunchSchemaRegistry",
    "dxLaunchSignupSchemaWithMetadata",
    "readDxLaunchSchemaMetadata",
    "readDxGlobalSchemaMetadata",
    '"z.registry"',
    '"z.globalRegistry"',
    '"schema.register"',
    '"schema.meta"',
    '"schema.describe"',
    '"lib/validation/zod/registry.ts"',
    'registryHelper: "readDxLaunchSchemaMetadata(schema)"',
  ]) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});

test("launch template consumes Zod registry metadata in its status proof", () => {
  const status = read("examples/template/zod-validation-status.tsx");

  for (const marker of [
    '@/lib/validation/zod/registry',
    "dxLaunchSignupSchemaWithMetadata",
    "readDxLaunchSchemaMetadata",
    "readDxGlobalSchemaMetadata",
    "data-dx-zod-registry-status",
    "registered",
  ]) {
    assert.match(status, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});
