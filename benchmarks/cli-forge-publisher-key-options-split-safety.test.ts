import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgePublisherKeyCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_publisher_key_command.rs",
);
const forgePublisherKeyOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_publisher_key_options.rs",
);

test("forge publisher-key command and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgePublisherKeyCommandPath),
    "expected dx-www/src/cli/forge_publisher_key_command.rs",
  );
  assert.ok(
    fs.existsSync(forgePublisherKeyOptionsPath),
    "expected dx-www/src/cli/forge_publisher_key_options.rs",
  );

  const forgePublisherKeyCommand = fs.readFileSync(
    forgePublisherKeyCommandPath,
    "utf8",
  );
  const forgePublisherKeyOptions = fs.readFileSync(
    forgePublisherKeyOptionsPath,
    "utf8",
  );

  assert.match(cliMod, /^mod forge_publisher_key_command;$/m);
  assert.match(cliMod, /^mod forge_publisher_key_options;$/m);
  assert.match(
    cliMod,
    /"publisher-key"\s*\|\s*"publisher"\s*=>\s*\{\s*forge_publisher_key_command::run_forge_publisher_key\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/s,
  );

  assert.doesNotMatch(
    cliMod,
    /fn cmd_forge_publisher_key/,
    "cli module should not own publisher-key command execution",
  );
  assert.doesNotMatch(
    cliMod,
    /parse_forge_publisher_key_(generate|sign)_options/,
    "cli module should not parse publisher-key options inline",
  );
  assert.doesNotMatch(
    cliMod,
    /dx forge publisher-key sign requires --key <private-key\.json>/,
    "cli module should not own publisher-key sign diagnostics",
  );

  assert.match(
    forgePublisherKeyCommand,
    /pub\(super\) fn run_forge_publisher_key\(/,
  );
  assert.match(
    forgePublisherKeyCommand,
    /run_forge_publisher_key_generate\(cwd, &args\[1\.\.\]\)/,
  );
  assert.match(
    forgePublisherKeyCommand,
    /run_forge_publisher_key_sign\(cwd, &args\[1\.\.\]\)/,
  );
  assert.match(
    forgePublisherKeyCommand,
    /parse_forge_publisher_key_generate_options\(cwd, args\)/,
  );
  assert.match(
    forgePublisherKeyCommand,
    /parse_forge_publisher_key_sign_options\(cwd, args\)/,
  );
  assert.match(
    forgePublisherKeyCommand,
    /dx forge publisher-key sign requires --key <private-key\.json>/,
  );
  assert.match(forgePublisherKeyCommand, /generate_forge_publisher_key/);
  assert.match(
    forgePublisherKeyCommand,
    /sign_forge_release_manifest_with_publisher_key/,
  );

  assert.match(
    forgePublisherKeyOptions,
    /pub\(super\) struct DxForgePublisherKeyGenerateCommandOptions/,
  );
  assert.match(
    forgePublisherKeyOptions,
    /pub\(super\) struct DxForgePublisherKeySignCommandOptions/,
  );
  assert.match(
    forgePublisherKeyOptions,
    /pub\(super\) fn parse_forge_publisher_key_generate_options\(/,
  );
  assert.match(
    forgePublisherKeyOptions,
    /pub\(super\) fn parse_forge_publisher_key_sign_options\(/,
  );
  assert.match(
    forgePublisherKeyOptions,
    /Unknown forge publisher-key generate option/,
  );
  assert.match(
    forgePublisherKeyOptions,
    /Unexpected forge publisher-key generate path/,
  );
  assert.match(forgePublisherKeyOptions, /Unknown forge publisher-key sign option/);
  assert.match(forgePublisherKeyOptions, /Unexpected forge publisher-key sign path/);
  assert.match(forgePublisherKeyOptions, /mod tests/);
});
