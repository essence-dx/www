import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const read = (relative: string) => fs.readFileSync(path.join(root, relative), "utf8");

test("dx-style receipts report generated CSS retention and pruning evidence", () => {
  const publicTools = read("dx-www/src/cli/public_framework_tools.rs");
  const tools = read("dx-www/src/cli/public_framework_tools/dx_style.rs");

  assert.match(publicTools, /mod dx_style;/);

  for (const marker of [
    "fn style_pruning_report(",
    "\"schema\": \"dx.style.generated_css_pruning\"",
    "\"mode\": \"static-token-retention\"",
    "\"generated_css_bytes\"",
    "\"retained_generated_class_count\"",
    "\"pruned_candidate_class_count\"",
    "\"estimated_pruned_css_bytes\"",
    "(\"generated_css_bytes\", sr_number(generated.len()))",
    "pruned_candidate_class_count",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
