import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeTrustPolicyOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_trust_policy_options.rs",
);
const forgeTrustPolicyCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_trust_policy_command.rs",
);

test("forge trust-policy option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeTrustPolicyOptionsPath),
    "expected dx-www/src/cli/forge_trust_policy_options.rs",
  );

  const forgeTrustPolicyOptions = fs.readFileSync(
    forgeTrustPolicyOptionsPath,
    "utf8",
  );
  const forgeTrustPolicyCommand = fs.readFileSync(
    forgeTrustPolicyCommandPath,
    "utf8",
  );

  assert.match(cliMod, /^mod forge_trust_policy_options;$/m);
  assert.match(
    cliMod,
    /^mod forge_trust_policy_command;$/m,
  );
  assert.match(
    cliMod,
    /"trust-policy"\s*=>\s*\{\s*forge_trust_policy_command::run_forge_trust_policy\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_trust_policy("),
    false,
    "trust-policy command body should be owned by forge_trust_policy_command.rs",
  );
  assert.match(
    forgeTrustPolicyCommand,
    /use (?:self|super)::forge_trust_policy_options::\{\s*DxForgeTrustPolicyCommandOptions,\s*parse_forge_trust_policy_options,?\s*\};/s,
  );

  assert.doesNotMatch(
    forgeTrustPolicyCommand,
    /let mut project: Option<PathBuf> = None/,
  );
  assert.doesNotMatch(forgeTrustPolicyCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(
    forgeTrustPolicyCommand,
    /Unknown forge trust-policy option/,
  );
  assert.doesNotMatch(
    forgeTrustPolicyCommand,
    /Unexpected forge trust-policy path/,
  );

  assert.match(
    forgeTrustPolicyOptions,
    /pub\(super\) struct DxForgeTrustPolicyCommandOptions/,
  );
  assert.match(
    forgeTrustPolicyOptions,
    /pub\(super\) fn parse_forge_trust_policy_options\(/,
  );
  assert.match(forgeTrustPolicyOptions, /Unknown forge trust-policy option/);
  assert.match(forgeTrustPolicyOptions, /Unexpected forge trust-policy path/);
  assert.match(forgeTrustPolicyOptions, /mod tests/);
});
