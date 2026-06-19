const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("validation/zod materializes real refinement helpers", () => {
  const source = read("core/src/ecosystem/forge_zod.rs");

  for (const marker of [
    '("js/validation/zod/refinements.ts", ZOD_REFINEMENTS_TS)',
    "const ZOD_REFINEMENTS_TS",
    ".refine(",
    ".superRefine(",
    ".check(",
    "z.refine(",
    "ctx.addIssue",
    'code: "custom"',
    'path: ["sourceReceipts"]',
    "dxLaunchApprovalGateSchema",
    "dxLaunchApprovalIdSchema",
    "parseDxLaunchApprovalGate",
    "safeParseDxLaunchApprovalGate",
    "formatDxLaunchApprovalIssues",
    '"schema.refine"',
    '"schema.superRefine"',
    '"schema.check"',
    '"z.refine"',
    '"ctx.addIssue"',
    '"lib/validation/zod/refinements.ts"',
    'refinementHelper: "safeParseDxLaunchApprovalGate(value)"',
  ]) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing ${marker}`,
    );
  }
});

test("launch template and CLI consume Zod refinement gate", () => {
  const status = read("examples/template/zod-validation-status.tsx");
  const cli = read("dx-www/src/cli/mod.rs");

  for (const marker of [
    "@/lib/validation/zod/refinements",
    "safeParseDxLaunchApprovalGate",
    "formatDxLaunchApprovalIssues",
    "data-dx-zod-refinement-status",
    "approvalGate",
  ]) {
    assert.match(
      status,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing launch marker ${marker}`,
    );
  }

  for (const marker of [
    "@/lib/validation/zod/refinements",
    "safeParseDxLaunchApprovalGate",
    "formatDxLaunchApprovalIssues",
  ]) {
    assert.match(
      cli,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `missing CLI marker ${marker}`,
    );
  }
});
