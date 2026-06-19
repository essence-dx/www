const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real Zod stringbool env helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/env.ts", ZOD_ENV_TS)',
    "const ZOD_ENV_TS",
    "z.stringbool",
    "dxStringBool",
    "dxLaunchEnvFlagsSchema",
    "parseDxLaunchEnvFlags",
    "safeParseDxLaunchEnvFlags",
    "encodeDxStringBool",
    "DX_ENABLE_RUNTIME_PREVIEW",
    "DX_REQUIRE_SOURCE_RECEIPTS",
    '"z.stringbool"',
    '"lib/validation/zod/env.ts"',
    'envFlagHelper: "parseDxLaunchEnvFlags(env)"',
  ]) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});

test("launch template consumes Zod stringbool env parsing", () => {
  const status = read("examples/template/zod-validation-status.tsx");

  for (const marker of [
    '@/lib/validation/zod/env',
    "parseDxLaunchEnvFlags",
    "encodeDxStringBool",
    "data-dx-zod-env-status",
    "source-receipts-required",
  ]) {
    assert.match(status, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing ${marker}`);
  }
});
