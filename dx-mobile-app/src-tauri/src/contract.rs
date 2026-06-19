use serde::Serialize;

use crate::credential_store::{credential_store_status, SecureCredentialStoreStatus};
use crate::pairing::FORBIDDEN_PAIRING_FIELD_NAMES;
use crate::scanner::{scanner_status, ScannerStatus};

pub const BRIDGE_NAME: &str = "dx-native-mobile-companion";

#[derive(Debug, Serialize)]
pub struct GatewayOriginPolicy {
    pub https_required_for_remote: bool,
    pub http_loopback_only: bool,
    pub forbid_credentials: bool,
}

#[derive(Debug, Serialize)]
pub struct NativeCompanionContract {
    pub app: &'static str,
    pub www_route: &'static str,
    pub native_bridge: &'static str,
    pub pairing_contract: &'static str,
    pub chat_contract: &'static str,
    pub credential_storage: &'static str,
    pub credential_store_status_command: &'static str,
    pub gateway_origin_policy: GatewayOriginPolicy,
    pub pairing_forbidden_fields: &'static [&'static str],
    pub qr_scanner_command: &'static str,
    pub credential_store: SecureCredentialStoreStatus,
    pub scanner: ScannerStatus,
}

pub fn native_companion_contract() -> NativeCompanionContract {
    NativeCompanionContract {
        app: "dx-mobile-app",
        www_route: "/",
        native_bridge: BRIDGE_NAME,
        pairing_contract: "dx.mobile.pairing",
        chat_contract: "dx.mobile.chat_handoff",
        credential_storage: "adapter-required",
        credential_store_status_command: "dx_mobile_credential_store_status",
        gateway_origin_policy: GatewayOriginPolicy {
            https_required_for_remote: true,
            http_loopback_only: true,
            forbid_credentials: true,
        },
        pairing_forbidden_fields: FORBIDDEN_PAIRING_FIELD_NAMES,
        qr_scanner_command: "dx_mobile_scan_pairing_qr",
        credential_store: credential_store_status(),
        scanner: scanner_status(),
    }
}
