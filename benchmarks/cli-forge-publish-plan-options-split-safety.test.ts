import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgePublishPlanOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_publish_plan_options.rs",
);

test("forge publish-plan option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgePublishPlanOptionsPath),
    "expected dx-www/src/cli/forge_publish_plan_options.rs",
  );

  const forgePublishPlanOptions = fs.readFileSync(
    forgePublishPlanOptionsPath,
    "utf8",
  );
  const publishPlanStart = cliMod.indexOf("fn cmd_forge_publish_plan");
  const releaseReviewStart = cliMod.indexOf(
    "fn cmd_forge_release_review",
    publishPlanStart,
  );

  assert.notEqual(
    publishPlanStart,
    -1,
    "expected cmd_forge_publish_plan in cli module",
  );
  assert.notEqual(
    releaseReviewStart,
    -1,
    "expected forge release-review command after publish-plan",
  );

  const publishPlanCommand = cliMod.slice(publishPlanStart, releaseReviewStart);

  assert.match(cliMod, /^mod forge_publish_plan_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_publish_plan_options::\{\s*parse_forge_publish_plan_options,\s*DxForgePublishPlanCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(
    publishPlanCommand,
    /let mut project: Option<PathBuf> = None/,
  );
  assert.doesNotMatch(publishPlanCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(
    publishPlanCommand,
    /Unknown forge publish-plan option/,
  );
  assert.doesNotMatch(
    publishPlanCommand,
    /Unexpected forge publish-plan path/,
  );

  assert.match(
    forgePublishPlanOptions,
    /pub\(super\) struct DxForgePublishPlanCommandOptions/,
  );
  assert.match(
    forgePublishPlanOptions,
    /pub\(super\) fn parse_forge_publish_plan_options\(/,
  );
  assert.match(forgePublishPlanOptions, /Unknown forge publish-plan option/);
  assert.match(forgePublishPlanOptions, /Unexpected forge publish-plan path/);
  assert.match(forgePublishPlanOptions, /mod tests/);
});
