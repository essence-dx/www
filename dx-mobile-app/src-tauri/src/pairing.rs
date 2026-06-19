use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use url::Url;

const REJECTS_BEARER_TOKENS: &str = "QR parser rejects bearer tokens";
const PAIRING_CHALLENGE_TTL_SECONDS: i64 = 300;
const PAIRING_CODE_MAX_LENGTH: usize = 128;
const SESSION_ID_MAX_LENGTH: usize = 128;
const SESSION_KEY_MAX_LENGTH: usize = 256;
const AGENT_ALIAS_MAX_LENGTH: usize = 64;
pub const FORBIDDEN_PAIRING_FIELD_NAMES: &[&str] = &[
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

#[derive(Debug, Deserialize)]
struct JsonPairingPayload {
    gateway: Option<String>,
    gateway_origin: Option<String>,
    origin: Option<String>,
    code: Option<String>,
    pairing_code: Option<String>,
    session_id: Option<String>,
    session_key: Option<String>,
    agent: Option<String>,
    agent_alias: Option<String>,
    expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PairingPayloadPreview {
    pub available: bool,
    pub gateway_origin: String,
    pub pairing_code: String,
    pub session_id: Option<String>,
    pub session_key: Option<String>,
    pub agent_alias: Option<String>,
    pub expires_at: Option<String>,
    pub token_safe: bool,
}

pub fn parse_pairing_payload(raw: &str) -> Result<PairingPayloadPreview, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("pairing payload is required".to_string());
    }
    if trimmed.len() > 4096 {
        return Err("pairing payload is too large".to_string());
    }
    reject_token_bearing_payload(trimmed)?;

    if trimmed.starts_with('{') {
        parse_json_pairing_payload(trimmed)
    } else {
        parse_url_pairing_payload(trimmed)
    }
}

fn parse_json_pairing_payload(raw: &str) -> Result<PairingPayloadPreview, String> {
    let raw_value: Value =
        serde_json::from_str(raw).map_err(|_| "pairing payload JSON is invalid".to_string())?;
    reject_forbidden_json_fields(&raw_value)?;
    let value: JsonPairingPayload = serde_json::from_value(raw_value)
        .map_err(|_| "pairing payload JSON is invalid".to_string())?;
    let gateway = select_json_alias(
        &[
            ("gateway", value.gateway.as_deref()),
            ("gateway_origin", value.gateway_origin.as_deref()),
            ("origin", value.origin.as_deref()),
        ],
        "gateway",
        true,
    )?
    .expect("required gateway alias");
    let code = select_json_alias(
        &[
            ("code", value.code.as_deref()),
            ("pairing_code", value.pairing_code.as_deref()),
        ],
        "code",
        true,
    )?
    .expect("required code alias");
    let agent_alias = select_json_alias(
        &[
            ("agent_alias", value.agent_alias.as_deref()),
            ("agent", value.agent.as_deref()),
        ],
        "agent",
        false,
    )?;

    preview(
        &gateway,
        &code,
        value.session_id,
        value.session_key,
        agent_alias,
        value.expires_at,
    )
}

fn parse_url_pairing_payload(raw: &str) -> Result<PairingPayloadPreview, String> {
    let url = Url::parse(raw).map_err(|_| "pairing payload URL is invalid".to_string())?;
    reject_token_bearing_url(&url)?;
    let gateway = select_query_alias(
        &url,
        &["gateway", "gateway_origin", "origin"],
        "gateway",
        false,
    )?
    .or_else(|| gateway_from_url_payload(&url))
    .ok_or_else(|| "pairing payload gateway is required".to_string())?;
    let code = select_query_alias(&url, &["code", "pairing_code"], "code", true)?
        .expect("required code alias");
    let session_id = select_query_alias(&url, &["session_id"], "session id", false)?;
    let session_key = select_query_alias(&url, &["session_key"], "session key", false)?;
    let agent_alias = select_query_alias(&url, &["agent_alias", "agent"], "agent", false)?;
    let expires_at = select_query_alias(&url, &["expires_at"], "expiry", false)?;

    preview(
        &gateway,
        &code,
        session_id,
        session_key,
        agent_alias,
        expires_at,
    )
}

