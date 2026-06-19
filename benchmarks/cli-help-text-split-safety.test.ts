import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath: string): string {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

function importedHelpNames(source: string, namespace: "self" | "super"): string[] {
  const importMatch = source.match(
    new RegExp(`use ${namespace}::help_text::\\{([\\s\\S]*?)\\};`),
  );
  assert.ok(importMatch, `expected ${namespace}::help_text helper import`);
  return importMatch[1]
    .split(",")
    .map((name) => name.trim())
    .filter(Boolean);
}

test("CLI help text and help arg detection live outside the giant cli module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const buildCommand = readRepoFile("dx-www/src/cli/build_command.rs");
  const devCommand = readRepoFile("dx-www/src/cli/dev_command.rs");
  const helpTextPath = resolve(repoRoot, "dx-www/src/cli/help_text.rs");
  assert.ok(existsSync(helpTextPath), "expected dx-www/src/cli/help_text.rs");

  const helpText = readFileSync(helpTextPath, "utf8");

  assert.match(cliMod, /^mod help_text;$/m);
  const cliHelpImports = importedHelpNames(cliMod, "self");
  for (const importedName of [
    "is_help_arg",
    "print_check_help",
    "print_check_web_perf_help",
    "print_dev_help",
    "print_forge_help",
    "print_help",
    "print_new_help",
    "print_serializer_help",
  ]) {
    assert.ok(cliHelpImports.includes(importedName), `cli module should import ${importedName}`);
  }
  assert.equal(
    cliHelpImports.includes("print_build_help"),
    false,
    "build help should be imported by build_command.rs, not the giant cli module",
  );

  const buildHelpImports = importedHelpNames(buildCommand, "super");
  for (const importedName of ["is_help_arg", "print_build_help"]) {
    assert.ok(
      buildHelpImports.includes(importedName),
      `build_command.rs should import ${importedName}`,
    );
  }
  assert.match(
    buildCommand,
    /if is_help_arg\(args\.first\(\)\) \{[\s\S]*print_build_help\(command_name\);[\s\S]*return Ok\(\(\)\);[\s\S]*\}/,
    "dx build help should be handled inside build_command before build execution",
  );

  const devHelpImports = importedHelpNames(devCommand, "super");
  for (const importedName of ["is_help_arg", "print_dev_help"]) {
    assert.ok(devHelpImports.includes(importedName), `dev_command.rs should import ${importedName}`);
  }
  assert.match(
    devCommand,
    /args\.iter\(\)\.any\(\|arg\| is_help_arg\(Some\(arg\)\)\)/,
    "dx dev help should reuse shared help-argument detection",
  );
  assert.doesNotMatch(
    devCommand,
    /matches!\(arg\.as_str\(\), "--help" \| "-h" \| "help"\)/,
    "dx dev should not carry a stale inline help matcher",
  );

  for (const oldDefinition of [
    "fn print_help(",
    "fn print_new_help(",
    "fn print_dev_help(",
    "fn print_build_help(",
    "fn print_check_help(",
    "fn print_check_web_perf_help(",
    "fn print_serializer_help(",
    "fn print_forge_help(",
    "fn is_help_arg(",
  ]) {
    assert.equal(
      cliMod.includes(oldDefinition),
      false,
      `${oldDefinition} should be owned by help_text.rs`,
    );
  }

  for (const exportedHelper of [
    "pub(super) fn print_help()",
    "pub(super) fn print_new_help()",
    "pub(super) fn print_dev_help()",
    "pub(super) fn print_build_help(command_name: &str)",
    "pub(super) fn print_check_help()",
    "pub(super) fn print_check_web_perf_help()",
    "pub(super) fn print_serializer_help()",
    "pub(super) fn print_forge_help()",
    "pub(super) fn is_help_arg(arg: Option<&String>) -> bool",
  ]) {
    assert.ok(helpText.includes(exportedHelper), `${exportedHelper} should be exported to cli`);
  }

  for (const commandHelpNeedle of [
    "dx-www: React-familiar Forge-first web framework",
    "dx new <name>",
    "dx dev --host 127.0.0.1 --port 3000 --no-hot-reload",
    "{command_name}: Run the DX source-owned build engine",
    "dx check: Score DX-WWW project readiness",
    "dx check web-perf: Measure WWW performance evidence",
    "dx serializer: Generate DX Serializer machine cache files",
    "dx forge: source-owned package firewall",
    'matches!(arg.as_str(), "--help" | "-h" | "help")',
  ]) {
    assert.ok(
      helpText.includes(commandHelpNeedle),
      `help_text.rs should preserve: ${commandHelpNeedle}`,
    );
  }
});
