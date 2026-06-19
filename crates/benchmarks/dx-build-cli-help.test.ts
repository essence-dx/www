const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const cliSourcePath = path.join(
  __dirname,
  "..",
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_core_impl.rs",
);
const cliModPath = path.join(__dirname, "..", "dx-www", "src", "cli", "mod.rs");
const buildCommandPath = path.join(
  __dirname,
  "..",
  "dx-www",
  "src",
  "cli",
  "build_command.rs",
);
const helpTextPath = path.join(__dirname, "..", "dx-www", "src", "cli", "help_text.rs");

test("dx build help is handled before production build execution", () => {
  const source = fs.readFileSync(cliSourcePath, "utf8");
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const buildCommand = fs.existsSync(buildCommandPath)
    ? fs.readFileSync(buildCommandPath, "utf8")
    : "";
  const helpText = fs.readFileSync(helpTextPath, "utf8");
  const buildDispatch = source.match(/"build" => \{([\s\S]*?)\n            \}/);

  assert.ok(buildDispatch, "dx build dispatch branch is missing");
  assert.match(cliMod, /^mod build_command;$/m, "dx build dispatch should live in a focused module");
  assert.match(
    buildDispatch[1],
    /run_build_command\(&args\[2\.\.\], "dx build", \|options\| \{[\s\S]*cli\.cmd_build_with_options\(options, "dx build"\)/,
    "dx build dispatch should delegate help/options ordering to build_command",
  );
  assert.match(
    buildCommand,
    /pub\(super\) fn run_build_command<F>\([\s\S]*command_name: &'static str[\s\S]*if is_help_arg\(args\.first\(\)\)[\s\S]*print_build_help\(command_name\);[\s\S]*return Ok\(\(\)\);[\s\S]*parse_build_options\(args, command_name\)[\s\S]*build\(options\)/,
    "dx build --help must print help before cmd_build can run",
  );
  assert.match(
    helpText,
    /pub\(super\) fn print_build_help\(command_name: &str\)[\s\S]*USAGE:[\s\S]*\{command_name\}[\s\S]*\{command_name\} --target android[\s\S]*--target <target>[\s\S]*does not install node_modules[\s\S]*DX Native\/Tauri arm64 debug APK/,
    "dx build help should describe the source-owned no-node_modules contract",
  );
});
