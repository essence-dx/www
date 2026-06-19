use argon2::{Algorithm, Argon2, Params, Version};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use super::env_error;
use crate::error::DxResult;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
const KEY_LEN: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct EnvEncryptedPayload {
    pub(super) kdf: String,
    pub(super) cipher: String,
    pub(super) salt: String,
    pub(super) nonce: String,
    pub(super) ciphertext: String,
}

pub(super) fn encrypt_payload(plaintext: &[u8], password: &str) -> DxResult<EnvEncryptedPayload> {
    if password.is_empty() {
        return Err(env_error("dx env password cannot be empty"));
    }

    let mut salt = [0_u8; SALT_LEN];
    let mut nonce = [0_u8; NONCE_LEN];
    getrandom::getrandom(&mut salt)
        .map_err(|error| env_error(format!("Failed to generate env salt: {error}")))?;
    getrandom::getrandom(&mut nonce)
        .map_err(|error| env_error(format!("Failed to generate env nonce: {error}")))?;

    let mut key = derive_key(password, &salt)?;
    let cipher = XChaCha20Poly1305::new((&key).into());
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext)
        .map_err(|error| env_error(format!("Failed to encrypt env store: {error}")))?;
    key.zeroize();

    Ok(EnvEncryptedPayload {
        kdf: "argon2id".to_string(),
        cipher: "xchacha20poly1305".to_string(),
        salt: STANDARD_NO_PAD.encode(salt),
        nonce: STANDARD_NO_PAD.encode(nonce),
        ciphertext: STANDARD_NO_PAD.encode(ciphertext),
    })
}

pub(super) fn decrypt_payload(payload: &EnvEncryptedPayload, password: &str) -> DxResult<Vec<u8>> {
    if payload.kdf != "argon2id" {
        return Err(env_error(format!("Unsupported env KDF `{}`", payload.kdf)));
    }
    if payload.cipher != "xchacha20poly1305" {
        return Err(env_error(format!(
            "Unsupported env cipher `{}`",
            payload.cipher
        )));
    }
    let salt = decode_fixed::<SALT_LEN>("salt", &payload.salt)?;
    let nonce = decode_fixed::<NONCE_LEN>("nonce", &payload.nonce)?;
    let ciphertext = STANDARD_NO_PAD
        .decode(payload.ciphertext.as_bytes())
        .map_err(|error| env_error(format!("Invalid env ciphertext encoding: {error}")))?;

    let mut key = derive_key(password, &salt)?;
    let cipher = XChaCha20Poly1305::new((&key).into());
    let plaintext = cipher
        .decrypt(XNonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| env_error("Env password is wrong or the sealed store was modified"))?;
    key.zeroize();
    Ok(plaintext)
}

fn derive_key(password: &str, salt: &[u8]) -> DxResult<[u8; KEY_LEN]> {
    let params = Params::new(19_456, 2, 1, Some(KEY_LEN))
        .map_err(|error| env_error(format!("Invalid env KDF parameters: {error}")))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0_u8; KEY_LEN];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|error| env_error(format!("Failed to derive env key: {error}")))?;
    Ok(key)
}

fn decode_fixed<const LEN: usize>(label: &str, encoded: &str) -> DxResult<[u8; LEN]> {
    let bytes = STANDARD_NO_PAD
        .decode(encoded.as_bytes())
        .map_err(|error| env_error(format!("Invalid env {label} encoding: {error}")))?;
    bytes
        .try_into()
        .map_err(|_| env_error(format!("Invalid env {label} length")))
}
