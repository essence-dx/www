import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeReleaseReviewOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_release_review_options.rs",
);

test("forge release-review option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeReleaseReviewOptionsPath),
    "expected dx-www/src/cli/forge_release_review_options.rs",
  );

  const forgeReleaseReviewOptions = fs.readFileSync(
    forgeReleaseReviewOptionsPath,
    "utf8",
  );
  const releaseReviewStart = cliMod.indexOf("fn cmd_forge_release_review");
  const initAppStart = cliMod.indexOf("fn cmd_forge_init_app", releaseReviewStart);

  assert.notEqual(
    releaseReviewStart,
    -1,
    "expected cmd_forge_release_review in cli module",
  );
  assert.notEqual(
    initAppStart,
    -1,
    "expected forge init-app command after release-review",
  );

  const releaseReviewCommand = cliMod.slice(releaseReviewStart, initAppStart);

  assert.match(cliMod, /^mod forge_release_review_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_release_review_options::\{\s*parse_forge_release_review_options,\s*DxForgeReleaseReviewCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(
    releaseReviewCommand,
    /let mut project: Option<PathBuf> = None/,
  );
  assert.doesNotMatch(releaseReviewCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(
    releaseReviewCommand,
    /Unknown forge release-review option/,
  );
  assert.doesNotMatch(
    releaseReviewCommand,
    /Unexpected forge release-review path/,
  );

  assert.match(
    forgeReleaseReviewOptions,
    /pub\(super\) struct DxForgeReleaseReviewCommandOptions/,
  );
  assert.match(
    forgeReleaseReviewOptions,
    /pub\(super\) fn parse_forge_release_review_options\(/,
  );
  assert.match(
    forgeReleaseReviewOptions,
    /Unknown forge release-review option/,
  );
  assert.match(
    forgeReleaseReviewOptions,
    /Unexpected forge release-review path/,
  );
  assert.match(forgeReleaseReviewOptions, /mod tests/);
});
