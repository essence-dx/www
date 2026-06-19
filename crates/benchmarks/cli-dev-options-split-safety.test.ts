import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const devCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "dev_command.rs");
const devOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "dev_options.rs");

function sliceRustMethod(source: string, signature: string): string {
  const start = source.indexOf(signature);
  assert.notEqual(start, -1, `expected to find ${signature}`);
  const nextMethod = source.indexOf("\n    pub fn ", start + signature.length);
  return source.slice(start, nextMethod === -1 ? undefined : nextMethod);
}

test("dx dev option parsing and listener binding live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const devCommand = fs.readFileSync(devCommandPath, "utf8");
  assert.ok(fs.existsSync(devOptionsPath), "expected dx-www/src/cli/dev_options.rs");

  const devOptions = fs.readFileSync(devOptionsPath, "utf8");
  const cmdDev = sliceRustMethod(cliMod, "pub fn cmd_dev(&self, args: &[String]) -> DxResult<()>");

  assert.match(cliMod, /^mod dev_options;$/m);
  assert.match(cliMod, /^mod dev_command;$/m);
  assert.match(
    cmdDev,
    /dev_command::cmd_dev\(\s*&self\.cwd,\s*args,\s*\|\| self\.load_translations\(\),\s*Self::handle_parsed_http_response,\s*\)/,
  );

  assert.doesNotMatch(cliMod, /^struct DxDevCommandOptions/m);
  assert.doesNotMatch(cliMod, /fn parse_dev_options\(/);
  assert.doesNotMatch(cliMod, /fn bind_dev_listener\(/);
  assert.doesNotMatch(cmdDev, /parse_dev_options|bind_dev_listener|DxDevCommandOptions/);

  const devOptionsUse = devCommand.match(/^use super::dev_options::\{([^}]+)\};$/m);
  assert.ok(devOptionsUse, "dx dev command should import dev option helpers from dev_options");
  for (const importedName of ["bind_dev_listener", "parse_dev_options", "DxDevServerBinding"]) {
    assert.match(devOptionsUse[1], new RegExp(`\\b${importedName}\\b`));
  }
  assert.match(devCommand, /let options = parse_dev_options\(args, &config\)\?/);
  assert.match(
    devCommand,
    /match bind_dev_listener\(&options\.host, options\.port, options\.port_explicit\)\?/,
  );
  assert.ok(
    devCommand.indexOf("parse_dev_options(args, &config)?") <
      devCommand.indexOf("bind_dev_listener(&options.host, options.port, options.port_explicit)?"),
    "dx dev must parse user options before binding the listener",
  );
  assert.match(devCommand, /DxDevServerBinding::Existing\(existing\)/);

  assert.match(devOptions, /pub\(super\) struct DxDevCommandOptions/);
  assert.match(devOptions, /pub\(super\) fn parse_dev_options\(/);
  assert.match(devOptions, /pub\(super\) fn bind_dev_listener\(/);
  assert.match(devOptions, /let host = host\.trim\(\);/);
  assert.match(devOptions, /fn parse_dev_options_trims_host_before_normalizing_localhost\(\)/);
  assert.match(devOptions, /fn bind_dev_listener_rejects_busy_explicit_port_without_fallback\(\)/);
  assert.match(devOptions, /mod tests/);
});
