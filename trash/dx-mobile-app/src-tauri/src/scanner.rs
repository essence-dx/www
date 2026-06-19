use serde::Serialize;

const SCANNER_UNAVAILABLE: &str = "camera scanner adapter is not configured";

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ScannerStatus {
    pub available: bool,
    pub permission_required: bool,
    pub adapter: &'static str,
    pub reason: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ScanQrResult {
    pub available: bool,
    pub permission_required: bool,
    pub raw_payload: Option<String>,
    pub reason: &'static str,
}

pub fn scanner_status() -> ScannerStatus {
    ScannerStatus {
        available: false,
        permission_required: false,
        adapter: "native-camera-scanner-unavailable",
        reason: SCANNER_UNAVAILABLE,
    }
}

pub fn scan_pairing_qr() -> ScanQrResult {
    ScanQrResult {
        available: false,
        permission_required: false,
        raw_payload: None,
        reason: SCANNER_UNAVAILABLE,
    }
}
