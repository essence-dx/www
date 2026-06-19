import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const formattingPath = path.join(repoRoot, "dx-www", "src", "cli", "formatting.rs");

test("shared CLI text formatting helpers live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(fs.existsSync(formattingPath), "expected dx-www/src/cli/formatting.rs");

  const formatting = fs.readFileSync(formattingPath, "utf8");

  assert.match(cliMod, /^mod formatting;$/m);
  assert.match(
    cliMod,
    /use self::formatting::\{\s*count_substrings,\s*html_href_values,\s*markdown_table_cell,\s*optional_f64,\s*optional_string,\s*optional_u64,?\s*\};/s,
  );

  for (const helper of [
    "markdown_table_cell",
    "optional_string",
    "optional_u64",
    "optional_f64",
    "count_substrings",
    "html_href_values",
  ]) {
    assert.doesNotMatch(cliMod, new RegExp(`^fn ${helper}\\(`, "m"));
    assert.match(formatting, new RegExp(`pub\\(super\\) fn ${helper}\\(`));
  }

  assert.match(formatting, /mod tests/);
});
