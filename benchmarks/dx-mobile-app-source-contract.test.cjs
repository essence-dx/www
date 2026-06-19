const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const appRoot = path.join(root, "dx-mobile-app");

function read(relativePath) {
  return fs.readFileSync(path.join(appRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

async function sha256File(filePath) {
  const hash = crypto.createHash("sha256");
  const stream = fs.createReadStream(filePath, { highWaterMark: 1024 * 1024 });

  for await (const chunk of stream) {
    hash.update(chunk);
  }

  return hash.digest("hex");
}

function readStyles(relativePath) {
  return fs
    .readdirSync(path.join(appRoot, relativePath))
    .filter((entry) => entry.endsWith(".css"))
    .sort()
    .map((entry) => read(path.join(relativePath, entry)))
    .join("\n");
}

test("DX Mobile App is a native-ready QR pairing and Zed chat companion", async () => {
  const dxConfig = read("dx");
  const readme = read("README.md");
  const sourceManifest = readJson(".dx/forge/source-manifest.json");
  const page = read("app/page.tsx");
  const screen = page;
  const screenModule = read("components/mobile-companion/companion-screen.tsx");
  const runtime = read("public/mobile-companion-runtime.js");
  const frameworkCompleteness = read("components/launch/framework-completeness.ts");
  const companionContract = read("lib/mobile-companion/companion-contract.ts");
  const readiness = read("server/mobile-companion/readiness.ts");
  const nativeShellReceipt = readJson(".dx/native/native-shell-receipt.json");
  const androidBuildReceipt = readJson(".dx/native/android-build-receipt.json");
  const tauriConfig = readJson("src-tauri/tauri.conf.json");
  const tauriMobileCapability = readJson("src-tauri/capabilities/mobile.json");
  const tauriMobilePermissions = read("src-tauri/permissions/dx-mobile-companion.toml");
  const tauriIosPlist = read("src-tauri/Info.ios.plist");
  const tauriMain = read("src-tauri/src/main.rs");
  const tauriLib = read("src-tauri/src/lib.rs");
  const tauriContract = read("src-tauri/src/contract.rs");
  const tauriCredentialStore = read("src-tauri/src/credential_store.rs");
  const tauriPairing = read("src-tauri/src/pairing.rs");
  const tauriScanner = read("src-tauri/src/scanner.rs");
  const tauriCargo = read("src-tauri/Cargo.toml");
  const css = `${read("styles/globals.css")}\n${read("styles/mobile-companion.css")}\n${readStyles("styles/mobile-companion")}`;
  const expectedCredentialPairingFields = [
    "token",
    "bearer",
    "authorization",
    "access_token",
    "refresh_token",
    "id_token",
    "client_secret",
    "api_key",
    "x_api_key",
  ];

  assert.match(dxConfig, /project\(name="dx-mobile-app" version=0\.1\.0 kind=www-app\)/);
  assert.match(dxConfig, /style\([\s\S]*tokens=styles\/theme\.css[\s\S]*generated_css=styles\/generated\.css/);
  assert.match(dxConfig, /icons\(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components\/icons\)/);
  assert.match(dxConfig, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(dxConfig, /check\(score_scale=500 lighthouse=true\)/);
  assert.match(dxConfig, /tooling\.biome\.version="2\.4\.15"/);
  assert.match(dxConfig, /tooling\.biome\.organize_imports\.enabled=true/);

  assert.match(readme, /dx www native-shell --target tauri --project \. --write/);
  assert.match(readme, /dx www build --target android/);
  assert.match(readme, /\.dx\/native\/android-build-receipt\.json/);
  assert.match(
    readme,
    /src-tauri\/gen\/android\/app\/build\/outputs\/apk\/arm64\/debug\/app-arm64-debug\.apk/,
  );
  assert.match(readme, /debug APK proof is receipt-backed/);
  assert.match(readme, /release signing, emulator install, and\s+device smoke proof remain separate/);
  assert.match(readme, /publish the native WWW artifact paths/);
  assert.match(readme, /`dx www build --target android` refresh command/);
  assert.match(readme, /identity validation is contract and readiness owned/);
  assert.match(
    readme,
    /Concrete session\s+IDs, pairing codes, bearer tokens, and agent runtime credentials/,
  );
  assert.doesNotMatch(readme, /native build, Android\/iOS release signing/);

  const packageIds = sourceManifest.packages.map((item) => item.package_id);
  assert.ok(packageIds.includes("www/minimal-starter"));
  assert.ok(packageIds.includes("auth/better-auth"));

  assert.match(page, /title: "DX Mobile App"/);
  assert.match(page, /native companion for Zed and DX Agents/);
  assert.match(page, /export function DxMobileCompanionScreen/);
  assert.match(page, /<DxMobileCompanionScreen \/>/);
  assert.doesNotMatch(page, /DxMobileAuthScreen/);
  assert.match(screenModule, /export \{ DxMobileCompanionScreen \} from "\.\.\/\.\.\/app\/page";/);

  assert.match(screen, /data-dx-mobile-app="companion"/);
  assert.match(screen, /data-dx-native-bridge="dx-native-mobile-companion"/);
  assert.match(screen, /data-dx-native-target="tauri-webview"/);
  assert.match(screen, /data-dx-native-bundle="com\.essencefromexistence\.dx\.mobile"/);
  assert.match(screen, /data-dx-style-surface="dx-mobile-companion-screen"/);
  assert.match(screen, /data-dx-check-surface="dx-mobile-companion"/);
  assert.match(screen, /data-dx-forge-package="auth\/better-auth"/);
  assert.match(screen, /data-dx-agents-gateway="dx-agents"/);
  assert.match(screen, /data-dx-pairing-contract="dx\.mobile\.pairing"/);
  assert.match(screen, /data-dx-pairing-endpoint="\/api\/pair"/);
  assert.match(screen, /data-dx-legacy-pairing-endpoint="\/pair"/);
  assert.match(screen, /data-dx-chat-transport="\/ws\/chat"/);
  assert.match(screen, /data-dx-acp-transport="\/acp"/);
  assert.match(screen, /data-dx-session-key="session_key"/);
  assert.match(screen, /data-dx-mobile-control-contract="dx\.mobile\.chat_control"/);
  assert.match(screen, /data-dx-session-sources="zed_editor,dx_cli,os_cli,vps_cli"/);
  assert.match(screen, /data-dx-chat-message-frame="mobile\.chat\.message"/);
  assert.match(screen, /dx-mobile-runtime-status/);
  assert.match(screen, /dx-mobile-pairing-form/);
  assert.match(screen, /dx-mobile-chat-list/);
  assert.match(screen, /dx-mobile-transcript/);
  assert.match(screen, /dx-mobile-message-form/);
  assert.match(screen, /dx_mobile_parse_pairing_payload/);
  assert.match(screen, /dx_mobile_store_device_credential/);
  assert.match(screen, /dx_mobile_load_device_credential/);
  assert.match(screen, /data-dx-icon="status:check"/);
  assert.match(screen, /Pair with Zed/);
  assert.match(screen, /Zed chats/);
  assert.match(screen, /DX Agents gateway/);
  assert.match(screen, /textarea[\s\S]*name="pairing_payload"/);
  assert.match(screen, /dxMobileCompanionContract\.pairingPayload\.preferredScheme/);
  assert.match(screen, /\?gateway=\.\.\.&code=\.\.\./);
  assert.match(screen, /input[\s\S]*name="gateway_origin"/);
  assert.match(screen, /input[\s\S]*name="agent_alias"/);
  assert.match(screen, /input[\s\S]*name="pairing_code"/);
  assert.match(screen, /button[\s\S]*data-dx-native-command="dx_mobile_scan_pairing_qr"/);
  assert.match(screen, /button[\s\S]*data-dx-forget-connection/);
  assert.doesNotMatch(screen, /inputMode="email"/);
  assert.doesNotMatch(screen, /type="password"/);
  assert.doesNotMatch(screen, /action="\/api\/auth\/sign-in\/email"/);
  assert.doesNotMatch(screen, /mobile-device-frame|mobile-device-preview|mobile-device-content/);

  assert.match(companionContract, /bridge: "dx-native-mobile-companion"/);
  assert.match(companionContract, /nativeShell: "tauri-webview"/);
  assert.match(companionContract, /nativeConfig: "src-tauri\/tauri\.conf\.json"/);
  assert.match(companionContract, /nativeShellReceipt: "\.dx\/native\/native-shell-receipt\.json"/);
  assert.match(companionContract, /nativeArtifacts/);
  assert.match(companionContract, /wwwOutputDirectory: "\.dx\/www\/output"/);
  assert.match(companionContract, /sourceRuntime: "public\/mobile-companion-runtime\.js"/);
  assert.match(companionContract, /packagedRuntime: "\.dx\/www\/output\/public\/mobile-companion-runtime\.js"/);
  assert.match(companionContract, /nativeEntrypoint: "\.dx\/www\/output\/index\.html"/);
  assert.match(companionContract, /androidBuildReceipt: "\.dx\/native\/android-build-receipt\.json"/);
  assert.match(
    companionContract,
    /androidDebugApk: "src-tauri\/gen\/android\/app\/build\/outputs\/apk\/arm64\/debug\/app-arm64-debug\.apk"/,
  );
  assert.match(companionContract, /refreshCommand: "dx www build --target android"/);
  assert.match(companionContract, /currentProofRequired: true/);
  assert.match(companionContract, /nativeBundleId: "com\.essencefromexistence\.dx\.mobile"/);
  assert.match(companionContract, /styleSurface: "dx-mobile-companion-screen"/);
  assert.match(companionContract, /checkSurface: "dx-mobile-companion"/);
  assert.match(companionContract, /pairingContract: "dx\.mobile\.pairing"/);
  assert.match(companionContract, /chatContract: "dx\.mobile\.chat_handoff"/);
  assert.match(companionContract, /chatControlContract: "dx\.mobile\.chat_control"/);
  assert.match(companionContract, /schema: "dx\.mobile\.chat_control\.v1"/);
  assert.match(companionContract, /sessionSources: \["zed_editor", "dx_cli", "os_cli", "vps_cli"\]/);
  assert.match(companionContract, /sendFrameType: "mobile\.chat\.message"/);
  assert.match(companionContract, /sessionEnvelope/);
  assert.match(companionContract, /messageEnvelope/);
  assert.match(companionContract, /dxMobileContractCommand: "dx mobile contract --json"/);
  assert.match(companionContract, /dxAgentChatCommand: "dx agents chat --agent <agent> --message <prompt>"/);
  assert.match(companionContract, /gatewayPairEndpoint: "\/pair"/);
  assert.match(companionContract, /deviceRegistryPairEndpoint: "\/api\/pair"/);
  assert.match(companionContract, /chatWebSocketRequiredQuery: \["agent", "session_id"\]/);
  assert.match(companionContract, /gatewayOriginPolicy/);
  assert.match(companionContract, /httpsRequiredForRemote: true/);
  assert.match(companionContract, /httpLoopbackOnly: true/);
  assert.match(companionContract, /forbidCredentials: true/);
  assert.match(companionContract, /credentialTokenPolicy/);
  assert.match(companionContract, /acceptedAuthorizationScheme: "Bearer"/);
  assert.match(companionContract, /storageShape: "raw-bearer-token"/);
  assert.match(companionContract, /trimEdgeWhitespace: true/);
  assert.match(companionContract, /stripSingleAuthorizationScheme: true/);
  assert.match(companionContract, /rejectEmptyToken: true/);
  assert.match(companionContract, /rejectWhitespaceOrControlCharacters: true/);
  assert.match(companionContract, /rejectNestedAuthorizationScheme: true/);
  assert.match(companionContract, /allowedTransports: \["Authorization", "Sec-WebSocket-Protocol"\]/);
  assert.match(
    companionContract,
    /forbiddenTransports: \["URL query", "DOM attributes", "sessionStorage", "localStorage"\]/,
  );
  assert.match(companionContract, /identityValidationPolicy/);
  assert.match(companionContract, /pairingCode: \{[\s\S]*maxLength: 128/);
  assert.match(companionContract, /sessionId: \{[\s\S]*maxLength: 128/);
  assert.match(companionContract, /sessionKey: \{[\s\S]*maxLength: 256/);
  assert.match(companionContract, /agentAlias: \{[\s\S]*maxLength: 64/);
  assert.match(companionContract, /rejectWhitespaceOrControlCharacters: true/);
  assert.match(companionContract, /domDisclosure: "field-names-only-no-live-identifiers"/);
  assert.match(companionContract, /suggestedAgentAlias: "friday"/);
  assert.match(companionContract, /sessionsEndpoint: "\/api\/sessions"/);
  assert.match(companionContract, /sessionMessagesEndpoint: "\/api\/sessions\/:session_key\/messages"/);
  assert.match(companionContract, /eventsEndpoint: "\/api\/events"/);
  assert.match(companionContract, /chatWebSocket: "\/ws\/chat"/);
  assert.match(companionContract, /acpWebSocket: "\/acp"/);
  assert.match(companionContract, /canonicalSessionKey: "session_key"/);
  assert.match(companionContract, /runtimeStatus: "source-ready"/);
  assert.match(companionContract, /rejectConflictingAliases: true/);
  assert.match(companionContract, /rejectRepeatedQueryFields: true/);
  assert.match(companionContract, /aliasGroups: \{/);
  assert.match(companionContract, /gateway: \["gateway", "gateway_origin", "origin"\]/);
  assert.match(companionContract, /code: \["code", "pairing_code"\]/);
  assert.match(companionContract, /agent: \["agent_alias", "agent"\]/);
  const contractPairingDenylist = companionContract.match(/forbiddenFields: \[([\s\S]*?)\]/);
  assert.ok(contractPairingDenylist, "DX Mobile contract must publish QR pairing forbidden fields");
  assert.deepEqual(
    Array.from(contractPairingDenylist[1].matchAll(/"([^"]+)"/g), (match) => match[1]),
    expectedCredentialPairingFields,
  );
  assert.doesNotMatch(contractPairingDenylist[1], /session[_-](?:id|key)/);
  assert.match(companionContract, /"Never encode bearer tokens in QR payloads"/);
  assert.match(companionContract, /"Store durable credentials through native secure storage only"/);
  assert.match(companionContract, /"Use Authorization or WebSocket bearer subprotocols, never query-string tokens"/);
  assert.match(companionContract, /"Route live chat through gateway WebSocket or ACP, not receipt polling"/);
  assert.match(companionContract, /"Render approval summaries only after gateway redaction"/);

  assert.match(readiness, /readDxMobileCompanionReadiness/);
  assert.match(readiness, /pairingRuntimeProven: false/);
  assert.match(readiness, /zedChatRuntimeProven: false/);
  assert.match(readiness, /nativeSecureStorageProven: false/);
  assert.match(readiness, /nativeWwwOutputCurrent: false/);
  assert.match(readiness, /nativeWwwOutputStatus: "stale-build-required"/);
  assert.match(readiness, /nativeWwwOutputRefreshCommand: nativeArtifacts\.refreshCommand/);
  assert.match(readiness, /nativeWwwOutputSourceRuntime: nativeArtifacts\.sourceRuntime/);
  assert.match(readiness, /nativeWwwOutputArtifactRuntime: nativeArtifacts\.packagedRuntime/);
  assert.match(readiness, /androidDebugApkSourceCurrent: false/);
  assert.match(readiness, /androidDebugApkStatus: "stale-source-build-required"/);
  assert.match(readiness, /androidReleaseProven: false/);
  assert.match(readiness, /requiredNativeArtifacts: nativeArtifacts/);
  assert.match(readiness, /gatewayOriginPolicy: dxMobileCompanionContract\.gatewayOriginPolicy/);
  assert.match(readiness, /credentialTokenPolicy: dxMobileCompanionContract\.credentialTokenPolicy/);
  assert.match(readiness, /pairingPayloadPolicy: dxMobileCompanionContract\.pairingPayload/);
  assert.match(readiness, /identityValidationPolicy: dxMobileCompanionContract\.identityValidationPolicy/);
  assert.match(readiness, /identityValidationRuntimeProven: true/);
  assert.match(readiness, /chatControlPolicy: dxMobileCompanionContract\.mobileControl/);

  assert.match(frameworkCompleteness, /dxMobileAppFrameworkCompleteness/);
  assert.match(frameworkCompleteness, /nativeBridge: "dx-native-mobile-companion"/);
  assert.match(frameworkCompleteness, /nativeTarget: "tauri-webview"/);
  assert.match(frameworkCompleteness, /dx-agents gateway/);
  assert.match(frameworkCompleteness, /zed-chat-companion/);
  assert.match(frameworkCompleteness, /dxMobileAppFrameworkCompletenessScore/);

  assert.equal(tauriConfig.productName, "DX Mobile App");
  assert.equal(tauriConfig.identifier, "com.essencefromexistence.dx.mobile");
  assert.equal(tauriConfig.build.devUrl, "http://127.0.0.1:3000");
  assert.equal(tauriConfig.build.frontendDist, "../.dx/www/output");
  assert.equal(tauriConfig.build.beforeDevCommand, "dx dev --host 127.0.0.1 --port 3000");
  assert.equal(tauriConfig.build.beforeBuildCommand, "dx www build");
  assert.match(tauriConfig.app.security.csp, /connect-src[^"]*wss:/);
  assert.match(tauriConfig.app.security.devCsp, /connect-src[^"]*ws:\/\/127\.0\.0\.1:\*/);
  assert.match(tauriMain, /dx_mobile_app_native_lib::run\(\)/);
  assert.match(tauriLib, /#\[tauri::command\]/);
  assert.match(tauriLib, /mod contract;/);
  assert.match(tauriLib, /mod credential_store;/);
  assert.match(tauriLib, /mod pairing;/);
  assert.match(tauriLib, /mod scanner;/);
  assert.match(tauriLib, /dx_mobile_companion_contract/);
  assert.match(tauriLib, /dx_mobile_parse_pairing_payload/);
  assert.match(tauriLib, /dx_mobile_credential_store_status/);
  assert.match(tauriLib, /dx_mobile_store_device_credential/);
  assert.match(tauriLib, /dx_mobile_load_device_credential/);
  assert.match(tauriLib, /dx_mobile_clear_device_credential/);
  assert.match(tauriLib, /tauri::mobile_entry_point/);
  assert.match(tauriLib, /#\[cfg\(mobile\)\]/);
  assert.match(tauriLib, /tauri_plugin_barcode_scanner::init\(\)/);
  assert.match(tauriContract, /NativeCompanionContract/);
  assert.match(tauriContract, /credential_store_status_command: "dx_mobile_credential_store_status"/);
  assert.match(tauriContract, /GatewayOriginPolicy/);
  assert.match(tauriContract, /gateway_origin_policy: GatewayOriginPolicy/);
  assert.match(tauriContract, /https_required_for_remote: true/);
  assert.match(tauriContract, /http_loopback_only: true/);
  assert.match(tauriContract, /forbid_credentials: true/);
  assert.match(tauriContract, /pub pairing_forbidden_fields: &'static \[&'static str\]/);
  assert.match(tauriContract, /pairing_forbidden_fields: FORBIDDEN_PAIRING_FIELD_NAMES/);
  assert.match(tauriCredentialStore, /SecureCredentialStoreStatus/);
  assert.match(tauriCredentialStore, /disabled-no-plaintext-fallback/);
  assert.match(tauriCredentialStore, /fn validate_device_credential/);
  assert.match(tauriCredentialStore, /credential\.token = normalize_gateway_bearer_token\(&credential\.token\)\?/);
  assert.match(tauriCredentialStore, /fn normalize_gateway_bearer_token/);
  assert.match(tauriCredentialStore, /device credential token must not contain whitespace or control characters/);
  assert.match(tauriCredentialStore, /let _credential = validate_device_credential\(credential\)\?/);
  assert.match(tauriPairing, /rejects bearer tokens/i);
  assert.match(tauriPairing, /const FORBIDDEN_PAIRING_FIELD_NAMES: &\[&str\] = &\[/);
  for (const field of expectedCredentialPairingFields) {
    assert.match(tauriPairing, new RegExp(`"${field}"`));
  }
  assert.match(tauriPairing, /reject_forbidden_json_fields/);
  assert.match(tauriPairing, /reject_token_bearing_url/);
  assert.match(tauriPairing, /reject_expired_pairing_payload/);
  assert.match(tauriPairing, /select_json_alias/);
  assert.match(tauriPairing, /select_query_alias/);
  assert.match(tauriPairing, /fields must not conflict/);
  assert.match(tauriPairing, /fields must not repeat/);
  assert.match(tauriPairing, /gateway origin must be HTTPS or loopback HTTP/);
  assert.match(tauriPairing, /gateway origin must not include credentials/);
  assert.doesNotMatch(tauriPairing, /\.or\(value\.pairing_code\)/);
  assert.doesNotMatch(tauriPairing, /query_pairs\(\)[\s\S]*?\.find/);
  assert.match(tauriScanner, /camera scanner adapter is not configured/);
  assert.match(tauriCargo, /\[lib\][\s\S]*dx_mobile_app_native_lib[\s\S]*staticlib/);
  assert.match(tauriCargo, /tauri = \{ path = "\.\.\/\.\.\/\.\.\/native\/crates\/tauri"/);
  assert.match(tauriCargo, /target\.'cfg\(any\(target_os = "android", target_os = "ios"\)\)'\.dependencies/);
  assert.match(tauriCargo, /tauri-plugin-barcode-scanner = "=2\.4\.5"/);
  assert.match(tauriCargo, /\[patch\.crates-io\][\s\S]*tauri = \{ path = "\.\.\/\.\.\/\.\.\/native\/crates\/tauri" \}/);
  assert.equal(tauriMobileCapability.identifier, "dx-mobile-companion");
  assert.deepEqual(tauriMobileCapability.platforms, ["iOS", "android"]);
  assert.ok(tauriMobileCapability.permissions.includes("barcode-scanner:allow-check-permissions"));
  assert.ok(tauriMobileCapability.permissions.includes("barcode-scanner:allow-request-permissions"));
  assert.ok(tauriMobileCapability.permissions.includes("barcode-scanner:allow-open-app-settings"));
  assert.ok(tauriMobileCapability.permissions.includes("barcode-scanner:allow-scan"));
  assert.ok(tauriMobileCapability.permissions.includes("barcode-scanner:allow-cancel"));
  assert.ok(tauriMobileCapability.permissions.includes("allow-dx-mobile-parse-pairing-payload"));
  assert.ok(tauriMobileCapability.permissions.includes("allow-dx-mobile-store-device-credential"));
  const nativePermissionCommands = [
    ["allow-dx-mobile-companion-contract", "dx_mobile_companion_contract"],
    ["allow-dx-mobile-credential-store-status", "dx_mobile_credential_store_status"],
    ["allow-dx-mobile-parse-pairing-payload", "dx_mobile_parse_pairing_payload"],
    ["allow-dx-mobile-store-device-credential", "dx_mobile_store_device_credential"],
    ["allow-dx-mobile-load-device-credential", "dx_mobile_load_device_credential"],
    ["allow-dx-mobile-clear-device-credential", "dx_mobile_clear_device_credential"],
    ["allow-dx-mobile-scan-pairing-qr", "dx_mobile_scan_pairing_qr"],
  ];
  for (const [permission, command] of nativePermissionCommands) {
    assert.ok(tauriMobileCapability.permissions.includes(permission));
    assert.match(
      tauriMobilePermissions,
      new RegExp(`identifier = "${permission}"[\\s\\S]*commands\\.allow = \\["${command}"\\]`),
    );
  }
  assert.match(tauriIosPlist, /NSCameraUsageDescription/);
  assert.match(tauriIosPlist, /Scan DX desktop pairing QR codes\./);

  assert.equal(nativeShellReceipt.schema, "dx.www.native_shell.receipt");
  assert.equal(nativeShellReceipt.target, "tauri");
  assert.equal(nativeShellReceipt.native.bridge, "dx-native-mobile-companion");
  assert.equal(nativeShellReceipt.native.dev_url, "http://127.0.0.1:3000");
  assert.equal(nativeShellReceipt.runtime_proof.native_build, false);
  assert.equal(nativeShellReceipt.runtime_proof.webview_run, false);
  assert.equal(nativeShellReceipt.runtime_proof.node_modules_created, false);

  assert.equal(androidBuildReceipt.schema, "dx.www.native_android_build.receipt");
  assert.equal(androidBuildReceipt.canonical_command, "dx www build --target android");
  assert.equal(androidBuildReceipt.status, "apk-built-static-proof");
  assert.equal(androidBuildReceipt.target, "android");
  assert.equal(androidBuildReceipt.build_profile, "debug");
  assert.equal(androidBuildReceipt.project.www_output_present, true);
  assert.equal(
    androidBuildReceipt.artifact.path,
    "src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk",
  );
  assert.ok(androidBuildReceipt.artifact.bytes > 0);
  assert.match(androidBuildReceipt.artifact.sha256, /^[a-f0-9]{64}$/);
  const androidApkPath = path.join(appRoot, androidBuildReceipt.artifact.path);
  assert.ok(
    fs.existsSync(androidApkPath),
    `Android build receipt artifact should exist on disk: ${androidBuildReceipt.artifact.path}`,
  );
  assert.equal(fs.statSync(androidApkPath).size, androidBuildReceipt.artifact.bytes);
  assert.equal(await sha256File(androidApkPath), androidBuildReceipt.artifact.sha256);
  assert.equal(androidBuildReceipt.proof.native_build, true);
  assert.equal(androidBuildReceipt.proof.dx_www_output_present, true);
  assert.equal(androidBuildReceipt.proof.native_www_entrypoint_present, true);
  assert.equal(androidBuildReceipt.proof.www_assets_packaged, true);
  assert.equal(androidBuildReceipt.proof.www_asset_marker_scan.status, "passed");
  assert.equal(
    androidBuildReceipt.proof.www_asset_marker_scan.details,
    `Scanned APK for ${androidBuildReceipt.proof.expected_www_assets.length} expected DX WWW asset marker(s).`,
  );
  assert.equal(androidBuildReceipt.proof.static_native_loadability_proof_complete, true);
  assert.equal(androidBuildReceipt.proof.static_apk_proof_complete, true);
  assert.equal(androidBuildReceipt.proof.expected_native_www_entrypoint, "index.html");
  for (const expectedAsset of [
    "manifest.json",
    "source-build-manifest.json",
    "source-build-receipt.json",
  ]) {
    assert.ok(
      androidBuildReceipt.proof.expected_www_assets.includes(expectedAsset),
      `Android static proof should require packaged WWW build evidence: ${expectedAsset}`,
    );
  }
  assert.deepEqual(androidBuildReceipt.proof.missing_www_assets, []);
  assert.equal(androidBuildReceipt.debug_signed, true);
  assert.equal(androidBuildReceipt.release_signed, false);
  assert.equal(androidBuildReceipt.distribution_ready, false);
  assert.equal(androidBuildReceipt.proof.device_smoke_proof_complete, false);
  assert.equal(androidBuildReceipt.proof.proof_complete, false);

  assert.match(runtime, /DXMobileCompanionRuntime/);
  assert.match(runtime, /dx_mobile_store_device_credential/);
  assert.match(runtime, /dx_mobile_load_device_credential/);
  assert.match(runtime, /dx_mobile_credential_store_status/);
  assert.match(runtime, /dx_mobile_scan_pairing_qr/);
  assert.match(runtime, /barcodeScannerApi/);
  assert.match(runtime, /window\.__TAURI__\?\.barcodeScanner/);
  assert.match(runtime, /scanner\.scan\(\{/);
  assert.match(runtime, /formats: \[scannerQrFormat\(scanner\)\]/);
  assert.match(runtime, /scanner\?\.Format\?\.QRCode \|\| "QR_CODE"/);
  assert.match(runtime, /checkPermissions/);
  assert.match(runtime, /requestPermissions/);
  assert.match(runtime, /permission-denied/);
  assert.match(runtime, /pairingState/);
  assert.match(runtime, /scannerState/);
  assert.match(runtime, /socketState/);
  assert.match(runtime, /setPairingState\(pairingState\.parsing\)/);
  assert.match(runtime, /setPairingState\(pairingState\.pairing\)/);
  assert.match(runtime, /setPairingState\(persisted \? pairingState\.pairedDurable : pairingState\.pairedSessionOnly\)/);
  assert.match(runtime, /setPairingState\(pairingState\.failed\)/);
  assert.match(runtime, /if \(event\.currentTarget !== runtimeState\.socket\) return;/);
  assert.match(runtime, /if \(runtimeState\.socketState !== socketState\.failed\) setSocketState\(socketState\.disconnected\)/);
  assert.match(runtime, /paired-session-only/);
  assert.match(runtime, /connected-live-chat/);
  assert.match(runtime, /applyScannedPairingPayload/);
  assert.match(runtime, /const preview = await parsePairingPayload\(payload\)/);
  assert.match(runtime, /textarea\.value = ""/);
  assert.doesNotMatch(runtime, /textarea\.value = payload/);
  assert.match(runtime, /PAIRING_CHALLENGE_TTL_SECONDS = 300/);
  assert.match(runtime, /pairingChallengeTtlSeconds: PAIRING_CHALLENGE_TTL_SECONDS/);
  assert.match(runtime, /PAIRING_CODE_MAX_LENGTH = 128/);
  assert.match(runtime, /SESSION_ID_MAX_LENGTH = 128/);
  assert.match(runtime, /SESSION_KEY_MAX_LENGTH = 256/);
  assert.match(runtime, /AGENT_ALIAS_MAX_LENGTH = 64/);
  assert.match(runtime, /function normalizeIdentityField/);
  assert.match(runtime, /must not contain whitespace or control characters/);
  assert.match(runtime, /rejectForbiddenPairingFields/);
  assert.match(runtime, /rejectForbiddenUrlFields/);
  assert.match(runtime, /selectObjectPairingAlias/);
  assert.match(runtime, /selectUrlPairingAlias/);
  assert.match(runtime, /rejectAmbiguousPairingPayloadAliases/);
  assert.match(runtime, /fields must not conflict/);
  assert.match(runtime, /fields must not repeat/);
  assert.match(runtime, /function rejectDecodedPairingPayloadCredentials\(raw\)/);
  assert.match(runtime, /url = new URL\(trimmed\)/);
  assert.match(runtime, /rejectForbiddenUrlFields\(url\)/);
  assert.match(
    runtime,
    /rejectDecodedPairingPayloadCredentials\(raw\);\s*rejectAmbiguousPairingPayloadAliases\(raw\);\s*const nativePreview = await invokeNative\("dx_mobile_parse_pairing_payload"/,
  );
  assert.doesNotMatch(runtime, /searchParams\.get\("code"\)\s*\|\|\s*url\.searchParams\.get\("pairing_code"\)/);
  assert.doesNotMatch(runtime, /value\.code\s*\|\|\s*value\.pairing_code/);
  assert.match(runtime, /gateway: normalizeGatewayBaseUrl\(nativePreview\.gateway_origin\)/);
  const pairingFieldDenylist = runtime.match(
    /const FORBIDDEN_PAIRING_FIELD_NAMES = Object\.freeze\(\[([\s\S]*?)\]\);/,
  );
  assert.ok(pairingFieldDenylist, "QR pairing credential denylist must stay source-owned");
  assert.deepEqual(
    Array.from(pairingFieldDenylist[1].matchAll(/"([^"]+)"/g), (match) => match[1]),
    expectedCredentialPairingFields,
  );
  assert.doesNotMatch(pairingFieldDenylist[1], /session[_-](?:id|key)/);
  assert.match(runtime, /Gateway base URL must not include credentials/);
  assert.match(runtime, /MOBILE_CAPABILITIES = Object\.freeze/);
  assert.match(runtime, /CHAT_SESSION_LIMIT = 24/);
  assert.match(runtime, /CHAT_MESSAGE_LIMIT = 80/);
  assert.match(runtime, /CHAT_MESSAGE_MAX_LENGTH = 8000/);
  assert.match(runtime, /CHAT_CONTROL_FRAME_TYPE = "mobile\.chat\.message"/);
  assert.match(runtime, /CHAT_SESSION_SOURCES = Object\.freeze\(\["zed_editor", "dx_cli", "os_cli", "vps_cli"\]\)/);
  assert.match(runtime, /function normalizeChatSession/);
  assert.match(runtime, /function normalizeChatMessage/);
  assert.match(runtime, /function appendMessage/);
  assert.match(runtime, /function buildMobileChatMessageFrame/);
  assert.match(runtime, /buildGatewayConnectFrame/);
  assert.match(runtime, /type: "connect"/);
  assert.match(runtime, /type: CHAT_CONTROL_FRAME_TYPE/);
  assert.match(runtime, /device_name: DEVICE_NAME/);
  assert.match(runtime, /capabilities: MOBILE_CAPABILITIES/);
  assert.match(runtime, /socket\.send\(JSON\.stringify\(buildGatewayConnectFrame\(sessionId\)\)\)/);
  assert.match(runtime, /data-dx-forget-connection/);
  assert.match(runtime, /agent_alias/);
  assert.match(runtime, /session_id/);
  assert.match(runtime, /session_key/);
  assert.match(runtime, /rejectExpiredPairingPayload/);
  assert.match(runtime, /normalizeGatewayBaseUrl/);
  assert.match(runtime, /gatewayEndpoint/);
  assert.match(runtime, /gatewayWebSocketEndpoint/);
  assert.match(runtime, /normalizeGatewayBaseUrl\(result\.credential\.gateway_origin\)/);
  assert.doesNotMatch(runtime, /parsed\.pathname = ""/);
  assert.match(runtime, /buildPairingRequestBody/);
  assert.match(runtime, /body\.session_id = sessionId/);
  assert.match(runtime, /body\.session_key = sessionKey/);
  assert.match(runtime, /body: JSON\.stringify\(buildPairingRequestBody\(code, context\)\)/);
  assert.match(runtime, /pairingFailureMessage\(response\.status\)/);
  assert.doesNotMatch(runtime, /Pairing failed \(\$\{response\.status\}\):/);
  assert.match(runtime, /function sanitizeStatusMessage\(message\)/);
  assert.match(runtime, /output\.textContent = sanitizeStatusMessage\(message\)/);
  assert.match(runtime, /STATUS_REDACTION_FIELDS/);
  assert.match(runtime, /STATUS_BEARER_FIELD_PATTERN/);
  assert.match(runtime, /STATUS_REDACTED = "\[redacted\]"/);
  assert.match(runtime, /"session_key"/);
  assert.match(runtime, /"pairing_code"/);
  assert.match(runtime, /pairWithGateway\(origin, code, \{/);
  assert.match(runtime, /sessionIdFromPairingPreview/);
  assert.match(runtime, /sessionKey\.startsWith\("gw_"\)/);
  assert.match(runtime, /runtimeState\.activeSessionId = sessionIdFromPairingPreview\(preview\)/);
  assert.match(
    companionContract,
    /Keep concrete session_id and session_key values out of public DOM attributes and visible labels/,
  );
  assert.doesNotMatch(runtime, /dataset\.dxSessionId\s*=/);
  assert.doesNotMatch(runtime, /dataset\.dxSessionKey\s*=/);
  assert.match(runtime, /Sec-WebSocket-Protocol/);
  assert.match(runtime, /function normalizeGatewayBearerToken\(token\)/);
  assert.match(runtime, /Gateway bearer token must not contain whitespace or control characters/);
  assert.match(runtime, /function normalizeDeviceCredential\(credential\)/);
  assert.match(runtime, /token: normalizeGatewayBearerToken\(credential\?\.token\)/);
  assert.match(runtime, /const normalizedCredential = normalizeDeviceCredential\(credential\)/);
  assert.match(runtime, /const token = normalizeGatewayBearerToken\(paired\.token\)/);
  assert.match(runtime, /Authorization: `Bearer \$\{token\}`/);
  assert.match(runtime, /bearer\.\$\{token\}/);
  assert.doesNotMatch(runtime, /Authorization: `Bearer \$\{credential\.token\}`/);
  assert.doesNotMatch(runtime, /bearer\.\$\{credential\.token\}/);
  assert.match(runtime, /ALLOWED_WEBSOCKET_QUERY_FIELD_NAMES/);
  const webSocketQueryAllowlist = runtime.match(
    /const ALLOWED_WEBSOCKET_QUERY_FIELD_NAMES = Object\.freeze\(\[([\s\S]*?)\]\);/,
  );
  assert.ok(webSocketQueryAllowlist, "WebSocket query allowlist must stay source-owned");
  assert.deepEqual(
    Array.from(webSocketQueryAllowlist[1].matchAll(/"([^"]+)"/g), (match) => match[1]),
    ["agent", "session_id"],
  );
  const tokenFreeWebSocketQuerySource = runtime.match(
    /function tokenFreeWebSocketQuery\(query\) \{([\s\S]*?)\n  \}/,
  );
  assert.ok(tokenFreeWebSocketQuerySource, "WebSocket query sanitizer must stay source-owned");
  assert.match(
    tokenFreeWebSocketQuerySource[1],
    /if \(!ALLOWED_WEBSOCKET_QUERY_FIELD_NAMES\.includes\(field\)\) \{\s*throw new Error\("WebSocket query must only include agent and session_id"\);\s*\}/,
  );
  assert.match(runtime, /FORBIDDEN_WEBSOCKET_QUERY_FIELD_NAMES/);
  const webSocketQueryDenylist = runtime.match(
    /const FORBIDDEN_WEBSOCKET_QUERY_FIELD_NAMES = Object\.freeze\(\[([\s\S]*?)\]\);/,
  );
  assert.ok(webSocketQueryDenylist, "WebSocket query credential denylist must stay source-owned");
  assert.deepEqual(
    Array.from(webSocketQueryDenylist[1].matchAll(/"([^"]+)"/g), (match) => match[1]),
    [
      "token",
      "bearer",
      "authorization",
      "access_token",
      "access-token",
      "refresh_token",
      "refresh-token",
      "id_token",
      "id-token",
      "client_secret",
      "client-secret",
      "api_key",
      "api-key",
      "x_api_key",
      "x-api-key",
    ],
  );
  assert.doesNotMatch(webSocketQueryDenylist[1], /session[_-](?:id|key)/);
  assert.match(runtime, /function tokenFreeWebSocketQuery\(query\)/);
  assert.match(runtime, /const normalizedQuery = tokenFreeWebSocketQuery\(query\)/);
  assert.match(runtime, /WebSocket query must not include bearer credentials/);
  const connectSocketSource = runtime.match(/function connectSocket\(credential, agentAlias\) \{([\s\S]*?)\n  \}/);
  assert.ok(connectSocketSource, "connectSocket should stay source-owned");
  assert.deepEqual(
    Array.from(connectSocketSource[1].matchAll(/params\.set\(["'`]([^"'`]+)["'`]/g), (match) => match[1]),
    ["session_id", "agent"],
  );
  assert.match(runtime, /deviceRegistryPairEndpoint/);
  assert.match(runtime, /deviceRegistryPairEndpoint = "\/api\/pair"/);
  assert.match(runtime, /fetch\(gatewayEndpoint\(gatewayOrigin, deviceRegistryPairEndpoint\)/);
  assert.match(runtime, /device_name: DEVICE_NAME/);
  assert.match(runtime, /device_type: "mobile"/);
  assert.doesNotMatch(runtime, /fetch\(`\$\{gatewayOrigin\}\/pair`[\s\S]*X-Pairing-Code/);
  assert.match(runtime, /\/api\/sessions/);
  assert.match(runtime, /\/api\/devices\/me\/capabilities/);
  assert.match(runtime, /Rejecting QR payload that contains a bearer token/);
  assert.match(runtime, /sessionStorage/);
  for (const key of ["token", "access_token", "authorization", "bearer"]) {
    assert.doesNotMatch(runtime, new RegExp(`params\\.set\\(["'\`]${key}["'\`]`, "i"));
    assert.doesNotMatch(runtime, new RegExp(`sessionStorage\\.setItem\\([^)]*${key}`, "i"));
    assert.doesNotMatch(runtime, new RegExp(`localStorage\\.setItem\\([^)]*${key}`, "i"));
  }
  assert.doesNotMatch(runtime, /params\.set\([^)]*credential\.token/);
  assert.doesNotMatch(runtime, /new WebSocket\([^,]*credential\.token/);
  assert.match(
    runtime,
    /new WebSocket\(\s*gatewayWebSocketEndpoint\(credential\.gateway_origin, "\/ws\/chat", params\.toString\(\)\),\s*protocols,\s*\)/,
  );
  assert.doesNotMatch(runtime, /localStorage\.setItem\([^)]*token/i);
  assert.doesNotMatch(runtime, /token=.*URLSearchParams/i);

  assert.match(css, /\.native-companion-shell/);
  assert.doesNotMatch(css, /mobile-device-frame|mobile-device-preview|mobile-device-content/);
  assert.match(css, /@import "\.\/mobile-companion\.css"/);
  assert.match(css, /@media \(max-width: 860px\)/);
  assert.match(css, /@media \(max-width: 560px\)/);
  assert.match(css, /min-height: 100dvh/);
  assert.match(css, /overflow-x: hidden/);
  assert.match(css, /box-sizing: border-box/);
  assert.match(css, /width: calc\(100vw - 20px\)/);
  assert.match(css, /\.companion-primary-action/);
  assert.match(css, /\.companion-chat-shell/);

  assert.equal(fs.existsSync(path.join(appRoot, "node_modules")), false);
});
