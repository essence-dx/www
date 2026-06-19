import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandOutputPath = path.join(repoRoot, "dx-www", "src", "cli", "command_output.rs");
const templateCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "templates_command.rs");
const templateOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "template_options.rs");

test("template command execution and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(templateCommandPath),
    "expected dx-www/src/cli/templates_command.rs",
  );
  assert.ok(
    fs.existsSync(commandOutputPath),
    "expected dx-www/src/cli/command_output.rs",
  );
  assert.ok(
    fs.existsSync(templateOptionsPath),
    "expected dx-www/src/cli/template_options.rs",
  );

  const templateCommand = fs.readFileSync(templateCommandPath, "utf8");
  const templateOptions = fs.readFileSync(templateOptionsPath, "utf8");
  const cmdTemplatesBlock =
    cliMod.match(
      /fn cmd_templates\(&self, args: &\[String\]\) -> DxResult<\(\)> \{[\s\S]*?\n    \}/,
    )?.[0] ?? "";

  assert.match(cliMod, /^mod command_output;$/m);
  assert.match(cliMod, /^mod template_options;$/m);
  assert.match(cliMod, /^mod templates_command;$/m);
  assert.match(cmdTemplatesBlock, /templates_command::cmd_templates\(&self\.cwd, args\)/);
  assert.doesNotMatch(cliMod, /\bfn cmd_templates_verify_readiness\b/);
  assert.doesNotMatch(cliMod, /\bfn cmd_templates_legacy_catalog\b/);
  assert.doesNotMatch(cliMod, /use self::template_options::/);

  assert.match(templateCommand, /pub\(super\) fn cmd_templates\(cwd: &Path, args: &\[String\]\)/);
  assert.match(templateCommand, /pub\(super\) fn cmd_templates_verify_readiness\(/);
  assert.match(templateCommand, /fn cmd_templates_legacy_catalog\(/);
  assert.match(templateCommand, /use super::command_output::write_rendered_output_path;/);
  assert.match(templateCommand, /parse_templates_verify_readiness_options\(cwd, args\)\?/);
  assert.match(templateCommand, /parse_templates_catalog_options\(cwd, args\)\?/);
  assert.match(templateCommand, /template_readiness::verify_template_readiness/);
  assert.match(templateCommand, /"schema": "dx\.www\.templates"/);
  assert.match(templateCommand, /"launch_package_catalog": launch_package_catalog/);
  assert.match(templateCommand, /write_rendered_output_path\(\s*output,\s*&rendered,\s*quiet,\s*"templates verify-readiness"\s*\)\?/);
  assert.match(templateCommand, /write_rendered_output_path\(output, &rendered, quiet, "templates"\)\?/);
  assert.doesNotMatch(templateCommand, /std::fs::write\(&output, &rendered\)/);
  assert.doesNotMatch(templateCommand, /let mut output: Option<PathBuf> = None/);
  assert.doesNotMatch(templateCommand, /Unknown templates verify-readiness option/);
  assert.doesNotMatch(templateCommand, /Unexpected templates verify-readiness argument/);
  assert.doesNotMatch(templateCommand, /Unknown templates option/);
  assert.doesNotMatch(templateCommand, /Unexpected templates argument/);

  assert.match(templateOptions, /pub\(super\) struct DxTemplatesCatalogOptions/);
  assert.match(templateOptions, /pub\(super\) struct DxTemplatesVerifyReadinessOptions/);
  assert.match(templateOptions, /pub\(super\) fn parse_templates_catalog_options\(/);
  assert.match(templateOptions, /pub\(super\) fn parse_templates_verify_readiness_options\(/);
  assert.match(templateOptions, /Unknown templates option/);
  assert.match(templateOptions, /Unknown templates verify-readiness option/);
  assert.match(templateOptions, /Unexpected templates argument/);
  assert.match(templateOptions, /Unexpected templates verify-readiness argument/);
  assert.match(templateOptions, /mod tests/);
});
