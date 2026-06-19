import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const addArgsPath = path.join(repoRoot, "dx-www", "src", "cli", "add_args.rs");

test("dx add argument routing helpers live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(fs.existsSync(addArgsPath), "expected dx-www/src/cli/add_args.rs");

  const addArgs = fs.readFileSync(addArgsPath, "utf8");

  assert.match(cliMod, /^mod add_args;$/m);
  assert.match(
    cliMod,
    /use self::add_args::\{\s*first_dx_add_subject,\s*is_source_owned_add_candidate,?\s*\};/s,
  );

  assert.doesNotMatch(cliMod, /^fn first_dx_add_subject/m);
  assert.doesNotMatch(cliMod, /^fn is_source_owned_add_candidate/m);

  assert.match(addArgs, /pub\(super\) fn first_dx_add_subject/);
  assert.match(addArgs, /pub\(super\) fn is_source_owned_add_candidate/);
  assert.match(addArgs, /mod tests/);
});
