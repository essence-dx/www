import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const cliCoreImplPath = path.join(repoRoot, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");
const previewCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "preview_command.rs");
const previewOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "preview_options.rs");

test("dx preview command serving and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const cliCoreImpl = fs.readFileSync(cliCoreImplPath, "utf8");
  assert.ok(
    fs.existsSync(previewCommandPath),
    "expected dx-www/src/cli/preview_command.rs",
  );
  assert.ok(
    fs.existsSync(previewOptionsPath),
    "expected dx-www/src/cli/preview_options.rs",
  );

  const previewCommand = fs.readFileSync(previewCommandPath, "utf8");
  const previewOptions = fs.readFileSync(previewOptionsPath, "utf8");
  const cmdPreviewBlock = cliCoreImpl.match(
    /pub fn cmd_preview\(&self, args: &\[String\]\) -> DxResult<\(\)> \{[\s\S]*?\n    \}/,
  )?.[0] ?? "";

  assert.match(cliMod, /^mod preview_command;$/m);
  assert.match(cliMod, /^mod preview_options;$/m);
  assert.match(
    cliCoreImpl,
    /preview_command::cmd_preview\(\s*&self\.cwd,\s*args,\s*server_action_runtime::execute_production_contract_server_action,\s*\)/,
  );

  assert.doesNotMatch(cliMod, /let mut production_contract = false/);
  assert.doesNotMatch(cliMod, /Unknown preview option/);
  assert.doesNotMatch(cliMod, /dx preview currently requires --production-contract/);
  assert.doesNotMatch(cliMod, /\bfn production_contract_wire_response\b/);
  assert.doesNotMatch(cmdPreviewBlock, /TcpListener::bind\(&addr\)/);
  assert.doesNotMatch(cmdPreviewBlock, /let mut buffer = \[0; 8192\]/);

  assert.match(previewCommand, /pub\(super\) fn cmd_preview\(/);
  assert.match(previewCommand, /execute_server_action: DxProductionServerActionExecutor/);
  assert.match(previewCommand, /production_contract_wire_response_cached_with_connection/);
  assert.match(previewCommand, /preview_request_wants_keep_alive/);
  assert.match(previewCommand, /parse_preview_options\(cwd, args\)\?/);
  assert.match(previewCommand, /TcpListener::bind\(&addr\)/);
  assert.match(previewCommand, /stream\.write_all\(&response_bytes\)/);

  assert.match(previewOptions, /pub\(super\) struct DxPreviewCommandOptions/);
  assert.match(previewOptions, /pub\(super\) fn parse_preview_options\(/);
  assert.match(previewOptions, /Unknown preview option/);
  assert.match(previewOptions, /dx preview currently requires --production-contract/);
  assert.match(previewOptions, /mod tests/);
});
