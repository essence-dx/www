const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real error policy helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/errors.ts", ZOD_ERRORS_TS)',
    "const ZOD_ERRORS_TS",
    "z.config",
    "z.locales.en",
    "z.ZodErrorMap",
    "{ error: dxLaunchErrorMap }",
    "z.flattenError",
    "z.treeifyError",
    "z.prettifyError",
    "configureDxZodEnglishLocale",
    "safeParseDxLaunchSignupForDisplay",
    "formatDxZodErrorForDisplay",
    '"z.config"',
    '"z.locales"',
    '"schema.safeParse(errorMap)"',
    '"lib/validation/zod/errors.ts"',
    'errorPolicyHelper: "safeParseDxLaunchSignupForDisplay(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template and CLI consume Zod error policy", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");

  for (const marker of [
    "@/lib/validation/zod/errors",
    "safeParseDxLaunchSignupForDisplay",
    "data-dx-zod-error-policy",
    "displayValidation",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/errors",
    "configureDxZodEnglishLocale",
    "safeParseDxLaunchSignupForDisplay",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }

  for (const marker of [
    "4.4.3-dx.12",
    "display error policies",
    "lib/validation/zod/errors.ts",
    "z.config",
    "z.locales",
    "schema.safeParse(errorMap)",
  ]) {
    assert.match(
      registry,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing registry marker ${marker}`,
    );
  }
});
