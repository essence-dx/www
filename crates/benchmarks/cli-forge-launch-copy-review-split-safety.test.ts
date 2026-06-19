import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const launchCopyReviewPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_launch_copy_review.rs",
);

test("Forge launch copy review command wrapper lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const launchCopyReview = fs.readFileSync(launchCopyReviewPath, "utf8");
  const commandStart = cliMod.indexOf("fn cmd_forge_launch_copy_review");
  const nextCommandStart = cliMod.indexOf("fn cmd_forge_launch_page", commandStart);
  assert.notEqual(commandStart, -1, "expected launch copy review command in cli module");
  assert.notEqual(nextCommandStart, -1, "expected launch page command after copy review");
  const commandBlock = cliMod.slice(commandStart, nextCommandStart);

  assert.match(cliMod, /^mod forge_launch_copy_review;$/m);
  assert.match(
    cliMod,
    /use forge_launch_copy_review::cmd_forge_launch_copy_review as run_forge_launch_copy_review_command;/,
  );
  assert.match(commandBlock, /run_forge_launch_copy_review_command\(&self\.cwd, args\)/);

  assert.doesNotMatch(commandBlock, /let mut copy_paths = Vec::new\(\)/);
  assert.doesNotMatch(commandBlock, /Unknown forge launch-copy-review option/);
  assert.doesNotMatch(commandBlock, /Unexpected forge launch-copy-review path/);
  assert.doesNotMatch(commandBlock, /std::fs::write\(&output, &rendered\)/);
  assert.doesNotMatch(commandBlock, /DX Forge launch-copy-review score/);
  assert.doesNotMatch(commandBlock, /forge_launch_copy_review_failure_summary\(&report\)/);

  assert.match(launchCopyReview, /pub\(super\) fn cmd_forge_launch_copy_review\(/);
  assert.match(launchCopyReview, /parse_score_threshold\(value\)\?/);
  assert.match(launchCopyReview, /Unknown forge launch-copy-review option/);
  assert.match(launchCopyReview, /Unexpected forge launch-copy-review path/);
  assert.match(launchCopyReview, /DX Forge launch-copy-review score/);
  assert.match(launchCopyReview, /forge_launch_copy_review_failure_summary\(&report\)/);
});
