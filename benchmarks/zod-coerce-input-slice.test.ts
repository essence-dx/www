const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real coerce input helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/coerce.ts", ZOD_COERCE_TS)',
    "const ZOD_COERCE_TS",
    "z.coerce.string",
    "z.coerce.number",
    "z.coerce.boolean",
    "z.coerce.bigint",
    "z.coerce.date",
    "z.input",
    "dxLaunchSearchParamsSchema",
    "parseDxLaunchSearchParams",
    "safeParseDxLaunchSearchParams",
    "readDxLaunchSearchParamsFromUrl",
    '"z.coerce.string"',
    '"z.coerce.number"',
    '"z.coerce.boolean"',
    '"z.coerce.bigint"',
    '"z.coerce.date"',
    '"z.input"',
    '"lib/validation/zod/coerce.ts"',
    'coerceHelper: "parseDxLaunchSearchParams(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template, CLI, and registry consume Zod coercion", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");

  for (const marker of [
    "@/lib/validation/zod/coerce",
    "parseDxLaunchSearchParams",
    "data-dx-zod-coerce-page",
    "coercedSearchParams",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/coerce",
    "parseDxLaunchSearchParams",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }

  for (const marker of [
    "4.4.3-dx.12",
    "query-string coercion",
    "lib/validation/zod/coerce.ts",
    "z.coerce.number",
    "z.coerce.date",
  ]) {
    assert.match(
      registry,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing registry marker ${marker}`,
    );
  }
});
