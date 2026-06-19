import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const forgeCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);

const forgeCommands = readFileSync(forgeCommandsPath, "utf8");
const forgeAcquireCommands = readFileSync(
  join(repoRoot, "dx-www", "src", "cli", "mod_parts", "cli_forge_commands_a.rs"),
  "utf8",
);

for (const command of [
  "cmd_forge_add",
  "cmd_forge_public_publish",
  "cmd_forge_remote_head",
  "cmd_forge_remove",
]) {
  test(`${command} exposes help before option parsing`, () => {
    const body = functionBody(command);

    assert.match(body, /if forge_help_requested\(args\)/);
    assert.match(body, /return Ok\(\(\)\);/);
  });
}

test("forge package lifecycle help documents safe modes", () => {
  assert.match(
    forgeCommands,
    /Usage: dx forge add <package> .*--dry-run\|--write/,
  );
  assert.match(
    forgeCommands,
    /dx forge add npm three --json/,
  );
  assert.match(
    forgeCommands,
    /alias for dx forge acquire npm <package>/,
  );
  assert.match(
    forgeCommands,
    /Usage: dx forge publish .*--dry-run\|--write/,
  );
  assert.match(
    forgeCommands,
    /Publish requires an explicit --dry-run or --write mode/,
  );
  assert.match(
    forgeCommands,
    /Usage: dx forge remote-head <package\[#exports\]> --registry r2 --remote-manifest <manifest\.json>/,
  );
  assert.match(
    forgeCommands,
    /planned mode performs no network request/,
  );
  assert.match(
    forgeCommands,
    /Usage: dx forge remove <package> .*--dry-run\|--write/,
  );
});

test("forge acquire help documents npm acquisition without installs", () => {
  assert.match(forgeAcquireCommands, /Usage: dx forge acquire <\{\}> <package>/);
  assert.match(forgeAcquireCommands, /dx forge acquire npm three --json/);
  assert.match(forgeAcquireCommands, /compatibility alias: dx forge add npm three --json/);
});

function functionBody(name: string): string {
  const start = forgeCommands.indexOf(`fn ${name}`);
  assert.notEqual(start, -1, `missing ${name}`);

  const next = forgeCommands.indexOf("\n    fn ", start + 1);
  return forgeCommands.slice(start, next === -1 ? undefined : next);
}
