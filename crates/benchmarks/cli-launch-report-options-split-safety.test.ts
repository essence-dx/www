import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const launchReportOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "launch_report_options.rs",
);
const templatesCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "templates_command.rs",
);

test("launch report option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(launchReportOptionsPath),
    "expected dx-www/src/cli/launch_report_options.rs",
  );
  assert.ok(
    fs.existsSync(templatesCommandPath),
    "expected dx-www/src/cli/templates_command.rs",
  );

  const launchReportOptions = fs.readFileSync(launchReportOptionsPath, "utf8");
  const templatesCommand = fs.readFileSync(templatesCommandPath, "utf8");
  const reportStart = cliMod.indexOf("fn cmd_forge_launch_readiness_bundle");
  const routesStart = cliMod.indexOf("fn cmd_routes", reportStart);
  assert.notEqual(reportStart, -1, "expected launch readiness command in cli module");
  assert.notEqual(routesStart, -1, "expected routes command after launch reports");
  const launchReportCommands = cliMod.slice(reportStart, routesStart);

  assert.match(cliMod, /^mod launch_report_options;$/m);
  assert.match(cliMod, /^mod templates_command;$/m);
  assert.match(templatesCommand, /fn cmd_templates_legacy_catalog\(/);
  assert.match(
    cliMod,
    /use self::launch_report_options::\{\s*parse_launch_report_options,\s*DxLaunchReportCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(launchReportCommands, /let mut fail_under = 100u8/);
  assert.doesNotMatch(launchReportCommands, /let mut index = 0usize/);
  assert.doesNotMatch(launchReportCommands, /Unknown forge launch-readiness-bundle option/);
  assert.doesNotMatch(launchReportCommands, /Unknown forge launch-adoption-report option/);
  assert.doesNotMatch(launchReportCommands, /Unknown forge launch-manifest-drift option/);
  assert.doesNotMatch(launchReportCommands, /Unknown forge launch-companion-receipts option/);
  assert.doesNotMatch(launchReportCommands, /Unknown forge launch-runtime-checklist option/);
  assert.match(
    launchReportCommands,
    /parse_launch_report_options\(&self\.cwd,\s*args,\s*"launch-readiness-bundle",\s*100\)/s,
  );
  assert.match(
    launchReportCommands,
    /parse_launch_report_options\(&self\.cwd,\s*args,\s*"launch-adoption-report",\s*100\)/s,
  );
  assert.match(
    launchReportCommands,
    /parse_launch_report_options\(&self\.cwd,\s*args,\s*"launch-manifest-drift",\s*100\)/s,
  );
  assert.match(
    launchReportCommands,
    /parse_launch_report_options\(&self\.cwd,\s*args,\s*"launch-companion-receipts",\s*100\)/s,
  );
  assert.match(
    launchReportCommands,
    /parse_launch_report_options\(&self\.cwd,\s*args,\s*"launch-runtime-checklist",\s*100\)/s,
  );

  assert.match(launchReportOptions, /pub\(super\) struct DxLaunchReportCommandOptions/);
  assert.match(launchReportOptions, /pub\(super\) fn parse_launch_report_options\(/);
  assert.match(launchReportOptions, /Unknown forge \{command_label\} option: \{value\}/);
  assert.match(launchReportOptions, /Unexpected forge launch-runtime-checklist argument/);
  assert.match(launchReportOptions, /mod tests/);
});
