use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::env_error;
use crate::error::DxResult;

pub(crate) const ENV_SCHEMA: &str = "dx.env.local.sealed.v1";
pub(crate) const ENV_RECEIPT_SCHEMA: &str = "dx.env.check.receipt.v1";
pub(crate) const ENV_AGENT_CONTEXT_SCHEMA: &str = "dx.env.agent_context.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum EnvScope {
    Public,
    Server,
}

impl EnvScope {
    pub(crate) fn infer(name: &str) -> Self {
        if name.starts_with("PUBLIC_")
            || name.starts_with("NEXT_PUBLIC_")
            || name.starts_with("DX_PUBLIC_")
        {
            Self::Public
        } else {
            Self::Server
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Server => "server",
        }
    }
}

impl fmt::Display for EnvScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EnvRecord {
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) scope: EnvScope,
    pub(crate) capability: Option<String>,
}

impl EnvRecord {
    pub(crate) fn parse_many(source: &str) -> DxResult<Vec<Self>> {
        let mut records = Vec::new();
        for (line_index, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if trimmed.eq_ignore_ascii_case("unlock") {
                continue;
            }
            let Some((name, value)) = trimmed.split_once('=') else {
                return Err(env_error(format!(
                    "Invalid .env line {}: expected NAME=value",
                    line_index + 1
                )));
            };
            let name = name.trim();
            validate_env_name(name, line_index + 1)?;
            let value = parse_env_value(value.trim());
            let scope = EnvScope::infer(name);
            let capability = infer_capability(name, scope);
            records.push(Self {
                name: name.to_string(),
                value,
                scope,
                capability,
            });
        }
        records.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(records)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvPlaintext {
    pub(crate) schema: String,
    pub(crate) records: Vec<EnvRecord>,
}

#[derive(Debug, Clone)]
pub(crate) struct EnvOpenOptions {
    pub(crate) password: String,
    pub(crate) ttl: Duration,
    pub(crate) json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvLockReport {
    pub(crate) schema: String,
    pub(crate) status: String,
    pub(crate) key_count: usize,
    pub(crate) store_path: String,
    pub(crate) machine_path: String,
    pub(crate) env_path: String,
    pub(crate) values_redacted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvOpenReport {
    pub(crate) schema: String,
    pub(crate) status: String,
    pub(crate) key_count: usize,
    pub(crate) expires_at_unix: u64,
    pub(crate) env_path: String,
    pub(crate) values_visible_in_viewport: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvViewReconcileReport {
    pub(crate) schema: String,
    pub(crate) status: String,
    pub(crate) resealed: bool,
    pub(crate) env_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnvCheckFormat {
    Terminal,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum EnvCheckStatus {
    Current,
    Missing,
    UnsealedViewport,
    MachineMissing,
}

impl EnvCheckStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Missing => "missing",
            Self::UnsealedViewport => "unsealed-viewport",
            Self::MachineMissing => "machine-missing",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvCheckKey {
    pub(crate) name: String,
    pub(crate) scope: EnvScope,
    pub(crate) capability: Option<String>,
    pub(crate) value_redacted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvCheckReport {
    pub(crate) schema: String,
    pub(crate) status: EnvCheckStatus,
    pub(crate) keys: Vec<EnvCheckKey>,
    pub(crate) store_path: String,
    pub(crate) machine_path: String,
    pub(crate) env_path: String,
    pub(crate) typed_contract_path: String,
    pub(crate) values_redacted: bool,
    pub(crate) receipt_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EnvAgentContext {
    pub(crate) schema: String,
    pub(crate) status: EnvCheckStatus,
    pub(crate) keys: Vec<EnvCheckKey>,
    pub(crate) safe_for_agents: bool,
    pub(crate) values_available_to_agent: bool,
}

fn validate_env_name(name: &str, line: usize) -> DxResult<()> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(env_error(format!(
            "Invalid .env line {line}: empty env name"
        )));
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return Err(env_error(format!(
            "Invalid .env line {line}: env name `{name}` must start with a letter or underscore"
        )));
    }
    if !chars.all(|char| char == '_' || char.is_ascii_alphanumeric()) {
        return Err(env_error(format!(
            "Invalid .env line {line}: env name `{name}` may only contain letters, numbers, and underscores"
        )));
    }
    Ok(())
}

fn parse_env_value(value: &str) -> String {
    let quoted = (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''));
    if quoted && value.len() >= 2 {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

fn infer_capability(name: &str, scope: EnvScope) -> Option<String> {
    if scope == EnvScope::Public {
        return Some("browser/public".to_string());
    }
    let upper = name.to_ascii_uppercase();
    let capability = if upper.contains("DATABASE") || upper.ends_with("_DB_URL") {
        "server/database"
    } else if upper.contains("STRIPE") || upper.contains("PADDLE") || upper.contains("LEMON") {
        "server/payment"
    } else if upper.contains("RESEND") || upper.contains("SENDGRID") || upper.contains("MAIL") {
        "server/email"
    } else if upper.contains("AUTH")
        || upper.contains("CLERK")
        || upper.contains("SESSION")
        || upper.contains("OAUTH")
    {
        "server/auth"
    } else if upper.contains("WEBHOOK") {
        "server/webhook"
    } else {
        "server/runtime"
    };
    Some(capability.to_string())
}