fn select_json_alias(
    aliases: &[(&str, Option<&str>)],
    label: &str,
    required: bool,
) -> Result<Option<String>, String> {
    let candidates: Vec<String> = aliases
        .iter()
        .filter_map(|(_, value)| {
            value
                .map(str::trim)
                .filter(|candidate| !candidate.is_empty())
                .map(str::to_string)
        })
        .collect();

    if candidates.len() > 1 {
        return Err(format!("pairing payload {label} fields must not conflict"));
    }
    if required && candidates.is_empty() {
        return Err(format!("pairing payload {label} is required"));
    }
    Ok(candidates.into_iter().next())
}

fn select_query_alias(
    url: &Url,
    fields: &[&str],
    label: &str,
    required: bool,
) -> Result<Option<String>, String> {
    let mut candidates: Vec<(String, String)> = Vec::new();

    for (field, value) in url.query_pairs() {
        let field = field.as_ref();
        if !fields.iter().any(|candidate| *candidate == field) {
            continue;
        }

        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        if candidates
            .iter()
            .any(|(candidate_field, _)| candidate_field == field)
        {
            return Err(format!("pairing payload {label} fields must not repeat"));
        }
        candidates.push((field.to_string(), value.to_string()));
    }

    if candidates.len() > 1 {
        return Err(format!("pairing payload {label} fields must not conflict"));
    }
    if required && candidates.is_empty() {
        return Err(format!("pairing payload {label} is required"));
    }
    Ok(candidates.into_iter().next().map(|(_, value)| value))
}

fn preview(
    gateway: &str,
    code: &str,
    session_id: Option<String>,
    session_key: Option<String>,
    agent_alias: Option<String>,
    expires_at: Option<String>,
) -> Result<PairingPayloadPreview, String> {
    let pairing_code =
        normalize_required_identity_field(code, "pairing code", PAIRING_CODE_MAX_LENGTH)?;
    reject_expired_pairing_payload(expires_at.as_deref())?;

    Ok(PairingPayloadPreview {
        available: true,
        gateway_origin: validate_gateway_origin(gateway)?,
        pairing_code,
        session_id: normalize_optional_identity_field(
            session_id,
            "session id",
            SESSION_ID_MAX_LENGTH,
        )?,
        session_key: normalize_optional_identity_field(
            session_key,
            "session key",
            SESSION_KEY_MAX_LENGTH,
        )?,
        agent_alias: normalize_optional_identity_field(
            agent_alias,
            "agent alias",
            AGENT_ALIAS_MAX_LENGTH,
        )?,
        expires_at,
        token_safe: true,
    })
}

fn normalize_required_identity_field(
    value: &str,
    label: &str,
    max_length: usize,
) -> Result<String, String> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(format!("{label} is required"));
    }
    validate_identity_field(normalized, label, max_length)?;
    Ok(normalized.to_string())
}

fn normalize_optional_identity_field(
    value: Option<String>,
    label: &str,
    max_length: usize,
) -> Result<Option<String>, String> {
    let Some(normalized) = value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
    else {
        return Ok(None);
    };
    validate_identity_field(&normalized, label, max_length)?;
    Ok(Some(normalized))
}

fn validate_identity_field(value: &str, label: &str, max_length: usize) -> Result<(), String> {
    if value
        .chars()
        .any(|character| character.is_whitespace() || character.is_control())
    {
        return Err(format!(
            "{label} must not contain whitespace or control characters"
        ));
    }
    if value.chars().count() > max_length {
        return Err(format!("{label} is too long"));
    }
    Ok(())
}

fn gateway_from_url_payload(url: &Url) -> Option<String> {
    matches!(url.scheme(), "https" | "http").then(|| url.as_str().to_string())
}

fn reject_token_bearing_payload(raw: &str) -> Result<(), String> {
    let lower = raw.to_ascii_lowercase();
    if lower.contains("bearer ")
        || lower.contains("bearer.")
        || FORBIDDEN_PAIRING_FIELD_NAMES
            .iter()
            .any(|field| contains_forbidden_pairing_field_marker(&lower, field))
    {
        return Err(REJECTS_BEARER_TOKENS.to_string());
    }
    Ok(())
}

fn contains_forbidden_pairing_field_marker(lower: &str, field: &str) -> bool {
    if lower.contains(&format!("{field}=")) || lower.contains(&format!("\"{field}\"")) {
        return true;
    }

    let hyphenated = field.replace('_', "-");
    hyphenated != field
        && (lower.contains(&format!("{hyphenated}="))
            || lower.contains(&format!("\"{hyphenated}\"")))
}

