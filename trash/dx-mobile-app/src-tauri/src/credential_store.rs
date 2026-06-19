use serde::{Deserialize, Serialize};

use crate::pairing::validate_gateway_origin;

const SECURE_STORAGE_UNAVAILABLE: &str = "native secure storage adapter is not configured";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceCredential {
    pub gateway_origin: String,
    pub token: String,
    pub device_id: String,
    pub paired_at: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SecureCredentialStoreStatus {
    pub available: bool,
    pub durable: bool,
    pub adapter: &'static str,
    pub token_persistence: &'static str,
    pub reason: &'static str,
}

#[derive(Debug, Serialize)]
pub struct StoreCredentialResult {
    pub stored: bool,
    pub durable: bool,
    pub reason: &'static str,
}

#[derive(Debug, Serialize)]
pub struct LoadCredentialResult {
    pub credential: Option<DeviceCredential>,
    pub durable: bool,
    pub reason: &'static str,
}

pub fn credential_store_status() -> SecureCredentialStoreStatus {
    SecureCredentialStoreStatus {
        available: false,
        durable: false,
        adapter: "native-secure-store-unavailable",
        token_persistence: "disabled-no-plaintext-fallback",
        reason: SECURE_STORAGE_UNAVAILABLE,
    }
}

pub fn store_device_credential(
    credential: DeviceCredential,
) -> Result<StoreCredentialResult, String> {
    let _credential = validate_device_credential(credential)?;

    Ok(StoreCredentialResult {
        stored: false,
        durable: false,
        reason: SECURE_STORAGE_UNAVAILABLE,
    })
}

pub fn load_device_credential() -> LoadCredentialResult {
    LoadCredentialResult {
        credential: None,
        durable: false,
        reason: SECURE_STORAGE_UNAVAILABLE,
    }
}

pub fn clear_device_credential() -> StoreCredentialResult {
    StoreCredentialResult {
        stored: false,
        durable: false,
        reason: SECURE_STORAGE_UNAVAILABLE,
    }
}

fn validate_device_credential(
    mut credential: DeviceCredential,
) -> Result<DeviceCredential, String> {
    credential.gateway_origin = validate_gateway_origin(&credential.gateway_origin)?;
    credential.token = normalize_gateway_bearer_token(&credential.token)?;
    credential.device_id = credential.device_id.trim().to_string();
    if credential.device_id.is_empty() {
        return Err("device id is required".to_string());
    }
    Ok(credential)
}

fn normalize_gateway_bearer_token(token: &str) -> Result<String, String> {
    let mut normalized = token.trim();
    if normalized
        .get(.."Bearer ".len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Bearer "))
    {
        normalized = normalized.get("Bearer ".len()..).unwrap_or("").trim();
    }
    if normalized.is_empty() {
        return Err("device credential token is required".to_string());
    }
    if normalized
        .chars()
        .any(|character| character.is_whitespace() || character.is_control())
        || normalized
            .get(.."Bearer ".len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Bearer "))
    {
        return Err(
            "device credential token must not contain whitespace or control characters".to_string(),
        );
    }
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn credential() -> DeviceCredential {
        DeviceCredential {
            gateway_origin: "http://127.0.0.1:42617".to_string(),
            token: "paired-token".to_string(),
            device_id: "dx-mobile".to_string(),
            paired_at: "2026-06-11T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn store_rejects_empty_tokens() {
        let mut credential = credential();
        credential.token.clear();

        assert_eq!(
            store_device_credential(credential).unwrap_err(),
            "device credential token is required"
        );
    }

    #[test]
    fn credential_validation_trims_edge_token_whitespace() {
        let mut credential = credential();
        credential.token = " \tpaired-token\r\n".to_string();

        let credential = validate_device_credential(credential).expect("valid credential");

        assert_eq!(credential.token, "paired-token");
    }

    #[test]
    fn credential_validation_strips_single_bearer_scheme() {
        let mut credential = credential();
        credential.token = " \tBearer paired-token\r\n".to_string();

        let credential = validate_device_credential(credential).expect("valid credential");

        assert_eq!(credential.token, "paired-token");
    }

    #[test]
    fn credential_validation_rejects_token_whitespace_and_control_characters() {
        for token in [
            "paired token",
            "paired\ttoken",
            "paired\ntoken",
            "paired\rtoken",
            "Bearer Bearer paired-token",
            "paired\u{7f}token",
        ] {
            let mut credential = credential();
            credential.token = token.to_string();

            assert_eq!(
                validate_device_credential(credential).unwrap_err(),
                "device credential token must not contain whitespace or control characters"
            );
        }
    }

    #[test]
    fn store_never_falls_back_to_plaintext() {
        let result = store_device_credential(credential()).expect("valid credential");

        assert!(!result.stored);
        assert!(!result.durable);
        assert_eq!(
            credential_store_status().token_persistence,
            "disabled-no-plaintext-fallback"
        );
    }
}
