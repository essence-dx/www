import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const cliCorePath = path.join(repoRoot, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");
const buildCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "build_command.rs");
const buildOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "build_options.rs");

function sliceAround(source: string, needle: string, length = 900): string {
  const start = source.indexOf(needle);
  assert.notEqual(start, -1, `expected to find ${needle}`);
  return source.slice(start, start + length);
}

function sliceRustMethod(source: string, signature: string): string {
  const start = source.indexOf(signature);
  assert.notEqual(start, -1, `expected to find ${signature}`);
  const nextMethod = source.indexOf("\n    pub fn ", start + signature.length);
  return source.slice(start, nextMethod === -1 ? undefined : nextMethod);
}

test("dx build command validates args before executing the build", () => {
  assert.ok(fs.existsSync(buildCommandPath), "expected focused dx build command module");
  assert.ok(fs.existsSync(buildOptionsPath), "expected focused dx build options module");
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const cliCore = fs.readFileSync(cliCorePath, "utf8");
  const buildCommand = fs.readFileSync(buildCommandPath, "utf8");
  const buildOptions = fs.readFileSync(buildOptionsPath, "utf8");
  const topLevelBuildBranch = sliceAround(cliCore, '"build" => {');
  const wwwBuildBranch = sliceAround(cliCore, 'Some("build") => {');
  const buildDispatcher = sliceRustMethod(
    cliCore,
    "fn cmd_build_with_options(",
  );
  const cmdBuild = sliceAround(cliCore, "fn cmd_build_web(&self) -> DxResult<()>", 14_000);

  assert.match(cliMod, /^mod build_command;$/m);
  assert.match(cliMod, /^mod build_options;$/m);
  assert.match(cliMod, /^mod native_android_build_command;$/m);
  assert.match(cliMod, /use self::build_command::\{[\s\S]*run_build_command[\s\S]*\};/);
  assert.match(cliMod, /use self::build_options::\{DxBuildCommandOptions, DxBuildTarget\};/);
  assert.match(cliMod, /use self::native_android_build_command::cmd_www_build_android;/);
  assert.match(
    topLevelBuildBranch,
    /"build" => \{[\s\S]*run_build_command\(&args\[2\.\.\], "dx build", \|options\| \{[\s\S]*cli\.cmd_build_with_options\(options, "dx build"\)/,
  );
  assert.match(
    wwwBuildBranch,
    /Some\("build"\) => \{[\s\S]*run_build_command\(&args\[1\.\.\], "dx www build", \|options\| \{[\s\S]*self\.cmd_build_with_options\(options, "dx www build"\)/,
  );
  assert.match(buildCommand, /pub\(super\) fn run_build_command<F>\(/);
  assert.match(buildCommand, /command_name: &'static str/);
  assert.match(buildCommand, /F: FnOnce\(DxBuildCommandOptions\) -> DxResult<\(\)>/);
  assert.match(buildCommand, /print_build_help\(command_name\)/);
  assert.match(buildCommand, /parse_build_options\(args, command_name\)/);
  assert.match(buildCommand, /build\(options\)/);
  assert.match(buildOptions, /pub\(super\) enum DxBuildTarget/);
  assert.match(buildOptions, /Android/);
  assert.match(buildOptions, /pub\(super\) struct DxBuildCommandOptions/);
  assert.match(buildOptions, /pub\(super\) fn parse_build_options\(/);
  assert.match(buildOptions, /args: &\[String\]/);
  assert.match(buildOptions, /command_name: &'static str/);
  assert.match(buildOptions, /\) -> DxResult<DxBuildCommandOptions>/);
  assert.match(buildOptions, /"--target"/);
  assert.match(buildOptions, /"--target="/);
  assert.match(buildOptions, /Supported targets: web, android/);
  assert.match(buildOptions, /Unknown target for \{command_name\}: \{value\}/);
  assert.match(buildOptions, /value\.starts_with\('-'\)/);
  assert.match(buildOptions, /Unknown option for \{command_name\}/);
  assert.match(buildOptions, /dx www build/);
  assert.match(buildOptions, /build_options_error\([\s\S]*"build"/);
  assert.match(buildDispatcher, /DxBuildTarget::Web => self\.cmd_build_web\(\)/);
  assert.match(buildDispatcher, /invoked_as: &'static str/);
  assert.match(buildDispatcher, /DxBuildTarget::Android => cmd_www_build_android\(&self\.cwd, invoked_as\)/);
  assert.match(cmdBuild, /ensure_build_output_dirs\(&self\.cwd,\s*&output_dir\)\?/);
  assert.match(
    cmdBuild,
    /let legacy_pages =\s+compile_legacy_pages\(\s*&self\.cwd,\s*&output_dir,\s*&translations,\s*\|page_path, translations\| self\.compile_to_binary\(page_path, translations\),\s*\)\?/,
  );
  assert.match(cmdBuild, /compiled_count \+= legacy_pages\.compiled_count;/);
  assert.match(cmdBuild, /total_size \+= legacy_pages\.total_size;/);
  assert.doesNotMatch(cmdBuild, /let pages_dir = self\.cwd\.join\("pages"\);/);
  assert.doesNotMatch(cmdBuild, /filter_map\(\|e\| e\.ok\(\)\)/);
  assert.doesNotMatch(cmdBuild, /strip_prefix\(&pages_dir\)\.unwrap\(\)/);
  assert.doesNotMatch(cmdBuild, /std::fs::create_dir_all\(parent\)\.ok\(\)/);
  assert.match(cmdBuild, /copy_build_asset_tree\(&self\.cwd, &output_dir, "styles"\)\?/);
  assert.match(cmdBuild, /copy_build_asset_tree\(&self\.cwd, &output_dir, "public"\)\?/);
  assert.doesNotMatch(cmdBuild, /std::fs::copy\(src, &dst\)\.ok\(\)/);
  assert.match(
    cmdBuild,
    /let server_build_artifacts = write_server_build_artifacts\(&output_dir, &server_sources\)\?;/,
  );
  assert.match(
    cmdBuild,
    /server_contracts_compiled = server_build_artifacts\.server_contracts_compiled;/,
  );
  assert.match(
    cmdBuild,
    /server_action_protocols_compiled =\s+server_build_artifacts\.server_action_protocols_compiled;/,
  );
  assert.match(
    cmdBuild,
    /let import_artifacts = write_import_build_artifacts\(&output_dir, &import_resolutions\)\?;/,
  );
  assert.match(
    cmdBuild,
    /import_resolutions_compiled = import_artifacts\.import_resolutions_compiled;/,
  );
  assert.match(
    cmdBuild,
    /let next_adapter_fixtures_emitted = import_artifacts\.next_adapter_fixtures_emitted;/,
  );
  assert.match(
    cmdBuild,
    /let next_migration_artifacts =\s+write_next_migration_build_artifacts\(next_migration_input\)\?;/,
  );
  assert.match(
    cmdBuild,
    /next_migration_artifacts\.next_migration_proof_emitted/,
  );
  assert.match(
    cmdBuild,
    /next_migration_artifacts\.next_familiar_compatibility_evidence_emitted/,
  );
  assert.match(
    cmdBuild,
    /write_build_manifest_and_deploy_adapter\(\s*&output_dir,\s*BuildManifestInput \{[\s\S]*compiled_count,[\s\S]*source_build_report: &source_build_report,[\s\S]*\},\s*\|manifest_json\| \{[\s\S]*deploy_adapter_contract::write_deploy_adapter_contract\(/,
  );
  assert.doesNotMatch(cmdBuild, /std::fs::write\(\s*output_dir\.join\("server-contracts\.json"\)/);
  assert.doesNotMatch(
    cmdBuild,
    /std::fs::write\(\s*output_dir\.join\("server-action-protocols\.json"\)/,
  );
  assert.doesNotMatch(
    cmdBuild,
    /std::fs::write\(\s*output_dir\.join\("server-action-runtime\.json"\)/,
  );
  assert.doesNotMatch(cmdBuild, /std::fs::write\(\s*output_dir\.join\("import-resolution\.json"\)/);
  assert.doesNotMatch(cmdBuild, /build_next_project_migration_proof/);
  assert.doesNotMatch(cmdBuild, /build_next_familiar_compatibility_evidence/);
  assert.doesNotMatch(cmdBuild, /NEXT_MIGRATION_PROOF_JSON/);
  assert.doesNotMatch(cmdBuild, /NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON/);
  assert.doesNotMatch(cmdBuild, /let manifest = serde_json::json!\(\{/);
  assert.doesNotMatch(cmdBuild, /serde_json::to_string_pretty\(&manifest\)/);
  assert.doesNotMatch(cmdBuild, /std::fs::write\(output_dir\.join\("manifest\.json"\)/);
  assert.match(
    buildCommand,
    /pub\(super\) fn ensure_build_output_dirs\(project_root: &Path,\s*output_dir: &Path\) -> DxResult<\(\)>/,
  );
  assert.match(
    buildCommand,
    /pub\(super\) fn compile_legacy_pages<F>\([\s\S]*compile_to_binary: F[\s\S]*LegacyPageBuildSummary[\s\S]*filter\(|entry\.file_type\(\)\.is_file\(\)[\s\S]*create_build_dir\(parent\)\?[\s\S]*std::fs::write\(&output_path, &binary\)\.map_err/,
  );
  assert.match(
    buildCommand,
    /pub\(super\) fn copy_build_asset_tree\([\s\S]*std::fs::copy\(src, &dst\)[\s\S]*map_err/,
  );
  assert.match(buildCommand, /pub\(super\) struct ServerBuildArtifacts/);
  assert.match(buildCommand, /pub\(super\) fn write_server_build_artifacts\(/);
  assert.match(buildCommand, /pub\(super\) struct ImportBuildArtifacts/);
  assert.match(buildCommand, /pub\(super\) fn write_import_build_artifacts\(/);
  assert.match(buildCommand, /pub\(super\) struct NextMigrationBuildArtifacts/);
  assert.match(buildCommand, /pub\(super\) fn write_next_migration_build_artifacts\(/);
  assert.match(buildCommand, /fn write_json_artifact<T: Serialize>\(/);
  assert.match(buildCommand, /pub\(super\) struct BuildManifestInput/);
  assert.match(buildCommand, /pub\(super\) fn write_build_manifest_and_deploy_adapter<F>\(/);
  assert.match(buildCommand, /fn build_manifest_value\(input: BuildManifestInput<'_>\) -> serde_json::Value/);
  assert.match(buildCommand, /fn serialize_json_artifact<T: Serialize>\(/);
  assert.match(buildCommand, /server-contracts\.json/);
  assert.match(buildCommand, /server-action-protocols\.json/);
  assert.match(buildCommand, /server-action-runtime\.json/);
  assert.match(buildCommand, /import-resolution\.json/);
  assert.match(buildCommand, /NEXT_MIGRATION_PROOF_JSON/);
  assert.match(buildCommand, /NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON/);
  assert.match(buildCommand, /manifest\.json/);
  assert.match(buildCommand, /deploy_adapter_emitted/);
});

test("dx www build --target android routes to the Android build lane", () => {
  const cliCore = fs.readFileSync(cliCorePath, "utf8");
  const buildCommand = fs.readFileSync(buildCommandPath, "utf8");
  const buildOptions = fs.readFileSync(buildOptionsPath, "utf8");
  const wwwBuildBranch = sliceAround(cliCore, 'Some("build") => {', 700);
  const buildDispatcher = sliceRustMethod(
    cliCore,
    "fn cmd_build_with_options(",
  );

  assert.match(
    wwwBuildBranch,
    /run_build_command\(&args\[1\.\.\], "dx www build", \|options\| \{/,
    "dx www build must parse the forwarded build args, not the nested command name",
  );
  assert.match(
    buildCommand,
    /parse_build_options\(args, command_name\)/,
    "dx www build must use the shared target parser before executing build work",
  );
  assert.match(
    buildOptions,
    /"android" => Ok\(DxBuildTarget::Android\)/,
    "the build target parser must accept android as a first-class target",
  );
  assert.match(
    buildDispatcher,
    /DxBuildTarget::Android => cmd_www_build_android\(&self\.cwd, invoked_as\)/,
    "the android target must enter the native Android build command",
  );
  assert.doesNotMatch(
    wwwBuildBranch,
    /cmd_build_web\(\)/,
    "the www build branch should not bypass the shared target dispatcher",
  );
});
