const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real transform and prefault helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/transforms.ts", ZOD_TRANSFORMS_TS)',
    "const ZOD_TRANSFORMS_TS",
    "z.preprocess",
    ".transform(",
    ".pipe(",
    ".prefault(",
    ".catch(",
    "dxLaunchScoreInputSchema",
    "parseDxLaunchScoreInput",
    "safeParseDxLaunchScoreInput",
    '"z.preprocess"',
    '"schema.transform"',
    '"schema.pipe"',
    '"schema.prefault"',
    '"schema.catch"',
    '"lib/validation/zod/transforms.ts"',
    'transformHelper: "parseDxLaunchScoreInput(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template and CLI consume Zod transform normalization", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");

  for (const marker of [
    "@/lib/validation/zod/transforms",
    "parseDxLaunchScoreInput",
    "data-dx-zod-transform-score",
    "normalizedScore",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/transforms",
    "parseDxLaunchScoreInput",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }
});
