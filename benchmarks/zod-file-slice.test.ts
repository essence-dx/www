const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real Zod file upload helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/files.ts", ZOD_FILES_TS)',
    "const ZOD_FILES_TS",
    "z.file()",
    ".min(1",
    ".max(",
    ".mime(",
    "dxLaunchAssetFileSchema",
    "safeParseDxLaunchAssetFile",
    "createDxLaunchAssetFileProbe",
    "dxLaunchAssetFileJsonSchema",
    '"z.file"',
    '"file.min"',
    '"file.max"',
    '"file.mime"',
    '"lib/validation/zod/files.ts"',
    'fileHelper: "safeParseDxLaunchAssetFile(file)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template and CLI consume Zod file upload validation", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");

  for (const marker of [
    "@/lib/validation/zod/files",
    "createDxLaunchAssetFileProbe",
    "data-dx-zod-file-status",
    "asset-file",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/files",
    "safeParseDxLaunchAssetFile",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }
});