fn reject_forbidden_json_fields(value: &Value) -> Result<(), String> {
    match value {
        Value::Object(map) => {
            for (key, nested) in map {
                if is_forbidden_pairing_field(key) {
                    return Err(REJECTS_BEARER_TOKENS.to_string());
                }
                reject_forbidden_json_fields(nested)?;
            }
        }
        Value::Array(items) => {
            for item in items {
                reject_forbidden_json_fields(item)?;
            }
        }
        Value::String(text) => reject_token_bearing_payload(text)?,
        _ => {}
    }

    Ok(())
}

fn reject_token_bearing_url(url: &Url) -> Result<(), String> {
    for (field, value) in url.query_pairs() {
        reject_decoded_pairing_entry(&field, &value)?;
    }
    Ok(())
}

fn reject_decoded_pairing_entry(field: &str, value: &str) -> Result<(), String> {
    if is_forbidden_pairing_field(field) {
        return Err(REJECTS_BEARER_TOKENS.to_string());
    }
    reject_token_bearing_payload(value)
}

fn is_forbidden_pairing_field(field: &str) -> bool {
    let normalized = field.trim().to_ascii_lowercase().replace(['-', ' '], "_");
    FORBIDDEN_PAIRING_FIELD_NAMES.contains(&normalized.as_str())
}

pub fn reject_expired_pairing_payload(expires_at: Option<&str>) -> Result<(), String> {
    let Some(expires_at) = expires_at.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(());
    };
    let expires = OffsetDateTime::parse(expires_at, &Rfc3339)
        .map_err(|_| "pairing payload expiry must be RFC3339".to_string())?;
    let now = OffsetDateTime::now_utc();
    if expires <= now {
        return Err("pairing payload is expired".to_string());
    }
    if expires - now > time::Duration::seconds(PAIRING_CHALLENGE_TTL_SECONDS) {
        return Err(format!(
            "pairing payload expiry exceeds {PAIRING_CHALLENGE_TTL_SECONDS} seconds"
        ));
    }
    Ok(())
}

pub fn validate_gateway_origin(origin: &str) -> Result<String, String> {
    let parsed = Url::parse(origin).map_err(|_| "gateway origin is invalid".to_string())?;
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("gateway origin must not include credentials".to_string());
    }
    match parsed.scheme() {
        "https" => {}
        "http" if is_loopback_host(parsed.host_str()) => {}
        _ => return Err("gateway origin must be HTTPS or loopback HTTP".to_string()),
    }

    let mut normalized = parsed;
    let normalized_path = normalized.path().trim_end_matches('/').to_string();
    normalized.set_path(if normalized_path.is_empty() {
        "/"
    } else {
        &normalized_path
    });
    normalized.set_query(None);
    normalized.set_fragment(None);
    Ok(normalized.as_str().trim_end_matches('/').to_string())
}

