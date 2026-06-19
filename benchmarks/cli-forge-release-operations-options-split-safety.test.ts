import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeReleaseOperationsOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_release_operations_options.rs",
);

test("forge release-operations option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeReleaseOperationsOptionsPath),
    "expected dx-www/src/cli/forge_release_operations_options.rs",
  );

  const forgeReleaseOperationsOptions = fs.readFileSync(
    forgeReleaseOperationsOptionsPath,
    "utf8",
  );
  const releaseOperationsStart = cliMod.indexOf("fn cmd_forge_release_operations");
  const publishPlanStart = cliMod.indexOf("fn cmd_forge_publish_plan", releaseOperationsStart);

  assert.notEqual(
    releaseOperationsStart,
    -1,
    "expected cmd_forge_release_operations in cli module",
  );
  assert.notEqual(
    publishPlanStart,
    -1,
    "expected forge publish-plan command after release-operations",
  );

  const releaseOperationsCommand = cliMod.slice(
    releaseOperationsStart,
    publishPlanStart,
  );

  assert.match(cliMod, /^mod forge_release_operations_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_release_operations_options::\{\s*parse_forge_release_operations_options,\s*DxForgeReleaseOperationsCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(
    releaseOperationsCommand,
    /let mut project: Option<PathBuf> = None/,
  );
  assert.doesNotMatch(releaseOperationsCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(
    releaseOperationsCommand,
    /Unknown forge release-operations option/,
  );
  assert.doesNotMatch(
    releaseOperationsCommand,
    /Unexpected forge release-operations path/,
  );

  assert.match(
    forgeReleaseOperationsOptions,
    /pub\(super\) struct DxForgeReleaseOperationsCommandOptions/,
  );
  assert.match(
    forgeReleaseOperationsOptions,
    /pub\(super\) fn parse_forge_release_operations_options\(/,
  );
  assert.match(
    forgeReleaseOperationsOptions,
    /Unknown forge release-operations option/,
  );
  assert.match(
    forgeReleaseOperationsOptions,
    /Unexpected forge release-operations path/,
  );
  assert.match(forgeReleaseOperationsOptions, /mod tests/);
});
