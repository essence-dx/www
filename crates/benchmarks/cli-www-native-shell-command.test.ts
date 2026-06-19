import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath: string): string {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

function commandBody(source: string, name: string, nextName: string): string {
  const start = source.indexOf(`pub fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = source.indexOf(`pub fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return source.slice(start, end);
}

test("dx www native-shell command is split out of the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const cliCore = readRepoFile("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const helpText = readRepoFile("dx-www/src/cli/help_text.rs");

  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_command.rs")),
    "native_shell_command.rs should own source materialization",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_options.rs")),
    "native_shell_options.rs should own parsing",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_materializer.rs")),
    "native_shell_materializer.rs should own source writes and build-time shell creation",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_naming.rs")),
    "native_shell_naming.rs should own shared native project identity helpers",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_plan.rs")),
    "native_shell_plan.rs should own report planning",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_render.rs")),
    "native_shell_render.rs should own terminal/json/markdown rendering",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_templates.rs")),
    "native_shell_templates.rs should own generated Tauri source files",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/native_shell_validation.rs")),
    "native_shell_validation.rs should own existing-shell validation",
  );
  assert.ok(
    existsSync(resolve(repoRoot, "dx-www/src/cli/www_output_presence.rs")),
    "www_output_presence.rs should own shared DX WWW output detection",
  );

  assert.match(cliMod, /^mod native_shell_command;$/m);
  assert.match(cliMod, /^mod native_shell_materializer;$/m);
  assert.match(cliMod, /^mod native_shell_naming;$/m);
  assert.match(cliMod, /^mod native_shell_options;$/m);
  assert.match(cliMod, /^mod native_shell_plan;$/m);
  assert.match(cliMod, /^mod native_shell_render;$/m);
  assert.match(cliMod, /^mod native_shell_templates;$/m);
  assert.match(cliMod, /^mod native_shell_validation;$/m);
  assert.match(cliMod, /^mod www_output_presence;$/m);
  assert.match(cliMod, /use self::native_shell_command::cmd_www_native_shell;/);
  assert.match(cliMod, /use self::help_text::\{[\s\S]*print_www_help[\s\S]*\};/);
  assert.match(cliMod, /use self::help_text::\{[\s\S]*print_www_native_shell_help[\s\S]*\};/);

  assert.match(cliCore, /Some\("native-shell"\) \| Some\("native"\) => cmd_www_native_shell\(&self\.cwd, &args\[1\.\.\]\)/);
  assert.match(cliCore, /print_www_help\(\)/);
  assert.doesNotMatch(cliCore, /let mut target =/);
  assert.doesNotMatch(cliCore, /native_shell_options_error/);
  assert.doesNotMatch(cliCore, /src-tauri\/tauri\.conf\.json/);

  const commandSource = readRepoFile("dx-www/src/cli/native_shell_command.rs");
  assert.ok(
    commandSource.length < 9_000,
    "native_shell_command.rs should stay a thin orchestrator, not own templates and validation",
  );
  assert.match(commandSource, /pub\(super\) fn cmd_www_native_shell\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(commandSource, /parse_native_shell_options\(cwd, args\)\?/);
  assert.match(commandSource, /materialize_native_shell/);
  assert.doesNotMatch(commandSource, /ensure_www_native_shell_for_build/);
  assert.doesNotMatch(commandSource, /\.dx\/native\/native-shell-receipt\.json/);
  assert.doesNotMatch(commandSource, /fn cargo_toml/);
  assert.doesNotMatch(commandSource, /fn tauri_config/);
  assert.doesNotMatch(commandSource, /fn validate_mobile_native_shell/);
  assert.doesNotMatch(commandSource, /Command::new\("cargo"\)/);
  assert.doesNotMatch(commandSource, /Command::new\("npm"\)/);

  const planSource = readRepoFile("dx-www/src/cli/native_shell_plan.rs");
  assert.match(planSource, /pub\(super\) fn build_native_shell_report/);
  assert.match(planSource, /\.dx\/native\/native-shell-receipt\.json/);
  assert.match(planSource, /node_modules_created/);
  assert.match(planSource, /discover_dx_native_root/);
  assert.match(planSource, /www_output_presence::dx_www_output_present\(&project_root\)/);
  assert.match(planSource, /next_commands: vec!\[/);
  assert.match(planSource, /"dx www build"\.to_string\(\)/);
  assert.match(planSource, /"dx www build --target android"\.to_string\(\)/);
  assert.doesNotMatch(planSource, /\.dx\/www\/output\/index\.html/);

  const materializerSource = readRepoFile("dx-www/src/cli/native_shell_materializer.rs");
  assert.match(materializerSource, /pub\(super\) fn materialize_native_shell/);
  assert.match(materializerSource, /pub\(super\) fn ensure_www_native_shell_for_build/);
  assert.match(materializerSource, /validate_mobile_native_shell/);
  assert.match(materializerSource, /NATIVE_SHELL_RECEIPT_PATH/);

  const namingSource = readRepoFile("dx-www/src/cli/native_shell_naming.rs");
  assert.match(namingSource, /pub\(super\) fn dx_project_name/);
  assert.match(namingSource, /pub\(super\) fn native_lib_name/);
  assert.match(namingSource, /project\(name="dx-mobile-app"\)/);
  assert.match(namingSource, /project\.name="dx-mobile-app"/);

  const androidCommandSource = readRepoFile("dx-www/src/cli/native_android_build_command.rs");
  assert.match(androidCommandSource, /native_shell_materializer::ensure_www_native_shell_for_build/);
  assert.doesNotMatch(androidCommandSource, /native_shell_command::ensure_www_native_shell_for_build/);

  const renderSource = readRepoFile("dx-www/src/cli/native_shell_render.rs");
  assert.match(renderSource, /pub\(super\) fn render_native_shell_report/);
  assert.match(renderSource, /native_shell_markdown/);
  assert.match(renderSource, /native_shell_terminal/);
  assert.match(renderSource, /append_terminal_next_commands\(&mut lines, &report\.next_commands\);/);
  assert.match(renderSource, /append_markdown_next_commands\(&mut lines, &report\.next_commands\);/);
  assert.match(renderSource, /report\.next_commands/);
  assert.match(renderSource, /Next commands:/);
  assert.match(renderSource, /## Next Commands/);
  assert.match(renderSource, /format!\("  - \{command\}"\)/);
  assert.match(renderSource, /format!\("- \{\}", markdown_inline_code\(command\)\)/);
  assert.match(renderSource, /fn markdown_inline_code/);
  assert.doesNotMatch(renderSource, /until `dx build`/);

  const templateSource = readRepoFile("dx-www/src/cli/native_shell_templates.rs");
  assert.match(templateSource, /pub\(super\) fn native_shell_files/);
  assert.match(templateSource, /src-tauri\/tauri\.conf\.json/);
  assert.match(templateSource, /src-tauri\/src\/lib\.rs/);
  assert.match(templateSource, /tauri::mobile_entry_point/);
  assert.match(templateSource, /beforeDevCommand/);
  assert.match(templateSource, /beforeBuildCommand/);
  assert.match(templateSource, /"beforeBuildCommand": "dx www build"/);

  const validationSource = readRepoFile("dx-www/src/cli/native_shell_validation.rs");
  assert.match(validationSource, /pub\(super\) fn validate_mobile_native_shell/);
  assert.match(validationSource, /pub\(super\) fn validate_www_project/);
  assert.match(validationSource, /pub\(super\) fn validate_native_root/);
  assert.match(validationSource, /toml::Value/);
  assert.doesNotMatch(validationSource, /cargo_toml\.contains/);
  assert.match(validationSource, /src-tauri\/tauri\.conf\.json/);

  const optionsSource = readRepoFile("dx-www/src/cli/native_shell_options.rs");
  assert.match(optionsSource, /pub\(super\) struct DxNativeShellCommandOptions/);
  assert.match(optionsSource, /pub\(super\) enum DxNativeShellTarget/);
  assert.match(optionsSource, /pub\(super\) fn parse_native_shell_options\(/);
  assert.match(optionsSource, /dx www native-shell requires --plan or --write/);
  assert.match(optionsSource, /dx www native-shell supports --target tauri only/);
  assert.match(optionsSource, /--force/);
  assert.match(optionsSource, /mod tests/);

  assert.match(helpText, /pub\(super\) fn print_www_help\(\)/);
  assert.match(helpText, /pub\(super\) fn print_www_native_shell_help\(\)/);
  assert.match(helpText, /dx www native-shell --target tauri --project \. --plan/);
});

test("dx-mobile-app native shell stays a real WWW route hosted by native shell", () => {
  const appRoot = resolve(repoRoot, "dx-mobile-app");
  const dxConfig = readRepoFile("dx-mobile-app/dx");
  const tauriConfig = JSON.parse(readRepoFile("dx-mobile-app/src-tauri/tauri.conf.json"));
  const tauriCargo = readRepoFile("dx-mobile-app/src-tauri/Cargo.toml");
  const tauriMain = readRepoFile("dx-mobile-app/src-tauri/src/main.rs");
  const tauriLib = readRepoFile("dx-mobile-app/src-tauri/src/lib.rs");
  const receipt = JSON.parse(readRepoFile("dx-mobile-app/.dx/native/native-shell-receipt.json"));
  const screen = readRepoFile("dx-mobile-app/app/page.tsx");
  const screenModule = readRepoFile("dx-mobile-app/components/mobile-companion/companion-screen.tsx");
  const contract = readRepoFile("dx-mobile-app/lib/mobile-companion/companion-contract.ts");
  const readiness = readRepoFile("dx-mobile-app/server/mobile-companion/readiness.ts");
  const layout = readRepoFile("dx-mobile-app/app/layout.tsx");
  const readme = readRepoFile("dx-mobile-app/README.md");

  assert.ok(existsSync(appRoot), "dx-mobile-app should be committed as a DX WWW fixture app");
  assert.match(dxConfig, /dev\(host=127\.0\.0\.1 port=3000 hot_reload=true devtools=true\)/);
  assert.equal(tauriConfig.build.devUrl, "http://127.0.0.1:3000");
  assert.equal(tauriConfig.build.frontendDist, "../.dx/www/output");
  assert.equal(tauriConfig.build.beforeDevCommand, "dx dev --host 127.0.0.1 --port 3000");
  assert.equal(tauriConfig.build.beforeBuildCommand, "dx www build");
  assert.equal(tauriConfig.build.removeUnusedCommands, true);
  assert.equal(tauriConfig.identifier, "com.essencefromexistence.dx.mobile");
  assert.match(tauriCargo, /tauri = \{ path = "\.\.\/\.\.\/\.\.\/native\/crates\/tauri"/);
  assert.match(tauriCargo, /\[lib\][\s\S]*dx_mobile_app_native_lib[\s\S]*staticlib/);
  assert.match(tauriMain, /dx_mobile_app_native_lib::run\(\)/);
  assert.match(tauriLib, /#\[tauri::command\]/);
  assert.match(tauriLib, /dx_mobile_companion_contract/);
  assert.match(tauriLib, /dx_mobile_parse_pairing_payload/);
  assert.match(tauriLib, /dx_mobile_store_device_credential/);
  assert.match(tauriLib, /tauri::mobile_entry_point/);

  assert.equal(receipt.schema, "dx.www.native_shell.receipt");
  assert.equal(receipt.target, "tauri");
  assert.equal(receipt.project.name, "dx-mobile-app");
  assert.equal(receipt.native.dev_url, "http://127.0.0.1:3000");
  assert.equal(receipt.native.frontend_dist, "../.dx/www/output");
  assert.equal(receipt.runtime_proof.native_build, false);
  assert.equal(receipt.runtime_proof.node_modules_created, false);

  assert.match(screen, /data-dx-native-bridge="dx-native-mobile-companion"/);
  assert.match(screen, /data-dx-pairing-contract="dx\.mobile\.pairing"/);
  assert.match(screen, /data-dx-chat-transport="\/ws\/chat"/);
  assert.match(screen, /data-dx-native-command="dx_mobile_scan_pairing_qr"/);
  assert.match(screen, /aria-disabled="true"/);
  assert.match(screenModule, /export \{ DxMobileCompanionScreen \} from "\.\.\/\.\.\/app\/page";/);
  assert.doesNotMatch(screen, /action="\/api\/auth\/sign-in\/email"/);
  assert.doesNotMatch(screen, /inputMode="email"|type="password"/);
  assert.doesNotMatch(screen, /href="\/signup"/);
  assert.doesNotMatch(screen, /mobile-device-frame|mobile-device-preview|mobile-device-content|phone-frame|phone-mock|mock-phone|iphone-frame|bezel|notch|status-bar|device-shell/);

  assert.match(contract, /nativeShellReceipt: "\.dx\/native\/native-shell-receipt\.json"/);
  assert.match(contract, /runtimeStatus: "source-ready"/);
  assert.match(contract, /gatewayPairEndpoint: "\/pair"/);
  assert.match(contract, /deviceRegistryPairEndpoint: "\/api\/pair"/);
  assert.match(contract, /chatWebSocket: "\/ws\/chat"/);
  assert.match(readiness, /nativeShellStatus: "source-ready"/);
  assert.match(readiness, /androidReleaseProven: false/);
  assert.match(readiness, /zedChatRuntimeProven: false/);
  assert.match(layout, /import type \{ ReactNode \} from "react"/);
  assert.doesNotMatch(layout, /children: any/);
  assert.match(readme, /dx www native-shell --target tauri --project \. --write/);
  assert.equal(existsSync(resolve(appRoot, "node_modules")), false);
});
