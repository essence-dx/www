import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandOutputPath = path.join(repoRoot, "dx-www", "src", "cli", "command_output.rs");
const studioSurfacePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "studio_json_surface.rs",
);
const studioCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "studio_command.rs",
);

test("Studio JSON surface parsing, output, and command glue live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(studioSurfacePath),
    "expected dx-www/src/cli/studio_json_surface.rs",
  );
  assert.ok(
    fs.existsSync(commandOutputPath),
    "expected dx-www/src/cli/command_output.rs",
  );
  assert.ok(
    fs.existsSync(studioCommandPath),
    "expected dx-www/src/cli/studio_command.rs",
  );

  const studioSurface = fs.readFileSync(studioSurfacePath, "utf8");
  const studioCommand = fs.readFileSync(studioCommandPath, "utf8");

  assert.match(cliMod, /^mod command_output;$/m);
  assert.match(cliMod, /^mod studio_command;$/m);
  assert.match(cliMod, /^mod studio_json_surface;$/m);
  assert.match(cliMod, /studio_command::cmd_routes\(&self\.cwd, args\)/);
  assert.match(cliMod, /studio_command::cmd_preview_manifest\(&self\.cwd, args\)/);

  assert.doesNotMatch(cliMod, /^\s*fn parse_studio_json_surface_args\(/m);
  assert.doesNotMatch(cliMod, /^\s*fn write_or_print_studio_json_surface\(/m);
  assert.doesNotMatch(cliMod, /^\s*fn attach_local_www_routes\(/m);
  assert.doesNotMatch(cliMod, /^\s*fn local_www_routes_report_entries\(/m);
  assert.doesNotMatch(cliMod, /^\s*fn route_preview_url\(/m);
  assert.doesNotMatch(cliMod, /^\s*fn project_relative_slash_path\(/m);

  assert.match(studioSurface, /pub\(super\) struct DxStudioJsonSurfaceArgs/);
  assert.match(studioSurface, /use super::command_output::write_rendered_output_path;/);
  assert.match(studioSurface, /pub\(super\) fn parse_studio_json_surface_args\(/);
  assert.match(studioSurface, /pub\(super\) fn write_or_print_studio_json_surface\(/);
  assert.match(
    studioSurface,
    /write_rendered_output_path\(output, rendered, quiet, "studio-json-surface"\)/,
  );
  assert.doesNotMatch(studioSurface, /std::fs::write\(&output, rendered\)/);
  assert.match(studioSurface, /mod tests/);

  assert.match(
    studioCommand,
    /pub\(super\) fn cmd_routes\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/,
  );
  assert.match(
    studioCommand,
    /pub\(super\) fn cmd_preview_manifest\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/,
  );
  assert.match(
    studioCommand,
    /parse_studio_json_surface_args\(cwd, args, "routes"\)\?/,
  );
  assert.match(
    studioCommand,
    /parse_studio_json_surface_args\(cwd, args, "preview-manifest"\)\?/,
  );
  assert.match(studioCommand, /build_www_routes_report\(&Utc::now\(\)\.to_rfc3339\(\)\)/);
  assert.match(
    studioCommand,
    /build_studio_preview_manifest\(&Utc::now\(\)\.to_rfc3339\(\)\)/,
  );
  assert.match(studioCommand, /Project::scan\(cwd, config\.clone\(\)\)\?/);
  assert.match(studioCommand, /fn attach_local_www_routes\(/);
});
