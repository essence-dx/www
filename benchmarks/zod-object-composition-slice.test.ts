const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real object composition helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/objects.ts", ZOD_OBJECTS_TS)',
    "const ZOD_OBJECTS_TS",
    ".safeExtend(",
    ".pick(",
    ".omit(",
    ".partial(",
    ".required(",
    "dxLaunchSignupDraftSchema",
    "dxLaunchSignupSubmissionSchema",
    "dxLaunchSignupPublicProfileSchema",
    "safeParseDxLaunchSignupSubmission",
    '"schema.safeExtend"',
    '"object.pick"',
    '"object.omit"',
    '"object.partial"',
    '"object.required"',
    '"lib/validation/zod/objects.ts"',
    'objectHelper: "safeParseDxLaunchSignupSubmission(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template, CLI, and registry consume Zod object composition", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");
  const registry = read("core/src/ecosystem/forge_registry.rs");

  for (const marker of [
    "@/lib/validation/zod/objects",
    "safeParseDxLaunchSignupSubmission",
    "data-dx-zod-object-status",
    "submissionValidation",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/objects",
    "safeParseDxLaunchSignupSubmission",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }

  for (const marker of [
    "4.4.3-dx.12",
    "object composition",
    "lib/validation/zod/objects.ts",
    ".safeExtend(",
    ".partial(",
    ".required(",
  ]) {
    assert.match(
      registry,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing registry marker ${marker}`,
    );
  }
});