fn is_loopback_host(host: Option<&str>) -> bool {
    matches!(host, Some("127.0.0.1" | "localhost" | "::1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_rejects_token_bearing_json() {
        let payload = r#"{"gateway":"https://dx.local","code":"123456","token":"secret"}"#;

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            REJECTS_BEARER_TOKENS
        );
    }

    #[test]
    fn parser_rejects_unicode_escaped_token_json_key() {
        let payload = r#"{"gateway":"https://dx.local","code":"123456","tok\u0065n":"secret"}"#;

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            REJECTS_BEARER_TOKENS
        );
    }

    #[test]
    fn parser_rejects_decoded_token_query_key() {
        let payload = "dxmobile://pair?gateway=https://dx.local&code=123456&%74oken=secret";

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            REJECTS_BEARER_TOKENS
        );
    }

    #[test]
    fn parser_rejects_decoded_bearer_query_value() {
        let payload = "dxmobile://pair?gateway=https://dx.local&code=123456&auth=Bearer%20secret";

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            REJECTS_BEARER_TOKENS
        );
    }

    #[test]
    fn parser_rejects_credential_shaped_pairing_fields() {
        let fields = [
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
        ];

        for field in fields {
            let url_payload =
                format!("dxmobile://pair?gateway=https://dx.local&code=123456&{field}=secret");
            assert_eq!(
                parse_pairing_payload(&url_payload).unwrap_err(),
                REJECTS_BEARER_TOKENS,
                "URL field {field} should be rejected"
            );

            let json_payload =
                format!(r#"{{"gateway":"https://dx.local","code":"123456","{field}":"secret"}}"#);
            assert_eq!(
                parse_pairing_payload(&json_payload).unwrap_err(),
                REJECTS_BEARER_TOKENS,
                "JSON field {field} should be rejected"
            );
        }
    }

    #[test]
    fn parser_rejects_gateway_userinfo() {
        let payload = "dxmobile://pair?gateway=https://user:pass@dx.local&code=123456";

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            "gateway origin must not include credentials"
        );
    }

    #[test]
    fn parser_rejects_non_loopback_http_gateway() {
        let payload = "dxmobile://pair?gateway=http://192.168.1.20:42617&code=123456";

        assert_eq!(
            parse_pairing_payload(payload).unwrap_err(),
            "gateway origin must be HTTPS or loopback HTTP"
        );
    }

    #[test]
    fn parser_rejects_pairing_code_whitespace_and_control_characters() {
        let whitespace_payload = "dxmobile://pair?gateway=https://dx.local&code=123%20456";
        let control_payload = "dxmobile://pair?gateway=https://dx.local&code=123%0A456";

        assert_eq!(
            parse_pairing_payload(whitespace_payload).unwrap_err(),
            "pairing code must not contain whitespace or control characters"
        );
        assert_eq!(
            parse_pairing_payload(control_payload).unwrap_err(),
            "pairing code must not contain whitespace or control characters"
        );
    }

    #[test]
    fn parser_rejects_session_and_agent_identity_whitespace_and_control_characters() {
        let session_id_payload =
            r#"{"gateway":"https://dx.local","code":"123456","session_id":"session\n1"}"#;
        let session_key_payload =
            "dxmobile://pair?gateway=https://dx.local&code=123456&session_key=gw_%00hidden";
        let agent_payload =
            "dxmobile://pair?gateway=https://dx.local&code=123456&agent_alias=fri%20day";

        assert_eq!(
            parse_pairing_payload(session_id_payload).unwrap_err(),
            "session id must not contain whitespace or control characters"
        );
        assert_eq!(
            parse_pairing_payload(session_key_payload).unwrap_err(),
            "session key must not contain whitespace or control characters"
        );
        assert_eq!(
            parse_pairing_payload(agent_payload).unwrap_err(),
            "agent alias must not contain whitespace or control characters"
        );
    }

    #[test]
    fn parser_rejects_overlong_pairing_identity_fields() {
        let code = "1".repeat(129);
        let session_id = "s".repeat(129);
        let agent_alias = "f".repeat(65);
        let code_payload = format!("dxmobile://pair?gateway=https://dx.local&code={code}");
        let session_payload =
            format!("dxmobile://pair?gateway=https://dx.local&code=123456&session_id={session_id}");
        let agent_payload = format!(
            "dxmobile://pair?gateway=https://dx.local&code=123456&agent_alias={agent_alias}"
        );

        assert_eq!(
            parse_pairing_payload(&code_payload).unwrap_err(),
            "pairing code is too long"
        );
        assert_eq!(
            parse_pairing_payload(&session_payload).unwrap_err(),
            "session id is too long"
        );
        assert_eq!(
            parse_pairing_payload(&agent_payload).unwrap_err(),
            "agent alias is too long"
        );
    }

    #[test]
    fn parser_rejects_conflicting_json_pairing_aliases() {
        let gateway_payload =
            r#"{"gateway":"https://dx.local","origin":"https://other.local","code":"123456"}"#;
        let code_payload =
            r#"{"gateway":"https://dx.local","code":"123456","pairing_code":"654321"}"#;
        let agent_payload = r#"{"gateway":"https://dx.local","code":"123456","agent":"friday","agent_alias":"zed"}"#;

        assert_eq!(
            parse_pairing_payload(gateway_payload).unwrap_err(),
            "pairing payload gateway fields must not conflict"
        );
        assert_eq!(
            parse_pairing_payload(code_payload).unwrap_err(),
            "pairing payload code fields must not conflict"
        );
        assert_eq!(
            parse_pairing_payload(agent_payload).unwrap_err(),
            "pairing payload agent fields must not conflict"
        );
    }

    #[test]
    fn parser_rejects_conflicting_url_pairing_aliases() {
        let gateway_payload =
            "dxmobile://pair?gateway=https://dx.local&gateway_origin=https://other.local&code=123456";
        let code_payload =
            "dxmobile://pair?gateway=https://dx.local&code=123456&pairing_code=654321";
        let agent_payload =
            "dxmobile://pair?gateway=https://dx.local&code=123456&agent=friday&agent_alias=zed";

        assert_eq!(
            parse_pairing_payload(gateway_payload).unwrap_err(),
            "pairing payload gateway fields must not conflict"
        );
        assert_eq!(
            parse_pairing_payload(code_payload).unwrap_err(),
            "pairing payload code fields must not conflict"
        );
        assert_eq!(
            parse_pairing_payload(agent_payload).unwrap_err(),
            "pairing payload agent fields must not conflict"
        );
    }

    #[test]
    fn parser_rejects_repeated_url_pairing_fields() {
        let fields = [
            ("code=123456&code=654321", "code"),
            (
                "code=123456&session_id=session-1&session_id=session-2",
                "session id",
            ),
            (
                "code=123456&session_key=gw_session&session_key=other",
                "session key",
            ),
            (
                "code=123456&expires_at=2026-01-01T00:00:00Z&expires_at=2026-01-01T00:01:00Z",
                "expiry",
            ),
        ];

        for (query, label) in fields {
            let payload = format!("dxmobile://pair?gateway=https://dx.local&{query}");
            assert_eq!(
                parse_pairing_payload(&payload).unwrap_err(),
                format!("pairing payload {label} fields must not repeat"),
                "{label} should reject repeated query fields"
            );
        }
    }

    #[test]
    fn parser_accepts_loopback_gateway() {
        let payload =
            "dxmobile://pair?gateway=http://127.0.0.1:42617/path&code=123456&agent=friday";
        let preview = parse_pairing_payload(payload).expect("loopback payload");

        assert_eq!(preview.gateway_origin, "http://127.0.0.1:42617/path");
        assert_eq!(preview.agent_alias.as_deref(), Some("friday"));
    }

    #[test]
    fn parser_accepts_gateway_origin_query_alias() {
        let payload = "dxmobile://pair?gateway_origin=https://desktop.example/gw&code=123456";
        let preview = parse_pairing_payload(payload).expect("gateway_origin alias");

        assert_eq!(preview.gateway_origin, "https://desktop.example/gw");
        assert_eq!(preview.pairing_code, "123456");
    }

    #[test]
    fn parser_preserves_https_gateway_path_prefix() {
        let payload =
            r#"{"gateway":"https://desktop.example/gw/?ignored=true#fragment","code":"123456"}"#;
        let preview = parse_pairing_payload(payload).expect("path-prefixed gateway");

        assert_eq!(preview.gateway_origin, "https://desktop.example/gw");
    }

    #[test]
    fn parser_accepts_https_gateway_url_with_path_prefix() {
        let payload = "https://desktop.example/gw/?code=123456&session_id=session-1&agent=friday";
        let preview = parse_pairing_payload(payload).expect("https gateway url");

        assert_eq!(preview.gateway_origin, "https://desktop.example/gw");
        assert_eq!(preview.pairing_code, "123456");
        assert_eq!(preview.session_id.as_deref(), Some("session-1"));
        assert_eq!(preview.agent_alias.as_deref(), Some("friday"));
    }

    #[test]
    fn parser_accepts_loopback_gateway_url_with_path_prefix() {
        let payload =
            "http://127.0.0.1:42617/zeroclaw/?pairing_code=654321&session_key=gw_session&agent_alias=friday";
        let preview = parse_pairing_payload(payload).expect("loopback gateway url");

        assert_eq!(preview.gateway_origin, "http://127.0.0.1:42617/zeroclaw");
        assert_eq!(preview.pairing_code, "654321");
        assert_eq!(preview.session_key.as_deref(), Some("gw_session"));
        assert_eq!(preview.agent_alias.as_deref(), Some("friday"));
    }

    #[test]
    fn parser_rejects_pairing_expiry_beyond_challenge_ttl() {
        let expires_at = (OffsetDateTime::now_utc() + time::Duration::minutes(10))
            .format(&Rfc3339)
            .expect("expiry");
        let payload = format!("https://desktop.example/gw/?code=123456&expires_at={expires_at}");

        assert_eq!(
            parse_pairing_payload(&payload).unwrap_err(),
            "pairing payload expiry exceeds 300 seconds"
        );
    }
}
