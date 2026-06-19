import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const read = (relative: string) => fs.readFileSync(path.join(root, relative), "utf8");

test("web-perf receipts are split between dev and static-build modes", () => {
  const tools = read("dx-www/src/cli/public_framework_tools.rs");
  const help = read("dx-www/src/cli/help_text.rs");

  for (const marker of [
    "--receipt-mode",
    "\"receipt_mode\"",
    "\"mode_receipt_path\"",
    ".dx/receipts/check/web-perf/dev/report.json",
    ".dx/receipts/check/web-perf/static-build/report.json",
    "attach_web_perf_receipt_mode(project, &mut report, &receipt_mode)?",
    "web_perf_mode_report_path(&report)",
    "web_perf_mode_cdp_plan_path(&report)",
    "\"preferred_mode\"",
    "\"available_modes\"",
  ]) {
    assert.match(`${tools}\n${help}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
