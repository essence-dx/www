import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeCiSnippetsOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_ci_snippets_options.rs",
);

test("forge ci-snippets option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeCiSnippetsOptionsPath),
    "expected dx-www/src/cli/forge_ci_snippets_options.rs",
  );

  const forgeCiSnippetsOptions = fs.readFileSync(forgeCiSnippetsOptionsPath, "utf8");
  const ciSnippetsStart = cliMod.indexOf("fn cmd_forge_ci_snippets");
  const auditStart = cliMod.indexOf("fn cmd_forge_audit", ciSnippetsStart);

  assert.notEqual(
    ciSnippetsStart,
    -1,
    "expected cmd_forge_ci_snippets in cli module",
  );
  assert.notEqual(auditStart, -1, "expected forge audit after ci-snippets");

  const command = cliMod.slice(ciSnippetsStart, auditStart);

  assert.match(cliMod, /^mod forge_ci_snippets_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_ci_snippets_options::\{\s*parse_forge_ci_snippets_options,\s*DxForgeCiSnippetsCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(command, /let mut out: Option<PathBuf> = None/);
  assert.doesNotMatch(command, /while index < args\.len\(\)/);
  assert.doesNotMatch(command, /Unknown forge ci-snippets option/);
  assert.doesNotMatch(command, /Unexpected forge ci-snippets path/);

  assert.match(
    forgeCiSnippetsOptions,
    /pub\(super\) struct DxForgeCiSnippetsCommandOptions/,
  );
  assert.match(
    forgeCiSnippetsOptions,
    /pub\(super\) fn parse_forge_ci_snippets_options\(/,
  );
  assert.match(forgeCiSnippetsOptions, /Unknown forge ci-snippets option/);
  assert.match(forgeCiSnippetsOptions, /Unexpected forge ci-snippets path/);
  assert.match(forgeCiSnippetsOptions, /mod tests/);
});
