import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeTrustRegressionOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_trust_regression_options.rs",
);
const forgeTrustRegressionCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_trust_regression_command.rs",
);

test("forge trust-regression option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeTrustRegressionOptionsPath),
    "expected dx-www/src/cli/forge_trust_regression_options.rs",
  );

  const forgeTrustRegressionOptions = fs.readFileSync(
    forgeTrustRegressionOptionsPath,
    "utf8",
  );
  const forgeTrustRegressionCommand = fs.readFileSync(
    forgeTrustRegressionCommandPath,
    "utf8",
  );

  assert.match(cliMod, /^mod forge_trust_regression_options;$/m);
  assert.match(
    cliMod,
    /^mod forge_trust_regression_command;$/m,
  );
  assert.match(
    cliMod,
    /"trust-regression"\s*=>\s*\{\s*forge_trust_regression_command::run_forge_trust_regression\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_trust_regression("),
    false,
    "trust-regression command body should be owned by forge_trust_regression_command.rs",
  );
  assert.match(
    forgeTrustRegressionCommand,
    /use (?:self|super)::forge_trust_regression_options::\{\s*DxForgeTrustRegressionCommandOptions,\s*parse_forge_trust_regression_options,?\s*\};/s,
  );

  assert.doesNotMatch(
    forgeTrustRegressionCommand,
    /let mut project: Option<PathBuf> = None/,
  );
  assert.doesNotMatch(forgeTrustRegressionCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(
    forgeTrustRegressionCommand,
    /Unknown forge trust-regression option/,
  );
  assert.doesNotMatch(
    forgeTrustRegressionCommand,
    /Unexpected forge trust-regression path/,
  );

  assert.match(
    forgeTrustRegressionOptions,
    /pub\(super\) struct DxForgeTrustRegressionCommandOptions/,
  );
  assert.match(
    forgeTrustRegressionOptions,
    /pub\(super\) fn parse_forge_trust_regression_options\(/,
  );
  assert.match(
    forgeTrustRegressionOptions,
    /Unknown forge trust-regression option/,
  );
  assert.match(
    forgeTrustRegressionOptions,
    /Unexpected forge trust-regression path/,
  );
  assert.match(forgeTrustRegressionOptions, /mod tests/);
});
