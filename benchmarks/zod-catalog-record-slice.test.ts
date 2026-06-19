const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real catalog record helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/catalog.ts", ZOD_CATALOG_TS)',
    "const ZOD_CATALOG_TS",
    "z.strictObject",
    "z.record(",
    "z.partialRecord",
    ".catchall(",
    ".readonly(",
    "dxLaunchPackageCatalogSchema",
    "dxLaunchPackageRoleBucketSchema",
    "parseDxLaunchPackageCatalog",
    "summarizeDxLaunchPackageCatalog",
    "indexDxLaunchPackageCatalog",
    '"z.strictObject"',
    '"z.record"',
    '"z.partialRecord"',
    '"schema.readonly"',
    '"object.catchall"',
    '"lib/validation/zod/catalog.ts"',
    'catalogHelper: "parseDxLaunchPackageCatalog(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template and CLI consume Zod catalog validation", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");

  for (const marker of [
    "@/lib/validation/zod/catalog",
    "launchPackageCatalog",
    "summarizeDxLaunchPackageCatalog",
    "data-dx-zod-catalog-packages",
    "catalogSummary.packageCount",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/catalog",
    "parseDxLaunchPackageCatalog",
    "summarizeDxLaunchPackageCatalog",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }
});
