use std::{fs::File, io::Read, path::Path};

use chrono::Utc;
use serde::Serialize;
use sha2::Digest as _;

use crate::error::{DxError, DxResult};

use super::native_android_build_environment::{
    AndroidBuildEnvironment, environment_path_for_receipt,
};
use super::native_android_build_plan::{
    ARM64_DEBUG_APK_PATH, AndroidBuildPlan, native_lib_name, read_project_name,
    read_tauri_config_value, relative_project_path, slash_path,
};
use super::native_android_build_process::{
    AndroidCommandProbe, adb_devices_command, apksigner_verify_command, jar_list_command,
    run_optional_android_command,
};
use super::www_output_presence::dx_www_output_present;

#[derive(Debug, Serialize)]
pub(super) struct AndroidBuildReceipt {
    schema: &'static str,
    canonical_command: &'static str,
    invoked_as: &'static str,
    status: &'static str,
    target: &'static str,
    build_profile: &'static str,
    distribution_ready: bool,
    release_signed: bool,
    debug_signed: bool,
    created_at: String,
    project: AndroidReceiptProject,
    native: AndroidReceiptNative,
    steps: Vec<AndroidBuildStepReceipt>,
    environment: AndroidEnvironmentReceipt,
    artifact: AndroidArtifactReceipt,
    proof: AndroidProofReceipt,
}

#[derive(Debug, Serialize)]
struct AndroidReceiptProject {
    name: String,
    root: String,
    dx_config: &'static str,
    www_output: &'static str,
    www_output_present: bool,
}

