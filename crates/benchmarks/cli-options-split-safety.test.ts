import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const optionsPath = path.join(repoRoot, "dx-www", "src", "cli", "options.rs");

test("shared CLI option parsing helpers live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(fs.existsSync(optionsPath), "expected dx-www/src/cli/options.rs");

  const options = fs.readFileSync(optionsPath, "utf8");

  assert.match(cliMod, /^mod options;$/m);
  assert.match(
    cliMod,
    /use self::options::\{\s*parse_score_threshold,\s*resolve_cli_path,\s*DxOutputFormat,?\s*\};/s,
  );

  assert.doesNotMatch(cliMod, /^pub\(super\) enum DxOutputFormat/m);
  assert.doesNotMatch(cliMod, /^impl DxOutputFormat/m);
  assert.doesNotMatch(cliMod, /^pub\(super\) fn resolve_cli_path\(/m);
  assert.doesNotMatch(cliMod, /^fn parse_score_threshold\(/m);

  assert.match(options, /pub\(super\) enum DxOutputFormat/);
  assert.match(options, /impl DxOutputFormat/);
  assert.match(options, /pub\(super\) fn resolve_cli_path\(/);
  assert.match(options, /pub\(super\) fn parse_score_threshold\(/);
  assert.match(options, /mod tests/);
});
