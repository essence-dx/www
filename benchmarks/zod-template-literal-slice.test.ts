const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real Zod template literal path helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/patterns.ts", ZOD_PATTERNS_TS)',
    "const ZOD_PATTERNS_TS",
    "z.templateLiteral",
    "dxLaunchRoutePathSchema",
    "dxForgeReceiptPathSchema",
    "parseDxLaunchRoutePath",
    "safeParseDxForgeReceiptPath",
    '"/"',
    '".dx/forge/"',
    '"z.templateLiteral"',
    '"lib/validation/zod/patterns.ts"',
    'patternHelper: "parseDxLaunchRoutePath(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template consumes Zod template literal route and receipt guards", () => {
  const status = read("examples/template/zod-validation-status.tsx");

  for (const marker of [
    "@/lib/validation/zod/patterns",
    "parseDxLaunchRoutePath",
    "safeParseDxForgeReceiptPath",
    "data-dx-zod-pattern-status",
    "route-pattern",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});