#[derive(Debug, Serialize)]
struct AndroidReceiptNative {
    shell: &'static str,
    target: &'static str,
    abi: &'static str,
    rust_target: &'static str,
    package_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct AndroidEnvironmentReceipt {
    cache_root: String,
    gradle_user_home: String,
    android_user_home: String,
    android_avd_home: String,
    cargo_home: String,
    cargo_target_dir: String,
    temp_dir: String,
    required_free_bytes: u64,
    available_free_bytes: Option<u64>,
    free_space_status: &'static str,
    writable: bool,
}

#[derive(Debug, Serialize)]
pub(super) struct AndroidBuildStepReceipt {
    pub(super) name: &'static str,
    pub(super) status: &'static str,
    pub(super) command: Option<String>,
    pub(super) details: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct AndroidArtifactReceipt {
    path: String,
    bytes: u64,
    sha256: String,
    modified_at: Option<String>,
    variant: &'static str,
    version: Option<String>,
}

#[derive(Debug, Serialize)]
struct AndroidProofReceipt {
    dx_www_output_present: bool,
    native_build: bool,
    native_library_packaged: Option<bool>,
    tauri_config_packaged: Option<bool>,
    android_manifest_packaged: Option<bool>,
    dex_bytecode_packaged: Option<bool>,
    native_www_entrypoint_present: bool,
    www_assets_packaged: Option<bool>,
    www_asset_proof_mode: &'static str,
    www_asset_marker_limit: usize,
    www_asset_markers_truncated: bool,
    expected_native_library: String,
    expected_tauri_config: &'static str,
    expected_android_manifest: &'static str,
    expected_dex_bytecode: &'static str,
    expected_native_www_entrypoint: &'static str,
    expected_www_assets: Vec<String>,
    missing_www_assets: Vec<String>,
    www_asset_marker_scan: AndroidProofCommandReceipt,
    apk_signature_scheme_v2: Option<bool>,
    signature_verification: AndroidProofCommandReceipt,
    apk_contents: AndroidProofCommandReceipt,
    adb_devices_connected: bool,
    adb_probe: AndroidProofCommandReceipt,
    installed_on_device: bool,
    static_native_loadability_proof_complete: bool,
    static_apk_proof_complete: bool,
    device_smoke_proof_complete: bool,
    proof_complete: bool,
}

#[derive(Debug, Serialize)]
struct AndroidProofCommandReceipt {
    status: &'static str,
    command: Option<String>,
    details: Option<String>,
}

pub(super) fn inspect_android_artifact(
    plan: &AndroidBuildPlan,
) -> DxResult<AndroidArtifactReceipt> {
    let metadata = std::fs::metadata(&plan.apk_path).map_err(|error| DxError::BuildFailed {
        message: format!(
            "Tauri Android build completed but expected APK was not found at {}: {error}",
            relative_project_path(&plan.project_root, &plan.apk_path)
        ),
    })?;

    Ok(AndroidArtifactReceipt {
        path: relative_project_path(&plan.project_root, &plan.apk_path),
        bytes: metadata.len(),
        sha256: apk_sha256(&plan.apk_path)?,
        modified_at: metadata
            .modified()
            .ok()
            .map(|modified| chrono::DateTime::<Utc>::from(modified).to_rfc3339()),
        variant: "arm64Debug",
        version: read_tauri_config_value(&plan.src_tauri_root, "version"),
    })
}

pub(super) fn build_android_receipt(
    plan: &AndroidBuildPlan,
    invoked_as: &'static str,
    steps: Vec<AndroidBuildStepReceipt>,
    artifact: AndroidArtifactReceipt,
    environment: &AndroidBuildEnvironment,
) -> DxResult<AndroidBuildReceipt> {
    let project_name = read_project_name(&plan.project_root)?;
    let www_output_present = dx_www_output_present(&plan.project_root);
    let expected_native_library = format!("lib/arm64-v8a/lib{}.so", native_lib_name(&project_name));
    let proof = collect_android_proof(
        plan,
        www_output_present,
        &expected_native_library,
        environment,
    );
    let debug_signed = proof.apk_signature_scheme_v2 == Some(true);
    let status = if proof.static_apk_proof_complete {
        "apk-built-static-proof"
    } else {
        "apk-built-proof-partial"
    };

    Ok(AndroidBuildReceipt {
        schema: "dx.www.native_android_build.receipt",
        canonical_command: "dx www build --target android",
        invoked_as,
        status,
        target: "android",
        build_profile: "debug",
        distribution_ready: false,
        release_signed: false,
        debug_signed,
        created_at: Utc::now().to_rfc3339(),
        project: AndroidReceiptProject {
            name: project_name,
            root: slash_path(&plan.project_root),
            dx_config: "dx",
            www_output: ".dx/www/output",
            www_output_present,
        },
        native: AndroidReceiptNative {
            shell: "tauri-webview",
            target: "android",
            abi: "arm64-v8a",
            rust_target: "aarch64-linux-android",
            package_id: read_tauri_config_value(&plan.src_tauri_root, "identifier"),
        },
        steps,
        environment: android_environment_receipt(plan, environment),
        artifact,
        proof,
    })
}

fn android_environment_receipt(
    plan: &AndroidBuildPlan,
    environment: &AndroidBuildEnvironment,
) -> AndroidEnvironmentReceipt {
    AndroidEnvironmentReceipt {
        cache_root: environment_path_for_receipt(&plan.project_root, &environment.cache_root),
        gradle_user_home: environment_path_for_receipt(
            &plan.project_root,
            &environment.gradle_user_home,
        ),
        android_user_home: environment_path_for_receipt(
            &plan.project_root,
            &environment.android_user_home,
        ),
        android_avd_home: environment_path_for_receipt(
            &plan.project_root,
            &environment.android_avd_home,
        ),
        cargo_home: environment_path_for_receipt(&plan.project_root, &environment.cargo_home),
        cargo_target_dir: environment_path_for_receipt(
            &plan.project_root,
            &environment.cargo_target_dir,
        ),
        temp_dir: environment_path_for_receipt(&plan.project_root, &environment.temp_dir),
        required_free_bytes: environment.required_free_bytes,
        available_free_bytes: environment.available_free_bytes,
        free_space_status: environment.free_space_status,
        writable: environment.writable,
    }
}

pub(super) fn write_android_receipt(path: &Path, receipt: &AndroidBuildReceipt) -> DxResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json =
        serde_json::to_string_pretty(receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to serialize Android build receipt: {error}"),
            field: Some("build.target.android.receipt".to_string()),
        })?;
    std::fs::write(path, format!("{json}\n")).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn collect_android_proof(
    plan: &AndroidBuildPlan,
    www_output_present: bool,
    expected_native_library: &str,
    environment: &AndroidBuildEnvironment,
) -> AndroidProofReceipt {
    let expected_www_asset_plan = expected_www_asset_marker_plan(&plan.project_root);
    let www_asset_markers_truncated = expected_www_asset_plan.truncated;
    let expected_www_assets = expected_www_asset_plan.markers;
    let content_entries = android_static_apk_proof_entries(expected_native_library);
    let content_entry_refs = content_entries
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let signature_probe = run_optional_android_command(
        &plan.project_root,
        apksigner_verify_command(ARM64_DEBUG_APK_PATH),
        environment,
    );
    let content_probe = run_optional_android_command(
        &plan.project_root,
        jar_list_command(ARM64_DEBUG_APK_PATH, &content_entry_refs),
        environment,
    );
    let adb_probe =
        run_optional_android_command(&plan.project_root, adb_devices_command(), environment);

    let apk_entries = if content_probe.status == "passed" {
        Some(content_probe.stdout.lines().collect::<Vec<_>>())
    } else {
        None
    };
    let apk_entry_slice = apk_entries.as_deref();
    let native_library_packaged =
        static_apk_entry_packaged(apk_entry_slice, expected_native_library);
    let tauri_config_packaged =
        static_apk_entry_packaged(apk_entry_slice, ANDROID_TAURI_CONFIG_ENTRY);
    let android_manifest_packaged =
        static_apk_entry_packaged(apk_entry_slice, ANDROID_MANIFEST_ENTRY);
    let dex_bytecode_packaged =
        static_apk_entry_packaged(apk_entry_slice, ANDROID_DEX_BYTECODE_ENTRY);
    let native_www_entrypoint_present = dx_www_native_entrypoint_present(&plan.project_root);
    let WwwAssetMarkerScanReceipt {
        status: www_asset_marker_scan_status,
        details: www_asset_marker_scan_details,
        missing_assets: scanned_missing_www_assets,
    } = www_asset_marker_scan_receipt(&plan.apk_path, &expected_www_assets);
    let missing_www_assets =
        scanned_missing_www_assets.unwrap_or_else(|| expected_www_assets.clone());
    let www_assets_packaged = if expected_www_assets.is_empty() {
        None
    } else {
        Some(
            !www_asset_markers_truncated
                && missing_www_assets.is_empty()
                && www_asset_marker_scan_status == "passed",
        )
    };

    let apk_signature_scheme_v2 = parse_apksigner_v2_result(&signature_probe.stdout);
    let adb_devices_connected = adb_devices_connected(&adb_probe.stdout);
    let installed_on_device = false;
    let static_native_loadability_proof_complete = native_library_packaged == Some(true)
        && tauri_config_packaged == Some(true)
        && android_manifest_packaged == Some(true)
        && dex_bytecode_packaged == Some(true);
    let static_apk_proof_complete = www_output_present
        && native_www_entrypoint_present
        && static_native_loadability_proof_complete
        && www_assets_packaged == Some(true)
        && apk_signature_scheme_v2 == Some(true);
    let device_smoke_proof_complete = adb_devices_connected && installed_on_device;

    AndroidProofReceipt {
        dx_www_output_present: www_output_present,
        native_build: true,
        native_library_packaged,
        tauri_config_packaged,
        android_manifest_packaged,
        dex_bytecode_packaged,
        native_www_entrypoint_present,
        www_assets_packaged,
        www_asset_proof_mode: "apk-binary-marker",
        www_asset_marker_limit: WWW_ASSET_MARKER_PROOF_LIMIT,
        www_asset_markers_truncated,
        expected_native_library: expected_native_library.to_string(),
        expected_tauri_config: ANDROID_TAURI_CONFIG_ENTRY,
        expected_android_manifest: ANDROID_MANIFEST_ENTRY,
        expected_dex_bytecode: ANDROID_DEX_BYTECODE_ENTRY,
        expected_native_www_entrypoint: NATIVE_WWW_ENTRYPOINT,
        expected_www_assets,
        missing_www_assets,
        www_asset_marker_scan: AndroidProofCommandReceipt {
            status: www_asset_marker_scan_status,
            command: None,
            details: www_asset_marker_scan_details,
        },
        apk_signature_scheme_v2,
        signature_verification: proof_command_receipt(&signature_probe),
        apk_contents: proof_command_receipt(&content_probe),
        adb_devices_connected,
        adb_probe: proof_command_receipt(&adb_probe),
        installed_on_device,
        static_native_loadability_proof_complete,
        static_apk_proof_complete,
        device_smoke_proof_complete,
        proof_complete: static_apk_proof_complete && device_smoke_proof_complete,
    }
}

fn proof_command_receipt(probe: &AndroidCommandProbe) -> AndroidProofCommandReceipt {
    AndroidProofCommandReceipt {
        status: probe.status,
        command: probe.command.clone(),
        details: proof_details(probe),
    }
}

fn proof_details(probe: &AndroidCommandProbe) -> Option<String> {
    let text = if probe.status == "passed" {
        probe.stdout.trim()
    } else {
        probe.stderr.trim()
    };
    if text.is_empty() {
        None
    } else {
        Some(text.chars().take(1_000).collect())
    }
}

const NATIVE_WWW_ENTRYPOINT: &str = "index.html";
const ANDROID_TAURI_CONFIG_ENTRY: &str = "assets/tauri.conf.json";
const ANDROID_MANIFEST_ENTRY: &str = "AndroidManifest.xml";
const ANDROID_DEX_BYTECODE_ENTRY: &str = "classes.dex";
const WWW_ASSET_MARKER_PROOF_LIMIT: usize = 128;
const WWW_BUILD_EVIDENCE_MARKERS: &[&str] = &[
    "manifest.json",
    "source-build-manifest.json",
    "source-build-receipt.json",
];

struct WwwAssetMarkerPlan {
    markers: Vec<String>,
    truncated: bool,
}

fn expected_www_asset_marker_plan(project_root: &Path) -> WwwAssetMarkerPlan {
    let output_root = project_root.join(".dx/www/output");
    let native_entrypoint_present = dx_www_native_entrypoint_present(project_root);
    if !native_entrypoint_present && !www_output_requires_native_entrypoint(&output_root) {
        return WwwAssetMarkerPlan {
            markers: Vec::new(),
            truncated: false,
        };
    }

    let mut markers = Vec::new();
    collect_www_asset_markers(&output_root, &output_root, &mut markers);
    if !native_entrypoint_present {
        markers.push(NATIVE_WWW_ENTRYPOINT.to_string());
    }
    markers.sort();
    markers.dedup();

    let truncated = markers.len() > WWW_ASSET_MARKER_PROOF_LIMIT;
    markers.truncate(WWW_ASSET_MARKER_PROOF_LIMIT);

    WwwAssetMarkerPlan { markers, truncated }
}

fn expected_www_asset_markers(project_root: &Path) -> Vec<String> {
    expected_www_asset_marker_plan(project_root).markers
}

fn dx_www_native_entrypoint_present(project_root: &Path) -> bool {
    project_root
        .join(".dx/www/output")
        .join(NATIVE_WWW_ENTRYPOINT)
        .is_file()
}

fn www_output_requires_native_entrypoint(output_root: &Path) -> bool {
    ["app/index.html", "source-routes/root/index.html"]
        .iter()
        .any(|relative_path| output_root.join(relative_path).is_file())
}

fn collect_www_asset_markers(output_root: &Path, current: &Path, markers: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(current) else {
        return;
    };

    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();

        if file_type.is_dir() {
            collect_www_asset_markers(output_root, &path, markers);
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let Ok(relative_path) = path.strip_prefix(output_root) else {
            continue;
        };
        let marker = relative_path.to_string_lossy().replace('\\', "/");
        if is_native_runtime_asset_marker(&marker) {
            markers.push(marker);
        }
    }
}

fn is_native_runtime_asset_marker(marker: &str) -> bool {
    if marker.is_empty() {
        return false;
    }

    let lowercase = marker.to_ascii_lowercase();
    if !marker.contains('/') && lowercase.ends_with(".json") {
        return is_www_build_evidence_marker(marker);
    }

    [
        ".html", ".js", ".css", ".json", ".svg", ".png", ".jpg", ".jpeg", ".webp", ".gif", ".ico",
        ".woff", ".woff2", ".ttf", ".otf",
    ]
    .iter()
    .any(|extension| lowercase.ends_with(extension))
}

fn is_www_build_evidence_marker(marker: &str) -> bool {
    WWW_BUILD_EVIDENCE_MARKERS.contains(&marker)
}

fn android_static_apk_proof_entries(expected_native_library: &str) -> Vec<String> {
    vec![
        expected_native_library.to_string(),
        ANDROID_TAURI_CONFIG_ENTRY.to_string(),
        ANDROID_MANIFEST_ENTRY.to_string(),
        ANDROID_DEX_BYTECODE_ENTRY.to_string(),
    ]
}

fn static_apk_entry_packaged(apk_entries: Option<&[&str]>, entry: &str) -> Option<bool> {
    apk_entries.map(|entries| entries.contains(&entry))
}

struct WwwAssetMarkerScanReceipt {
    status: &'static str,
    details: Option<String>,
    missing_assets: Option<Vec<String>>,
}

fn www_asset_marker_scan_receipt(
    apk_path: &Path,
    expected_www_assets: &[String],
) -> WwwAssetMarkerScanReceipt {
    if expected_www_assets.is_empty() {
        return WwwAssetMarkerScanReceipt {
            status: "skipped",
            details: Some("No DX WWW output asset markers were expected.".to_string()),
            missing_assets: Some(Vec::new()),
        };
    }

    match missing_expected_www_asset_markers(apk_path, expected_www_assets) {
        Ok(missing_assets) if missing_assets.is_empty() => WwwAssetMarkerScanReceipt {
            status: "passed",
            details: Some(format!(
                "Scanned APK for {} expected DX WWW asset marker(s).",
                expected_www_assets.len()
            )),
            missing_assets: Some(missing_assets),
        },
        Ok(missing_assets) => WwwAssetMarkerScanReceipt {
            status: "failed",
            details: Some(format!(
                "Missing {} of {} expected DX WWW asset marker(s).",
                missing_assets.len(),
                expected_www_assets.len()
            )),
            missing_assets: Some(missing_assets),
        },
        Err(error) => WwwAssetMarkerScanReceipt {
            status: "failed",
            details: Some(format!(
                "Failed to scan APK for DX WWW asset markers: {error}"
            )),
            missing_assets: None,
        },
    }
}

const APK_MARKER_SCAN_CHUNK_BYTES: usize = 64 * 1024;

fn missing_expected_www_asset_markers(
    apk_path: &Path,
    expected_www_assets: &[String],
) -> std::io::Result<Vec<String>> {
    let candidates = expected_www_assets
        .iter()
        .map(|asset| www_asset_marker_candidates(asset))
        .collect::<Vec<_>>();
    let max_marker_len = candidates.iter().flatten().map(Vec::len).max().unwrap_or(0);
    let carry_bytes = max_marker_len.saturating_sub(1);
    let mut file = File::open(apk_path)?;
    let mut buffer = [0_u8; APK_MARKER_SCAN_CHUNK_BYTES];
    let mut carry = Vec::new();
    let mut found = vec![false; expected_www_assets.len()];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let mut window = Vec::with_capacity(carry.len() + bytes_read);
        window.extend_from_slice(&carry);
        window.extend_from_slice(&buffer[..bytes_read]);

        for (index, asset_candidates) in candidates.iter().enumerate() {
            if found[index] {
                continue;
            }

            found[index] = asset_candidates
                .iter()
                .any(|candidate| contains_subslice(&window, candidate));
        }

        if found.iter().all(|is_found| *is_found) {
            break;
        }

        let keep = window.len().min(carry_bytes);
        carry.clear();
        if keep > 0 {
            carry.extend_from_slice(&window[window.len() - keep..]);
        }
    }

