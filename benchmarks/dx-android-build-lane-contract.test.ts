import assert from "node:assert/strict";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath: string): string {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

function assertSourceFile(relativePath: string): string {
  const path = resolve(repoRoot, relativePath);
  assert.ok(existsSync(path), `${relativePath} should exist`);
  return readFileSync(path, "utf8");
}

function sliceAround(source: string, needle: string, length = 4_000): string {
  const start = source.indexOf(needle);
  assert.notEqual(start, -1, `expected to find ${needle}`);
  return source.slice(start, start + length);
}

test("DX Android build lane is split into focused production modules", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const commandPath = "dx-www/src/cli/native_android_build_command.rs";
  const command = assertSourceFile(commandPath);
  const disk = assertSourceFile("dx-www/src/cli/native_android_build_disk.rs");
  const environment = assertSourceFile("dx-www/src/cli/native_android_build_environment.rs");
  const plan = assertSourceFile("dx-www/src/cli/native_android_build_plan.rs");
  const preflight = assertSourceFile("dx-www/src/cli/native_android_build_preflight.rs");
  const process = assertSourceFile("dx-www/src/cli/native_android_build_process.rs");
  const gradle = assertSourceFile("dx-www/src/cli/native_android_build_gradle.rs");
  const receipt = assertSourceFile("dx-www/src/cli/native_android_build_receipt.rs");
  const proofStruct = sliceAround(receipt, "struct AndroidProofReceipt", 1_800);
  const collectProof = sliceAround(receipt, "fn collect_android_proof", 6_500);
  const staticEntries = sliceAround(receipt, "fn android_static_apk_proof_entries", 900);
  const proofInit = sliceAround(receipt, "    AndroidProofReceipt {", 1_800);
  const cliCore = assertSourceFile("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const nativeShellValidation = assertSourceFile("dx-www/src/cli/native_shell_validation.rs");
  const helpText = assertSourceFile("dx-www/src/cli/help_text.rs");

  assert.ok(
    statSync(resolve(repoRoot, commandPath)).size < 7_500,
    "native_android_build_command.rs should stay a small orchestrator",
  );
  assert.match(cliMod, /^mod native_android_build_plan;$/m);
  assert.match(cliMod, /^mod native_android_build_disk;$/m);
  assert.match(cliMod, /^mod native_android_build_environment;$/m);
  assert.match(cliMod, /^mod native_android_build_preflight;$/m);
  assert.match(cliMod, /^mod native_android_build_process;$/m);
  assert.match(cliMod, /^mod native_android_build_gradle;$/m);
  assert.match(cliMod, /^mod native_android_build_receipt;$/m);
  assert.match(command, /native_android_build_plan::\{/);
  assert.match(command, /AndroidBuildPlan/);
  assert.match(command, /build_android_build_plan/);
  assert.match(command, /native_android_build_preflight::\{/);
  assert.match(command, /AndroidToolchainPreflight/);
  assert.match(command, /check_android_toolchain_preflight/);
  assert.match(command, /android_toolchain_preflight_step/);
  assert.match(command, /let preflight = check_android_toolchain_preflight\(\)\?/);
  assert.match(command, /native_android_build_environment::\{/);
  assert.match(command, /AndroidBuildEnvironment/);
  assert.match(command, /check_android_build_environment/);
  assert.match(command, /android_build_environment_step/);
  assert.match(command, /let environment = check_android_build_environment\(&plan\.project_root\)\?/);
  assert.match(command, /android_build_environment_step\(&plan, &environment\)/);
  assert.match(command, /preflight\.tauri_program\(\)/);
  const buildTargetDispatch = sliceAround(cliCore, "fn cmd_build_with_options", 900);
  const androidBuildDispatch = sliceAround(buildTargetDispatch, "DxBuildTarget::Android", 260);
  assert.match(androidBuildDispatch, /self\.cmd_build_web\(\)\?/);
  assert.ok(
    androidBuildDispatch.indexOf("self.cmd_build_web()?") <
      androidBuildDispatch.indexOf("cmd_www_build_android(&self.cwd, invoked_as)"),
    "Android target must refresh DX WWW output before native packaging",
  );
  const androidBuildCommand = sliceAround(command, "pub(super) fn cmd_www_build_android", 2_200);
  assert.ok(
    androidBuildCommand.indexOf("check_android_toolchain_preflight") <
      androidBuildCommand.indexOf("check_android_build_environment") &&
      androidBuildCommand.indexOf("check_android_build_environment") <
      androidBuildCommand.indexOf("ensure_www_native_shell_for_build"),
    "Android toolchain and environment preflight must run before native shell materialization",
  );
  assert.match(environment, /pub\(super\) struct AndroidBuildEnvironment/);
  assert.match(environment, /ANDROID_BUILD_MIN_FREE_BYTES/);
  assert.match(environment, /GRADLE_USER_HOME/);
  assert.match(environment, /ANDROID_USER_HOME/);
  assert.match(environment, /ANDROID_AVD_HOME/);
  assert.match(environment, /CARGO_HOME/);
  assert.match(environment, /CARGO_TARGET_DIR/);
  assert.match(environment, /TEMP/);
  assert.match(environment, /TMP/);
  assert.match(environment, /\.dx\/native\/android-cache/);
  assert.match(environment, /\.dx\/native\/android-target/);
  assert.match(environment, /verify_android_environment_writable/);
  assert.match(environment, /available_free_bytes/);
  assert.match(environment, /insufficient disk space/);
  assert.match(environment, /android_build_environment_step/);
  assert.match(disk, /pub\(super\) fn available_free_bytes/);
  assert.match(disk, /GetDiskFreeSpaceExW/);
  assert.match(disk, /SAFETY:/);
  assert.match(command, /native_android_build_process::\{/);
  assert.match(command, /android_build_command/);
  assert.match(command, /run_android_command/);
  assert.match(command, /use super::native_android_build_gradle::patch_android_gradle_helper;/);
  assert.match(command, /native_android_build_receipt::\{/);
  assert.match(command, /build_android_receipt/);
  assert.match(command, /inspect_android_artifact/);
  assert.match(command, /write_android_receipt/);
  assert.doesNotMatch(command, /std::process::Command/);
  assert.match(plan, /pub\(super\) struct AndroidBuildPlan/);
  assert.match(preflight, /pub\(super\) struct AndroidToolchainPreflight/);
  assert.match(preflight, /pub\(super\) fn tauri_program\(&self\) -> String/);
  assert.match(preflight, /pub\(super\) fn check_android_toolchain_preflight/);
  assert.match(preflight, /pub\(super\) fn android_toolchain_preflight_step/);
  assert.match(preflight, /require_tool_on_path\("tauri"\)/);
  assert.match(preflight, /require_tool_on_path\("rustup"\)/);
  assert.match(preflight, /target[\s\S]*list[\s\S]*--installed/);
  assert.match(preflight, /aarch64-linux-android/);
  assert.match(preflight, /ANDROID_HOME/);
  assert.match(preflight, /ANDROID_SDK_ROOT/);
  assert.match(preflight, /platform-tools/);
  assert.match(preflight, /build-tools/);
  assert.match(preflight, /platforms/);
  assert.match(preflight, /source\.properties/);
  assert.match(preflight, /valid_java_home/);
  assert.match(preflight, /build\.target\.android\.toolchain/);
  assert.match(preflight, /does_not_mutate_project/);
  assert.doesNotMatch(preflight, /std::fs::write/);
  assert.doesNotMatch(preflight, /std::fs::create_dir_all/);
  assert.match(process, /AndroidBuildEnvironment/);
  assert.match(process, /CARGO_BUILD_JOBS/);
  assert.match(process, /GRADLE_OPTS/);
  assert.match(process, /apply_android_environment\(&mut command, environment\)/);
  const applyAndroidEnvironment = sliceAround(process, "fn apply_android_environment", 2_200);
  assert.doesNotMatch(applyAndroidEnvironment, /project_root: &Path/);
  assert.doesNotMatch(applyAndroidEnvironment, /create_dir_all/);
  assert.doesNotMatch(process, /\.dx\/native\/android-cache/);
  assert.match(process, /filter\(\|option\| !option\.starts_with\("-Dorg\.gradle\.workers\.max="\)\)/);
  assert.match(process, /filter\(\|option\| !option\.starts_with\("-Dorg\.gradle\.daemon="\)\)/);
  assert.match(process, /filter\(\|option\| !option\.starts_with\("-Dorg\.gradle\.jvmargs="\)\)/);
  assert.match(process, /filter\(\|option\| !option\.starts_with\("-Xmx"\)\)/);
  assert.match(process, /-Dorg\.gradle\.daemon=false/);
  assert.match(process, /-Xmx1536m/);
  assert.match(process, /android_build_command\(tauri_program: String\)/);
  assert.match(process, /tauri_icon_command\(tauri_program: String/);
  assert.match(process, /tauri_android_init_command\(tauri_program: String\)/);
  assert.match(process, /jar_list_command\([\s\S]*apk_relative_path: &str,[\s\S]*entries: &\[&str\]/);
  assert.match(process, /ANDROID_HOME/);
  assert.match(process, /COMMAND_OUTPUT_TAIL_CHARS/);
  assert.match(process, /BoundedOutputTail/);
  assert.match(process, /AndroidCommandResult/);
  assert.match(process, /run_android_command_capture/);
  assert.match(process, /display_command_part/);
  assert.match(process, /part\.chars\(\)\.any\(char::is_whitespace\)/);
  assert.match(process, /android_disk_space_failure/);
  assert.match(process, /ENOSPC/);
  assert.match(process, /os error 112/);
  assert.match(process, /No space left on device/);
  const runAndroidCommand = sliceAround(process, "pub(super) fn run_android_command", 1_500);
  assert.match(runAndroidCommand, /run_android_command_capture\(project_root, spec, environment\)/);
  assert.doesNotMatch(runAndroidCommand, /\.output\(\)/);
  const captureRunner = sliceAround(process, "fn run_android_command_capture", 5_500);
  assert.match(captureRunner, /stdout\(Stdio::piped\(\)\)/);
  assert.match(captureRunner, /stderr\(Stdio::piped\(\)\)/);
  assert.match(captureRunner, /\.spawn\(\)/);
  assert.match(captureRunner, /child\.wait\(\)/);
  assert.match(captureRunner, /spawn_output_tail_reader/);
  assert.doesNotMatch(captureRunner, /\.output\(\)/);
  assert.doesNotMatch(captureRunner, /read_until/);
  assert.match(captureRunner, /reader\.read\(&mut buffer\)/);
  const runOptionalAndroidCommand = sliceAround(process, "pub(super) fn run_optional_android_command", 2_500);
  assert.match(runOptionalAndroidCommand, /environment: &AndroidBuildEnvironment/);
  assert.match(runOptionalAndroidCommand, /run_android_command_capture\(project_root, &spec, environment\)/);
  assert.doesNotMatch(runOptionalAndroidCommand, /\.output\(\)/);
  assert.match(process, /numeric_revision_key/);
  assert.match(process, /35\.0\.0/);
  assert.match(process, /9\.0\.0/);
  assert.match(gradle, /patch_gradle_helper_source/);
  assert.match(proofStruct, /native_library_packaged: Option<bool>/);
  assert.match(proofStruct, /tauri_config_packaged: Option<bool>/);
  assert.match(proofStruct, /android_manifest_packaged: Option<bool>/);
  assert.match(proofStruct, /dex_bytecode_packaged: Option<bool>/);
  assert.match(proofStruct, /native_www_entrypoint_present: bool/);
  assert.match(proofStruct, /www_assets_packaged: Option<bool>/);
  assert.match(proofStruct, /www_asset_proof_mode: &'static str/);
  assert.match(proofStruct, /www_asset_marker_limit: usize/);
  assert.match(proofStruct, /www_asset_markers_truncated: bool/);
  assert.match(proofStruct, /expected_android_manifest: &'static str/);
  assert.match(proofStruct, /expected_dex_bytecode: &'static str/);
  assert.match(proofStruct, /expected_native_www_entrypoint: &'static str/);
  assert.match(proofStruct, /expected_www_assets: Vec<String>/);
  assert.match(proofStruct, /missing_www_assets: Vec<String>/);
  assert.match(proofStruct, /www_asset_marker_scan: AndroidProofCommandReceipt/);
  assert.match(proofStruct, /static_native_loadability_proof_complete: bool/);
  assert.doesNotMatch(proofInit, /android_manifest_packaged:\s*Some\(true\)/);
  assert.doesNotMatch(proofInit, /dex_bytecode_packaged:\s*Some\(true\)/);
  assert.doesNotMatch(proofInit, /static_native_loadability_proof_complete:\s*true/);
  assert.match(receipt, /apk_signature_scheme_v2/);
  assert.match(receipt, /distribution_ready/);
  assert.match(receipt, /release_signed/);
  assert.match(receipt, /debug_signed/);
  assert.match(receipt, /let debug_signed = proof\.apk_signature_scheme_v2 == Some\(true\)/);
  assert.doesNotMatch(receipt, /debug_signed: true/);
  assert.match(receipt, /environment: AndroidEnvironmentReceipt/);
  assert.match(receipt, /AndroidBuildEnvironment/);
  assert.match(receipt, /cache_root/);
  assert.match(receipt, /required_free_bytes/);
  assert.match(receipt, /available_free_bytes/);
  assert.match(receipt, /free_space_status/);
  assert.match(receipt, /canonical_command/);
  assert.match(receipt, /invoked_as/);
  assert.match(receipt, /let expected_www_asset_plan = expected_www_asset_marker_plan\(&plan\.project_root\)/);
  assert.match(staticEntries, /expected_native_library\.to_string\(\)/);
  assert.match(staticEntries, /ANDROID_TAURI_CONFIG_ENTRY\.to_string\(\)/);
  assert.match(staticEntries, /ANDROID_MANIFEST_ENTRY\.to_string\(\)/);
  assert.match(staticEntries, /ANDROID_DEX_BYTECODE_ENTRY\.to_string\(\)/);
  assert.match(collectProof, /let content_entries = android_static_apk_proof_entries\(expected_native_library\)/);
  assert.match(collectProof, /jar_list_command\(ARM64_DEBUG_APK_PATH, &content_entry_refs\)/);
  assert.match(receipt, /NATIVE_WWW_ENTRYPOINT/);
  assert.match(receipt, /dx_www_native_entrypoint_present/);
  assert.match(receipt, /WWW_ASSET_MARKER_PROOF_LIMIT/);
  assert.match(receipt, /fn expected_www_asset_marker_plan\(project_root: &Path\) -> WwwAssetMarkerPlan/);
  assert.match(receipt, /fn expected_www_asset_markers\(project_root: &Path\) -> Vec<String>/);
  assert.match(receipt, /fn collect_www_asset_markers\(output_root: &Path, current: &Path, markers: &mut Vec<String>\)/);
  assert.match(receipt, /std::fs::read_dir/);
  assert.match(receipt, /\.dx\/www\/output/);
  assert.match(receipt, /app\/index\.html/);
  assert.match(receipt, /source-routes\/root\/index\.html/);
  assert.match(receipt, /public\/mobile-companion-runtime\.js/);
  assert.match(receipt, /WWW_BUILD_EVIDENCE_MARKERS/);
  assert.match(receipt, /manifest\.json/);
  assert.match(receipt, /source-build-manifest\.json/);
  assert.match(receipt, /source-build-receipt\.json/);
  assert.match(receipt, /is_www_build_evidence_marker/);
  assert.match(receipt, /apk-binary-marker/);
  assert.match(receipt, /APK_MARKER_SCAN_CHUNK_BYTES/);
  assert.match(receipt, /missing_expected_www_asset_markers/);
  assert.match(receipt, /sha256/);
  assert.match(receipt, /File::open/);
  assert.match(receipt, /std::io::copy/);
  assert.doesNotMatch(receipt, /std::fs::read\(path\)/);
  assert.doesNotMatch(receipt, /std::fs::read\(apk_path\)/);
  assert.match(receipt, /adb_devices_connected/);
  const loadabilityGate = sliceAround(receipt, "let static_native_loadability_proof_complete", 900);
  assert.match(loadabilityGate, /native_library_packaged == Some\(true\)/);
  assert.match(loadabilityGate, /tauri_config_packaged == Some\(true\)/);
  assert.match(loadabilityGate, /android_manifest_packaged == Some\(true\)/);
  assert.match(loadabilityGate, /dex_bytecode_packaged == Some\(true\)/);
  const staticApkGate = sliceAround(receipt, "let static_apk_proof_complete", 900);
  assert.match(staticApkGate, /www_output_present\s*&&/);
  assert.match(staticApkGate, /native_www_entrypoint_present\s*&&/);
  assert.match(staticApkGate, /static_native_loadability_proof_complete\s*&&/);
  assert.match(staticApkGate, /www_assets_packaged == Some\(true\)/);
  assert.doesNotMatch(
    staticApkGate,
    /native_library_packaged == Some\(true\)[\s\S]*tauri_config_packaged == Some\(true\)/,
  );
  assert.match(receipt, /device_smoke_proof_complete/);
  assert.match(receipt, /proof_complete/);
  assert.match(receipt, /apk-built-static-proof/);
  assert.match(receipt, /apk-built-proof-partial/);
  assert.doesNotMatch(receipt, /apk-built-verified/);
  assert.match(receipt, /state == "device"/);
  assert.match(receipt, /offline/);
  assert.match(receipt, /unauthorized/);
  assert.match(nativeShellValidation, /validate_native_shell_tauri_config/);
  assert.match(nativeShellValidation, /frontendDist[\s\S]*\.\.\/\.dx\/www\/output/);
  assert.match(nativeShellValidation, /beforeBuildCommand[\s\S]*dx www build/);
  assert.match(nativeShellValidation, /removeUnusedCommands/);
  assert.match(helpText, /build\s+Build web output or target native artifacts/);
});
