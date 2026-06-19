mod contract;
mod credential_store;
mod pairing;
mod scanner;

use contract::{native_companion_contract, NativeCompanionContract};
use credential_store::{
    clear_device_credential, credential_store_status, load_device_credential,
    store_device_credential, DeviceCredential, LoadCredentialResult, SecureCredentialStoreStatus,
    StoreCredentialResult,
};
use pairing::{parse_pairing_payload, PairingPayloadPreview};
use scanner::{scan_pairing_qr, ScanQrResult};

#[tauri::command]
fn dx_mobile_companion_contract() -> NativeCompanionContract {
    native_companion_contract()
}

#[tauri::command]
fn dx_mobile_credential_store_status() -> SecureCredentialStoreStatus {
    credential_store_status()
}

#[tauri::command]
fn dx_mobile_parse_pairing_payload(raw: String) -> Result<PairingPayloadPreview, String> {
    parse_pairing_payload(&raw)
}

#[tauri::command]
fn dx_mobile_store_device_credential(
    credential: DeviceCredential,
) -> Result<StoreCredentialResult, String> {
    store_device_credential(credential)
}

#[tauri::command]
fn dx_mobile_load_device_credential() -> LoadCredentialResult {
    load_device_credential()
}

#[tauri::command]
fn dx_mobile_clear_device_credential() -> StoreCredentialResult {
    clear_device_credential()
}

#[tauri::command]
fn dx_mobile_scan_pairing_qr() -> ScanQrResult {
    scan_pairing_qr()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();
    #[cfg(mobile)]
    let builder = builder.setup(|app| {
        app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
        Ok(())
    });

    builder
        .invoke_handler(tauri::generate_handler![
            dx_mobile_companion_contract,
            dx_mobile_credential_store_status,
            dx_mobile_parse_pairing_payload,
            dx_mobile_store_device_credential,
            dx_mobile_load_device_credential,
            dx_mobile_clear_device_credential,
            dx_mobile_scan_pairing_qr
        ])
        .run(tauri::generate_context!())
        .expect("failed to run DX Mobile App native shell");
}