    Ok(expected_www_assets
        .iter()
        .zip(found)
        .filter_map(
            |(asset, is_found)| {
                if is_found { None } else { Some(asset.clone()) }
            },
        )
        .collect())
}

fn www_asset_marker_candidates(asset: &str) -> Vec<Vec<u8>> {
    if asset == "index.html" {
        return vec![asset.as_bytes().to_vec()];
    }

    vec![format!("/{asset}").into_bytes(), asset.as_bytes().to_vec()]
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }

    if needle.len() > haystack.len() {
        return false;
    }

    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn parse_apksigner_v2_result(output: &str) -> Option<bool> {
    output.lines().find_map(|line| {
        let line = line.trim();
        if line.starts_with("Verified using v2 scheme") {
            Some(line.ends_with("true"))
        } else {
            None
        }
    })
}

fn adb_devices_connected(output: &str) -> bool {
    output
        .lines()
        .skip(1)
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('*'))
        .any(|line| {
            let mut columns = line.split_whitespace();
            let _serial = columns.next();
            if let Some(state) = columns.next() {
                return state == "device";
            }
            false
        })
}

fn apk_sha256(path: &Path) -> DxResult<String> {
    let mut file = std::fs::File::open(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;
    let mut hasher = sha2::Sha256::new();
    std::io::copy(&mut file, &mut hasher).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;
    let digest = hasher.finalize();
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_apksigner_v2_status_from_verbose_output() {
        let output = "Verifies\nVerified using v2 scheme (APK Signature Scheme v2): true\n";

        assert_eq!(parse_apksigner_v2_result(output), Some(true));
    }

    #[test]
    fn detects_connected_adb_devices_from_devices_output() {
        let no_devices = "List of devices attached\n\n";
        let one_device = "List of devices attached\nemulator-5554\tdevice\n";
        let offline = "List of devices attached\nemulator-5554\toffline\n";
        let unauthorized = "List of devices attached\nemulator-5554\tunauthorized\n";

        assert!(!adb_devices_connected(no_devices));
        assert!(adb_devices_connected(one_device));
        assert!(!adb_devices_connected(offline));
        assert!(!adb_devices_connected(unauthorized));
    }

    #[test]
    fn android_receipt_keeps_jar_probe_to_static_apk_entries() {
        let entries =
            android_static_apk_proof_entries("lib/arm64-v8a/libdx_mobile_app_native_lib.so");

        assert_eq!(
            entries,
            vec![
                "lib/arm64-v8a/libdx_mobile_app_native_lib.so".to_string(),
                ANDROID_TAURI_CONFIG_ENTRY.to_string(),
                ANDROID_MANIFEST_ENTRY.to_string(),
                ANDROID_DEX_BYTECODE_ENTRY.to_string(),
            ]
        );
    }

    #[test]
    fn android_receipt_maps_static_apk_entry_presence_from_probe_output() {
        let entries = ["AndroidManifest.xml", "assets/tauri.conf.json"];

        assert_eq!(
            static_apk_entry_packaged(Some(&entries), ANDROID_MANIFEST_ENTRY),
            Some(true)
        );
        assert_eq!(
            static_apk_entry_packaged(Some(&entries), ANDROID_DEX_BYTECODE_ENTRY),
            Some(false)
        );
        assert_eq!(
            static_apk_entry_packaged(None, ANDROID_MANIFEST_ENTRY),
            None
        );
    }

    #[test]
    fn android_receipt_reports_missing_www_asset_markers() {
        let expected_www_assets = vec![
            "app/index.html".to_string(),
            "source-routes/root/index.html".to_string(),
            "public/mobile-companion-runtime.js".to_string(),
        ];
        let apk = tempfile::NamedTempFile::new().expect("apk");
        std::fs::write(apk.path(), b"lib/arm64-v8a/libdx_mobile_app_native_lib.so")
            .expect("apk contents");

        assert_eq!(
            missing_expected_www_asset_markers(apk.path(), &expected_www_assets).expect("scan"),
            vec![
                "app/index.html".to_string(),
                "source-routes/root/index.html".to_string(),
                "public/mobile-companion-runtime.js".to_string(),
            ]
        );
    }

    #[test]
    fn android_receipt_detects_embedded_www_asset_markers_across_scan_chunks() {
        let expected_www_assets = vec!["app/index.html".to_string()];
        let apk = tempfile::NamedTempFile::new().expect("apk");
        let mut bytes = vec![b'x'; APK_MARKER_SCAN_CHUNK_BYTES - 2];
        bytes.extend_from_slice(b"/app/index.html");
        std::fs::write(apk.path(), bytes).expect("apk contents");

        assert_eq!(
            missing_expected_www_asset_markers(apk.path(), &expected_www_assets).expect("scan"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn android_receipt_collects_nested_www_output_markers_deterministically() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");
        write_www_output(project.path(), "styles/generated.css");
        write_www_output(project.path(), "app/index.html");
        write_www_output(project.path(), "source-routes/root/index.html");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec![
                "app/index.html".to_string(),
                "index.html".to_string(),
                "source-routes/root/index.html".to_string(),
                "styles/generated.css".to_string(),
            ]
        );
    }

    #[test]
    fn android_receipt_includes_www_build_evidence_markers() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");
        write_www_output(project.path(), "manifest.json");
        write_www_output(project.path(), "source-build-manifest.json");
        write_www_output(project.path(), "source-build-receipt.json");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec![
                "index.html".to_string(),
                "manifest.json".to_string(),
                "source-build-manifest.json".to_string(),
                "source-build-receipt.json".to_string(),
            ]
        );
    }

    #[test]
    fn android_receipt_caps_large_www_asset_marker_plan() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");
        for index in 0..=WWW_ASSET_MARKER_PROOF_LIMIT {
            write_www_output(project.path(), &format!("chunks/{index:03}.js"));
        }

        let plan = expected_www_asset_marker_plan(project.path());

        assert!(plan.truncated);
        assert_eq!(plan.markers.len(), WWW_ASSET_MARKER_PROOF_LIMIT);
    }

    #[test]
    fn android_receipt_requires_loadable_www_entrypoint_not_manifest_only() {
        let manifest_only = tempfile::tempdir().expect("tempdir");
        write_www_output(manifest_only.path(), "manifest.json");

        assert_eq!(
            expected_www_asset_markers(manifest_only.path()),
            Vec::<String>::new()
        );

        let native_ready = tempfile::tempdir().expect("tempdir");
        write_www_output(native_ready.path(), "index.html");
        write_www_output(native_ready.path(), "manifest.json");

        assert_eq!(
            expected_www_asset_markers(native_ready.path()),
            vec!["index.html".to_string(), "manifest.json".to_string()]
        );
    }

    #[test]
    fn android_receipt_requires_native_entrypoint_for_app_router_output() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "app/index.html");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec!["app/index.html".to_string(), "index.html".to_string()]
        );
    }

    #[test]
    fn android_receipt_maps_legacy_www_output_to_asset_marker() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec!["index.html".to_string()]
        );
    }

    #[test]
    fn android_receipt_maps_app_router_www_output_to_asset_marker() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");
        write_www_output(project.path(), "app/index.html");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec!["app/index.html".to_string(), "index.html".to_string()]
        );
    }

    #[test]
    fn android_receipt_maps_source_route_www_output_to_asset_marker() {
        let project = tempfile::tempdir().expect("tempdir");
        write_www_output(project.path(), "index.html");
        write_www_output(project.path(), "source-routes/root/index.html");

        assert_eq!(
            expected_www_asset_markers(project.path()),
            vec![
                "index.html".to_string(),
                "source-routes/root/index.html".to_string(),
            ]
        );
    }

    fn write_www_output(project_root: &Path, relative_path: &str) {
        let path = project_root.join(".dx/www/output").join(relative_path);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("output dir");
        std::fs::write(path, "<!doctype html>").expect("output file");
    }
}
